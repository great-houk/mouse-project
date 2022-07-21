use crate::gpio::GpioReg;

#[derive(Clone, Copy, Debug)]
pub enum Port {
    Port0 = 0,
    Port1 = 1,
    Port2 = 2,
    Port3 = 3,
    Port4 = 4,
}

#[derive(Clone, Copy, Debug)]
pub enum PinNum {
    Pin0 = 0,
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
    Pin4 = 4,
    Pin5 = 5,
    Pin6 = 6,
    Pin7 = 7,
}

#[derive(Clone, Copy, Debug)]
pub enum PinOutMode {
    // Possible With Value == 0
    HighImpedanceInput = 0x00 | 0,
    OpenDriveLow = 0x00 | 1,
    NormalDriveLow = 0x00 | 5,
    SlowDriveLow = 0x00 | 7,
    FastDriveLow = 0x00 | 9,
    WeakPulldown = 0x00 | 10,
    HighImpedance = 0x00 | 15,
    // Possible With Value == 1
    WeakPullup = 0x10 | 0,
    NormalDriveHigh = 0x10 | 5,
    SlowDriveHigh = 0x10 | 7,
    FastDriveHigh = 0x10 | 9,
    OpenDriveHigh = 0x10 | 11,
}

impl PinOutMode {
    fn get_mode_and_value(self) -> (u8, bool) {
        (self as u8 & 0xF, (self as u8 & 0xF0) == 0x10)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PinInMode {
    Normal = 0b00,
    Inverted = 0b01,
    Always0 = 0b10,
    Always1 = 0b11,
}

#[derive(Clone, Copy, Debug)]
pub enum PinInterruptMode {
    Disabled = 0b000,
    FallingEdge = 0b001,
    RisingEdge = 0b010,
    EdgeDetect = 0b011,
    ActiveLow = 0b100,
    ActiveHigh = 0b101,
}

#[derive(Clone, Copy, Debug)]
pub enum PinVoltageSupply {
    VDDIO = 0,
    VDDIOH = 1,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum GpioError {
    PinAlreadyTaken,
    PinNotFree,
}

pub struct Pin {
    gpio: GpioReg,
    port: Port,
    pin_num: PinNum,
}

impl Pin {
    pub unsafe fn new(gpio: GpioReg, port: Port, pin_num: PinNum) -> Self {
        Self {
            gpio,
            port,
            pin_num,
        }
    }

    pub fn port(&self) -> Port {
        self.port
    }

    pub fn pin_num(&self) -> PinNum {
        self.pin_num
    }

    pub fn set_output(&mut self, mode: PinOutMode) -> Result<(), GpioError> {
        // Make sure we're free and the pin will actually change
        if !self.is_free() {
            return Err(GpioError::PinNotFree);
        }
        // Change the mode and value of the pin
        let (mode, value) = mode.get_mode_and_value();
        let get_mode = |bits| {
            let shift = self.pin_num as u8 * 4;
            (bits & !(0xF << shift)) | ((mode as u32) << shift)
        };
        let get_val = |bits| {
            let shift = self.pin_num as u8;
            (bits & !(0x1 << shift)) | ((value as u32) << shift)
        };
        match self.port {
            Port::Port0 => {
                self.gpio
                    .out_mode_p0
                    .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) });
                self.gpio
                    .out_val_p0
                    .modify(|r, w| unsafe { w.bits(get_val(r.bits())) });
            }
            Port::Port1 => {
                self.gpio
                    .out_mode_p1
                    .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) });
                self.gpio
                    .out_val_p1
                    .modify(|r, w| unsafe { w.bits(get_val(r.bits())) });
            }
            Port::Port2 => {
                self.gpio
                    .out_mode_p2
                    .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) });
                self.gpio
                    .out_val_p2
                    .modify(|r, w| unsafe { w.bits(get_val(r.bits())) });
            }
            Port::Port3 => {
                self.gpio
                    .out_mode_p3
                    .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) });
                self.gpio
                    .out_val_p3
                    .modify(|r, w| unsafe { w.bits(get_val(r.bits())) });
            }
            Port::Port4 => {
                self.gpio
                    .out_mode_p4
                    .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) });
                self.gpio
                    .out_val_p4
                    .modify(|r, w| unsafe { w.bits(get_val(r.bits())) });
            }
        }

        Ok(())
    }

    pub fn set_in_mode(&mut self, mode: PinInMode) {
        // Change mode of the pin
        let get_mode = |bits| {
            let shift = self.pin_num as u8 * 4;
            (bits & !(0xF << shift)) | ((mode as u32) << shift)
        };
        match self.port {
            Port::Port0 => self
                .gpio
                .in_mode_p0
                .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) }),
            Port::Port1 => self
                .gpio
                .in_mode_p1
                .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) }),
            Port::Port2 => self
                .gpio
                .in_mode_p2
                .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) }),
            Port::Port3 => self
                .gpio
                .in_mode_p3
                .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) }),
            Port::Port4 => self
                .gpio
                .in_mode_p4
                .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) }),
        }
    }

    pub fn read(&self) -> bool {
        let get_state = |bits| (bits >> self.pin_num as u8) & 0x1 == 1;
        match self.port {
            Port::Port0 => get_state(self.gpio.in_val_p0.read().bits()),
            Port::Port1 => get_state(self.gpio.in_val_p1.read().bits()),
            Port::Port2 => get_state(self.gpio.in_val_p2.read().bits()),
            Port::Port3 => get_state(self.gpio.in_val_p3.read().bits()),
            Port::Port4 => get_state(self.gpio.in_val_p4.read().bits()),
        }
    }

    pub fn set_int_mode(&mut self, mode: PinInterruptMode) {
        // Change mode of the pin
        let get_mode = |bits| {
            let shift = self.pin_num as u8 * 4;
            (bits & !(0xF << shift)) | ((mode as u32) << shift)
        };
        match self.port {
            Port::Port0 => self
                .gpio
                .int_mode_p0
                .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) }),
            Port::Port1 => self
                .gpio
                .int_mode_p1
                .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) }),
            Port::Port2 => self
                .gpio
                .int_mode_p2
                .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) }),
            Port::Port3 => self
                .gpio
                .int_mode_p3
                .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) }),
            Port::Port4 => self
                .gpio
                .int_mode_p4
                .modify(|r, w| unsafe { w.bits(get_mode(r.bits())) }),
        }
    }

    pub fn enable_interrupt(&mut self, enable: bool) {
        // Set enabled
        let get_val = |bits| {
            let shift = self.pin_num as u8;
            (bits & !(0x1 << shift)) | ((enable as u32) << shift)
        };
        match self.port {
            Port::Port0 => self
                .gpio
                .inten_p0
                .modify(|r, w| unsafe { w.bits(get_val(r.bits())) }),
            Port::Port1 => self
                .gpio
                .inten_p1
                .modify(|r, w| unsafe { w.bits(get_val(r.bits())) }),
            Port::Port2 => self
                .gpio
                .inten_p2
                .modify(|r, w| unsafe { w.bits(get_val(r.bits())) }),
            Port::Port3 => self
                .gpio
                .inten_p3
                .modify(|r, w| unsafe { w.bits(get_val(r.bits())) }),
            Port::Port4 => self
                .gpio
                .inten_p4
                .modify(|r, w| unsafe { w.bits(get_val(r.bits())) }),
        }
    }

    pub fn clear_interrupt_flag(&mut self) {
        // Clear flag
        let bits = || 1 << self.pin_num as u8;
        match self.port {
            Port::Port0 => self.gpio.intfl_p0.write(|w| unsafe { w.bits(bits()) }),
            Port::Port1 => self.gpio.intfl_p1.write(|w| unsafe { w.bits(bits()) }),
            Port::Port2 => self.gpio.intfl_p2.write(|w| unsafe { w.bits(bits()) }),
            Port::Port3 => self.gpio.intfl_p3.write(|w| unsafe { w.bits(bits()) }),
            Port::Port4 => self.gpio.intfl_p4.write(|w| unsafe { w.bits(bits()) }),
        }
    }

    pub fn set_supply(&mut self, supply: PinVoltageSupply) {
        const IOMAN_USE_VDDIOH_0: *mut u32 = 0x4000_0D00 as _;
        const IOMAN_USE_VDDIOH_1: *mut u32 = 0x4000_0D04 as _;

        if self.port as usize == 4 {
            // Use vddioh_1
            let curr_value = unsafe { core::ptr::read_volatile(IOMAN_USE_VDDIOH_1) };
            let shift = self.pin_num as u8;
            let new_value = (curr_value & !(0x1 << shift)) | ((supply as u32) << shift);
            unsafe { core::ptr::write_volatile(IOMAN_USE_VDDIOH_1, new_value) };
        } else {
            // use vddioh_0
            let curr_value = unsafe { core::ptr::read_volatile(IOMAN_USE_VDDIOH_0) };
            let shift = self.pin_num as u8 + 8 * self.port as u8;
            let new_value = (curr_value & !(0x1 << shift)) | ((supply as u32) << shift);
            unsafe { core::ptr::write_volatile(IOMAN_USE_VDDIOH_0, new_value) };
        }
    }

    pub fn is_free(&self) -> bool {
        let check_free = |bits| (bits >> self.pin_num as u8) & 0x1 == 1;

        match self.port {
            Port::Port0 => check_free(self.gpio.free_p0.read().bits()),
            Port::Port1 => check_free(self.gpio.free_p1.read().bits()),
            Port::Port2 => check_free(self.gpio.free_p2.read().bits()),
            Port::Port3 => check_free(self.gpio.free_p3.read().bits()),
            Port::Port4 => check_free(self.gpio.free_p4.read().bits()),
        }
    }
}

unsafe impl Send for Pin {}
