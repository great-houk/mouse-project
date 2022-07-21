use core::marker::PhantomData;

use max32625::{CLKMAN, IOMAN, SPIM0, SPIM1, SPIM2};
use max32625_gpio::{PinInMode, PinNum, Port, GPIO};

use crate::trans::*;

pub trait SPIM {}
impl SPIM for SPIM0 {}
impl SPIM for SPIM1 {}
impl SPIM for SPIM2 {}

#[derive(Debug)]
pub struct Spim<SPI: SPIM> {
    pub(crate) spi_raw: SPI,
    slaves: [Option<SpiSlave<SPI>>; 5],
}

impl<SPI: SPIM> Spim<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self {
            spi_raw: spi,
            slaves: Default::default(),
        }
    }

    fn take_pin(gpio: &mut GPIO, port: Port, pin: PinNum) -> Result<(), SpiError> {
        let mut pin = gpio
            .take(port, pin)
            .map_err(|_| SpiError::PinAlreadyTaken(port, pin))?;
        pin.set_in_mode(PinInMode::Normal);
        Ok(())
    }
}
impl_spims! { Spim<> SPI : mod _spim
    pub fn init(self, gpio: &mut GPIO, ioman: &IOMAN, scale: SpiClockScale, clkman: &CLKMAN) -> Result<Self, SpiError> {
        // Enable clock
        Self::init_clk(scale, clkman);
        // Enable/Take GPIO Pins
        Self::take_gpio(gpio)?;
        // Request SPIM function
        Self::request_spim_core_function(ioman);
        // Enable SPI module and fifos
        // Modify so sclk fb stays enabled
        const SPI_ENABLE: u32 =
            (1 << 0 /* Master Enable */) |
            (1 << 1 /* Tx Enable */) |
            (1 << 2 /* Rx Enable */) |
            (1 << 24 /* SCK FB Enable */);
        self.spi_raw.gen_ctrl.write(|w| unsafe { w.bits(0) });
        self.spi_raw.gen_ctrl.write(|w| unsafe { w.bits(SPI_ENABLE) });
        Ok(self)
    }

    pub fn add_slave(&mut self, gpio: &mut GPIO, ioman: &IOMAN, ss: SS, ss_polarity: Polarity, sr_polarity: Polarity) -> Result<&SpiSlave<SPI>, SpiError> {
        // Check for valid ss
        if let Some(_) = self.slaves[ss as usize] {
            return Err(SpiError::SlaveAlreadyExists);
        }
        // Take pin
        Self::take_slave_gpio(ss, gpio)?;
        // Request Function
        Self::request_slave_function(ss, ioman)?;
        // Set polarities
        self.set_ss_polarity(ss, ss_polarity);
        self.set_sr_polarity(ss, sr_polarity);
        // Make slave
        let slave = SpiSlave { ss, three_wire: false, spi_mode: SpiMode::Mode0, page_size: SpiPageSize::Bytes4, phantom: PhantomData };
        self.slaves[ss as usize] = Some(slave);
        // Return it
        Ok(self.slaves[ss as usize].as_ref().unwrap())
    }

    pub fn get_slave(&self, ss: SS) -> Result<&SpiSlave<SPI>, SpiError> {
        match self.slaves[ss as usize] {
            Some(ref slave) => Ok(slave),
            None => Err(SpiError::InvalidSS),
        }
    }

    pub fn get_slave_mut(&mut self, ss: SS) -> Result<&mut SpiSlave<SPI>, SpiError> {
        match self.slaves[ss as usize] {
            Some(ref mut slave) => Ok(slave),
            None => Err(SpiError::InvalidSS),
        }
    }

    pub fn transaction(&self, ss: SS, trans: impl for<'a> FnOnce(SpiTransaction<'a, SPI>)) -> Result<(), SpiError> {
        // Have slave send mstr cfg header
        let slave = self.get_slave(ss)?;
        slave.push_start(&self);
        // Make transaction
        let spi_trans = SpiTransaction { spim: self, slave: slave, end: false };
        // Call closure
        trans(spi_trans);
        // Have slave end transaction
        slave.push_end(&self);
        Ok(())
    }

    pub unsafe fn unbounded_transaction<'a>(&'a self, ss: SS) -> Result<SpiTransaction<'a, SPI>, SpiError> {
        // Have slave send mstr cfg header
        let slave = self.get_slave(ss)?;
        slave.push_start(&self);
        // Make transaction
        let spi_trans = SpiTransaction { spim: self, slave: slave, end: false };
        Ok(spi_trans)
    }

    fn set_ss_polarity(&mut self, ss: SS, polarity: Polarity) {
        let value = self.spi_raw.ss_sr_polarity.read().ss_polarity().bits();
        let mask = !(1 << ss as u8);
        let new_value = (value & mask) | ((polarity as u8) << (ss as u8));
        self.spi_raw.ss_sr_polarity.modify(|_, w| w.ss_polarity().variant(new_value));
    }

    fn set_sr_polarity(&mut self, ss: SS, polarity: Polarity) {
        let value = self.spi_raw.ss_sr_polarity.read().fc_polarity().bits();
        let mask = !(1 << ss as u8);
        let new_value = (value & mask) | ((polarity as u8) << (ss as u8));
        self.spi_raw.ss_sr_polarity.modify(|_, w| w.fc_polarity().variant(new_value));
    }
}

