use crate::pin::{GpioError, Pin, PinNum, Port};
use core::ops::Deref;
use max32625::gpio;
use max32625::CLKMAN;
use max32625::GPIO as RAW_GPIO;

pub struct GPIO {
    // Raw gpio
    _gpio: RAW_GPIO,
    // Pins
    taken: [bool; 40],
}

impl From<RAW_GPIO> for GPIO {
    fn from(gpio: RAW_GPIO) -> Self {
        Self {
            _gpio: gpio,
            taken: [false; 40],
        }
    }
}

impl GPIO {
    pub fn new(gpio: RAW_GPIO, clkman: &CLKMAN) -> Self {
        clkman
            .sys_clk_ctrl_6_gpio
            .write(|w| w.gpio_clk_scale().variant(9));
        gpio.into()
    }

    pub fn take(&mut self, port: Port, pin: PinNum) -> Result<Pin, GpioError> {
        let ind = Self::to_ind(port, pin);
        if self.taken[ind] {
            return Err(GpioError::PinAlreadyTaken);
        }
        self.taken[ind] = true;
        let gpio = unsafe { GpioReg::new(RAW_GPIO::ptr()) };
        Ok(unsafe { Pin::new(gpio, port, pin) })
    }

    pub fn put_back(&mut self, pin: Pin) {
        let ind = Self::to_ind(pin.port(), pin.pin_num());
        self.taken[ind] = false;
    }

    fn to_ind(port: Port, pin: PinNum) -> usize {
        (port as usize * 8) + (pin as usize)
    }
}

pub struct GpioReg {
    ptr: *const gpio::RegisterBlock,
}

impl GpioReg {
    pub unsafe fn new(ptr: *const gpio::RegisterBlock) -> Self {
        Self { ptr }
    }
}

impl Deref for GpioReg {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}
