use cortex_m::asm;
use max32625::{ADC as HADC, CLKMAN};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum AdcChannel {
    AIN0 = 0,
    AIN1 = 1,
    AIN2 = 2,
    AIN3 = 3,
    AIN0HV = 4,
    AIN1HV = 5,
    VDDB = 6,
    VDD18 = 7,
    VDD12 = 8,
    VRTC = 9,
    VDDIO = 11,
    VDDIOH = 12,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum AdcAlignment {
    LSBAligned = 0,
    MSBAligned = 1,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AdcSettings {
    channel: AdcChannel,
    data_scaling: bool,
    buffer_enabled: bool,
    data_alignment: AdcAlignment,
}

impl AdcSettings {
    pub const fn new(channel: AdcChannel) -> Self {
        Self {
            channel,
            data_scaling: false,
            buffer_enabled: false,
            data_alignment: AdcAlignment::LSBAligned,
        }
    }

    pub const fn data_scaling(&self) -> bool {
        self.data_scaling
    }

    pub fn set_data_scaling(&mut self, data_scaling: bool) {
        self.data_scaling = data_scaling;
    }

    pub const fn buffer_enabled(&self) -> bool {
        self.buffer_enabled
    }

    pub fn set_buffer_enabled(&mut self, buffer_enabled: bool) {
        self.buffer_enabled = buffer_enabled;
    }

    pub const fn data_alignment(&self) -> &AdcAlignment {
        &self.data_alignment
    }

    pub fn set_data_alignment(&mut self, data_alignment: AdcAlignment) {
        self.data_alignment = data_alignment;
    }

    pub const fn channel(&self) -> AdcChannel {
        self.channel
    }

    pub fn set_channel(&mut self, channel: AdcChannel) {
        self.channel = channel;
    }
}

#[derive(Debug)]
pub struct Adc {
    inner: HADC,
}

impl Adc {
    pub fn init(adc: HADC, clkman: &CLKMAN) -> Self {
        /*
        1 If not already enabled, enable dynamic clock gating to ADC - CLKMAN_CLK_GATE_CTRL2.adc_clk_gater = 1
        2 Enable ADC clock - CLKMAN_CLK_CTRL.adc_clock_enable = 1
        3 Enable ADC clock interface clock - ADC_CTRL.adc_clk_en = 1
        4 Clear AFE ready interrupt - ADC_INTR.adc_ref_ready_if = 1
        5 Enable AFE ready interrupt - ADC_INTR.adc_ref_ready_ie = 1
        6 Enable Power
            – ADC_CTRL.adc_pu = 1
            – ADC_CTRL.buf_pu = 0 (default); set to 1 to enable ADC input buffer
            – ADC_CTRL.adc_refbuf_pu = 1
            – ADC_CTRL.adc_chgpump_pu = 1
            – NOTE: ADC_CTRL.buf_bypass != ADC_CTRL.buf_pu
        7 Wait for AFE ready interrupt (interrupt flag ADC_INTR.adc_ref_ready_if)
        8 Clear AFE ready interrupt by writing 1 to ADC_INTR.adc_ref_ready_if
        9 Disable AFE ready interrupt by setting ADC_INTR.adc_ref_ready_ie = 0
        */
        // 1.
        clkman
            .clk_gate_ctrl2
            .modify(|_, w| w.adc_clk_gater().variant(1));
        // 2.
        clkman
            .clk_ctrl
            .modify(|_, w| w.adc_clock_enable().set_bit());
        // 3.
        adc.ctrl.modify(|_, w| w.adc_clk_en().set_bit());
        // 4/5.
        adc.intr.write(|w| {
            w.adc_ref_ready_if()
                .clear_bit_by_one()
                .adc_ref_ready_ie()
                .clear_bit()
        });
        // 6.
        adc.ctrl.modify(|_, w| {
            w.adc_pu()
                .set_bit()
                .buf_pu()
                .set_bit()
                .adc_refbuf_pu()
                .set_bit()
                .adc_chgpump_pu()
                .set_bit()
        });
        // 7
        while adc.intr.read().adc_ref_ready_if().bit_is_clear() {
            asm::nop()
        }
        // 8
        adc.intr.write(|w| w.adc_done_if().clear_bit_by_one());
        // 9 is unnecessary, since we disabled it earlier
        // Return
        Self { inner: adc }
    }

    pub fn get_value(&self, settings: AdcSettings) -> u16 {
        /*
        1 Configure the ADC for:
            – Channel selection - ADC_CTRL.adc_chsel
            – Data scaling (AIN0-AIN3 only) - ADC_CTRL.adc_scale
            – Buffer enable/disable - ADC_CTRL.buf_pu
            – Data alignment - ADC_CTRL.adc_dataalign
            – Limits: enable, channel, Hi/Lo limits - See ADC Data Limits below
        2 Clear the ADC interrupt done flag - ADC_INTR.adc_done_if = 1
        3 Enable the ADC interrupt done - ADC_INTR.adc_done_ie = 1
        4 Start the ADC - ADC_CTRL.cpu_adc_start = 1
        5 Wait for ADC done interrupt
        6 Read the result from ADC_DATA register (Note: repeat Channel Selection step to sample additional channels)
        */
        // 1
        self.inner.ctrl.modify(|_, w| {
            w.adc_chsel()
                .variant(settings.channel as u8)
                .adc_scale()
                .variant(settings.data_scaling || settings.channel > AdcChannel::AIN3)
                .buf_bypass()
                .variant(settings.buffer_enabled)
                .adc_dataalign()
                .variant(settings.data_alignment as u8 == 1)
                // Stuff from ADC source code
                .adc_refscl()
                .variant(
                    settings.channel != AdcChannel::VDD12 && settings.channel != AdcChannel::VDD18,
                )
        });
        // 2/3
        self.inner
            .intr
            .modify(|_, w| w.adc_done_if().clear_bit_by_one());
        // 4
        self.inner.ctrl.modify(|_, w| w.cpu_adc_start().set_bit());
        // 5
        while self.inner.intr.read().adc_done_if().bit_is_clear() {
            asm::nop()
        }
        // 6
        self.inner.data.read().adc_data().bits()
    }
}