unique_impl_spims! { Spim
    pub fn push_trans(&self, data: u16) {
        where (trans_ptr) =
            SPI0 => (0x4010_A000)
            SPI1 => (0x4010_B000)
            SPI2 => (0x4010_C000)
        // Wait for the FIFO to not be full
        // The bit is set while the FIFO has <16 things in it, and the max
        // it can hold is 16
        while !self.spi_raw.intfl.read().tx_fifo_ae().bit_is_set() {}
        self.spi_raw.intfl.write(|w| w.tx_fifo_ae().clear_bit_by_one());

        unsafe { core::ptr::write_volatile(trans_ptr as *mut u16, data) };
    }

    pub fn pull_rslts(&self) -> Option<u8> {
        where (rslts_ptr) =
            SPI0 => (0x4010_A800)
            SPI1 => (0x4010_B800)
            SPI2 => (0x4010_C800)
        // Only pull from results when the fifo isn't empty, because the processor will
        // hard crash if you don't.
        if self.spi_raw.fifo_ctrl.read().rx_fifo_used().bits() > 0 {
            Some(unsafe { core::ptr::read_volatile(rslts_ptr as *const u8) })
        } else {
            None
        }
    }

    pub fn take_gpio(gpio: &mut GPIO) -> Result<(), SpiError> {
        where (pins) =
            SPI0 => ([
                        (Port::Port0, PinNum::Pin4),
                        (Port::Port0, PinNum::Pin5),
                        (Port::Port0, PinNum::Pin6)
                    ])
            SPI1 => ([
                        (Port::Port1, PinNum::Pin0),
                        (Port::Port1, PinNum::Pin1),
                        (Port::Port1, PinNum::Pin2),
                    ])
            SPI2 => ([
                        (Port::Port2, PinNum::Pin4),
                        (Port::Port2, PinNum::Pin5),
                        (Port::Port2, PinNum::Pin6),
                    ])
        for (port, pin) in pins {
            Self::take_pin(gpio, port, pin)?;
        }
        Ok(())
    }

    pub fn request_spim_core_function(ioman: &IOMAN) {
        where (reg) =
            SPI0 => (&ioman.spi0_req)
            SPI1 => (&ioman.spi1_req)
            SPI2 => (&ioman.spi2_req)
        reg.write(|w| w.core_io_req().set_bit().fast_mode().set_bit());
    }

    pub fn request_slave_function(slave: SS, ioman: &IOMAN) -> Result<(), SpiError> {
        where (func) =
            SPI0 => (Self::_request_slave_spi0)
            SPI1 => (Self::_request_slave_spi12)
            SPI2 => (Self::_request_slave_spi12)

        func(slave, ioman)
    }

    fn _request_slave_spi0(slave: SS, ioman: &IOMAN) -> Result<(), SpiError> {
        where (reg) =
            SPI0 => (&ioman.spi0_req)
            SPI1 => (&ioman.spi0_req)
            SPI2 => (&ioman.spi0_req)

        reg.modify(|_, w| match slave {
            SS::Slave0 => w.ss0_io_req().set_bit(),
            SS::Slave1 => w.ss1_io_req().set_bit(),
            SS::Slave2 => w.ss2_io_req().set_bit(),
            SS::Slave3 => w.ss3_io_req().set_bit(),
            SS::Slave4 => w.ss4_io_req().set_bit(),
        });
        Ok(())
    }

    fn _request_slave_spi12(slave: SS, ioman: &IOMAN) -> Result<(), SpiError> {
        where (reg) =
            SPI0 => (&ioman.spi0_req)
            SPI1 => (&ioman.spi1_req)
            SPI2 => (&ioman.spi2_req)

        match slave {
            SS::Slave3 | SS::Slave4 => return Err(SpiError::InvalidSS),
            _ => {},
        }
        reg.modify(|_, w| match slave {
            SS::Slave0 => w.ss0_io_req().set_bit(),
            SS::Slave1 => w.ss1_io_req().set_bit(),
            SS::Slave2 => w.ss2_io_req().set_bit(),
            _ => unreachable!(),
        });
        Ok(())
    }

    pub fn take_slave_gpio(slave: SS, gpio: &mut GPIO) -> Result<(), SpiError> {
        where (pins) =
            SPI0 => ([
                        Ok((Port::Port0, PinNum::Pin7)),
                        Ok((Port::Port4, PinNum::Pin4)),
                        Ok((Port::Port4, PinNum::Pin5)),
                        Ok((Port::Port4, PinNum::Pin6)),
                        Ok((Port::Port4, PinNum::Pin7)),
                    ])
            SPI1 => ([
                        Ok((Port::Port1, PinNum::Pin3)),
                        Ok((Port::Port3, PinNum::Pin6)),
                        Ok((Port::Port3, PinNum::Pin7)),
                        Err(SpiError::InvalidSS),
                        Err(SpiError::InvalidSS),
                    ])
            SPI2 => ([
                        Ok((Port::Port2, PinNum::Pin7)),
                        Ok((Port::Port3, PinNum::Pin4)),
                        Ok((Port::Port3, PinNum::Pin5)),
                        Err(SpiError::InvalidSS),
                        Err(SpiError::InvalidSS),
                    ])

        let (port, pin) = match slave {
            SS::Slave0 => pins[0]?,
            SS::Slave1 => pins[1]?,
            SS::Slave2 => pins[2]?,
            SS::Slave3 => pins[3]?,
            SS::Slave4 => pins[4]?,
        };
        Self::take_pin(gpio, port, pin)?;
        Ok(())
    }

    pub fn init_clk(scale: SpiClockScale, clkman: &CLKMAN) {
        where (_) =
            SPI0 => ({
                clkman
                    .sys_clk_ctrl_11_spi0
                    .modify(|_, w| w.spi0_clk_scale().variant(scale as u8));
                clkman
                    .clk_gate_ctrl2
                    .modify(|_, w| w.spi0_clk_gater().variant(1));
            })
            SPI1 => ({
                clkman
                    .sys_clk_ctrl_12_spi1
                    .modify(|_, w| w.spi1_clk_scale().variant(scale as u8));
                clkman
                    .clk_gate_ctrl2
                    .modify(|_, w| w.spi1_clk_gater().variant(1));
            })
            SPI2 => ({
                clkman
                    .sys_clk_ctrl_13_spi2
                    .modify(|_, w| w.spi2_clk_scale().variant(scale as u8));
                clkman
                    .clk_gate_ctrl2
                    .modify(|_, w| w.spi2_clk_gater().variant(1));
            })
    }
}
