use core::cell::UnsafeCell;

use max32625::{interrupt, Interrupt, NVIC};

const TIMER_PRIORITY: u8 = 1 << 3;

struct NoneInit<T> {
    inner: UnsafeCell<Option<T>>,
}

impl<T> NoneInit<T> {
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    pub fn get(&self) -> &Option<T> {
        unsafe { &*self.inner.get() }
    }

    pub fn set(&self, val: T) {
        let inner = unsafe { &mut *self.inner.get() };
        *inner = Some(val);
    }
}

unsafe impl<T> Send for NoneInit<T> {}
unsafe impl<T> Sync for NoneInit<T> {}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TimerError {
    InvalidDelay,
}

pub enum TimerMode {
    OneShot = 0,
    Continuous = 1,
}

pub trait Timer {
    fn setup_timer(&self, us: u32, mode: TimerMode) -> Result<(), TimerError>;
    fn delay_us(&self, us: u32) -> Result<(), TimerError>;
    fn delay_ms(&self, ms: u32) -> Result<(), TimerError> {
        self.delay_us(ms * 1000)
    }
    /// Calls callback every us microseconds until callback returns false, at which point it stops
    fn setup_loop_us(
        &self,
        us: u32,
        callback: fn() -> bool,
        nvic: &mut NVIC,
    ) -> Result<(), TimerError>;
    /// Calls callback every ms milliseconds until callback returns false, at which point it stops
    fn setup_loop_ms(
        &self,
        ms: u32,
        callback: fn() -> bool,
        nvic: &mut NVIC,
    ) -> Result<(), TimerError> {
        self.setup_loop_us(ms * 1000, callback, nvic)
    }
}

enum Timers {
    TMR0,
    TMR1,
    TMR2,
    TMR3,
    TMR4,
    TMR5,
}

impl_for_raw_timers! { Timer / Timers :
    /// Max delay is 0x5555_5555 (1431655765 in decimal) microseconds or about 24 min
    fn setup_timer(&self, us: u32, mode: TimerMode) -> Result<(), TimerError> {
        // Check valid us
        if us >= (0xFFFF_FFFF / 3) || us == 0 {
            return Err(TimerError::InvalidDelay);
        }
        /*
        The steps for configuring a 32-bit timer are as follows:
            1. Disable the timer by setting TMRn_CTRL.enable0 to 0.
            2. Select 32-bit timer mode by setting TMRn_CTRL.tmr2x16 to 0.
            3. Select the One-Shot timer mode by setting TMRn_CTRL.mode to 0.
            4. Set TMRn_CTRL.prescale to the desired prescale value.
            5. If the timer output function will be used, set the initial output level (0 or 1) in the TMRn_CTRL.polarity field.
            6. If the timer output function will be used, configure the desired GPIO pin(s) to connect to the timer output as described in Timer GPIO Mapping.
            7. Set TMRn_COUNT32 to the initial timer count value.
            8. Set TMRn_TERM_CNT32 to the terminal count value.
            9. If desired, enable the timer interrupt by setting TMRn_INTEN.timer0 to 1.
        */
        // Use 5, because that equals main clock (96mHz) / 32, which is 3 mHz
        // This allows for easy micro second precision by multiplying the us value by 3
        const DIV_32: u8 = 5;
        // Disable Timer, Set for 32 Bit mode, set for oneshot operation,
        // and set prescale to desired value.
        self.ctrl.write(|w| w
            .enable0().clear_bit()
            .tmr2x16().clear_bit()
            .mode().variant(mode as u8)
            .prescale().variant(DIV_32)
        );
        // Set timer to proper start count
        self.count32.write(|w| unsafe { w.bits(1) });
        // Set timer termination to (us * 3) + 1
        self.term_cnt32.write(|w| unsafe { w.bits((us * 3) + 1) });
        // Timer is set up, so return ok
        Ok(())
    }
    fn delay_us(&self, us: u32) -> Result<(), TimerError> {
        // Setup oneshot timer
        self.setup_timer(us, TimerMode::OneShot)?;
        // Enable timer
        self.ctrl.modify(|_, w| w.enable0().set_bit());
        // Wait for flag
        while self.intfl.read().timer0().bit_is_clear() { cortex_m::asm::nop(); }
        self.intfl.write(|w| w.timer0().clear_bit_by_one());
        // Return, since timer is done
        Ok(())
    }
    fn setup_loop_us(&self, us: u32, callback: fn() -> bool, nvic: &mut NVIC) -> Result<(), TimerError> {
        // Setup Continuous timer
        self.setup_timer(us, TimerMode::Continuous)?;
        // Enable interrupt
        self.inten.modify(|_, w| w.timer0().set_bit());
        unsafe { match TMR {
            Timers::TMR0 => enable_int_tmr0(nvic,callback),
            Timers::TMR1 => enable_int_tmr1(nvic,callback),
            Timers::TMR2 => enable_int_tmr2(nvic,callback),
            Timers::TMR3 => enable_int_tmr3(nvic,callback),
            Timers::TMR4 => enable_int_tmr4(nvic,callback),
            Timers::TMR5 => enable_int_tmr5(nvic,callback),
        }};
        // Enable timer
        self.ctrl.modify(|_, w| w.enable0().set_bit());
        Ok(())
    }
}

unsafe fn enable_int_tmr0(nvic: &mut NVIC, callback: fn() -> bool) {
    // Set priority
    nvic.set_priority(Interrupt::TMR0, TIMER_PRIORITY);
    // Set callback
    TMR0_CALLBACK.set(callback);
    // Enable interrupt
    max32625::NVIC::unmask(Interrupt::TMR0);
}
unsafe fn enable_int_tmr1(nvic: &mut NVIC, callback: fn() -> bool) {
    // Set priority
    nvic.set_priority(Interrupt::TMR1, TIMER_PRIORITY);
    // Set callback
    TMR1_CALLBACK.set(callback);
    // Enable interrupt
    max32625::NVIC::unmask(Interrupt::TMR1);
}
unsafe fn enable_int_tmr2(nvic: &mut NVIC, callback: fn() -> bool) {
    // Set priority
    nvic.set_priority(Interrupt::TMR2, TIMER_PRIORITY);
    // Set callback
    TMR2_CALLBACK.set(callback);
    // Enable interrupt
    max32625::NVIC::unmask(Interrupt::TMR2);
}
unsafe fn enable_int_tmr3(nvic: &mut NVIC, callback: fn() -> bool) {
    // Set priority
    nvic.set_priority(Interrupt::TMR3, TIMER_PRIORITY);
    // Set callback
    TMR3_CALLBACK.set(callback);
    // Enable interrupt
    max32625::NVIC::unmask(Interrupt::TMR3);
}
unsafe fn enable_int_tmr4(nvic: &mut NVIC, callback: fn() -> bool) {
    // Set priority
    nvic.set_priority(Interrupt::TMR4, TIMER_PRIORITY);
    // Set callback
    TMR4_CALLBACK.set(callback);
    // Enable interrupt
    max32625::NVIC::unmask(Interrupt::TMR4);
}
unsafe fn enable_int_tmr5(nvic: &mut NVIC, callback: fn() -> bool) {
    // Set priority
    nvic.set_priority(Interrupt::TMR5, TIMER_PRIORITY);
    // Set callback
    TMR5_CALLBACK.set(callback);
    // Enable interrupt
    max32625::NVIC::unmask(Interrupt::TMR5);
}

static TMR0_CALLBACK: NoneInit<fn() -> bool> = NoneInit::new();
#[interrupt]
fn TMR0() {
    // Clear flag
    let timer = unsafe { max32625::Peripherals::steal() }.TMR0;
    timer.intfl.write(|w| w.timer0().clear_bit_by_one());
    // Run callback
    if let Some(func) = TMR0_CALLBACK.get() {
        if !func() {
            // Break
            max32625::NVIC::mask(Interrupt::TMR0);
            timer.ctrl.modify(|_, w| w.enable0().clear_bit());
        }
    }
}
static TMR1_CALLBACK: NoneInit<fn() -> bool> = NoneInit::new();
#[interrupt]
fn TMR1() {
    // Clear flag
    let timer = unsafe { max32625::Peripherals::steal() }.TMR1;
    timer.intfl.write(|w| w.timer0().clear_bit_by_one());
    // Run callback
    if let Some(func) = TMR1_CALLBACK.get() {
        if !func() {
            // Break
            max32625::NVIC::mask(Interrupt::TMR1);
            timer.ctrl.modify(|_, w| w.enable0().clear_bit());
        }
    }
}
static TMR2_CALLBACK: NoneInit<fn() -> bool> = NoneInit::new();
#[interrupt]
fn TMR2() {
    // Clear flag
    let timer = unsafe { max32625::Peripherals::steal() }.TMR2;
    timer.intfl.write(|w| w.timer0().clear_bit_by_one());
    // Run callback
    if let Some(func) = TMR2_CALLBACK.get() {
        if !func() {
            // Break
            max32625::NVIC::mask(Interrupt::TMR2);
            timer.ctrl.modify(|_, w| w.enable0().clear_bit());
        }
    }
}
static TMR3_CALLBACK: NoneInit<fn() -> bool> = NoneInit::new();
#[interrupt]
fn TMR3() {
    // Clear flag
    let timer = unsafe { max32625::Peripherals::steal() }.TMR3;
    timer.intfl.write(|w| w.timer0().clear_bit_by_one());
    // Run callback
    if let Some(func) = TMR3_CALLBACK.get() {
        if !func() {
            // Break
            max32625::NVIC::mask(Interrupt::TMR3);
            timer.ctrl.modify(|_, w| w.enable0().clear_bit());
        }
    }
}
static TMR4_CALLBACK: NoneInit<fn() -> bool> = NoneInit::new();
#[interrupt]
fn TMR4() {
    // Clear flag
    let timer = unsafe { max32625::Peripherals::steal() }.TMR4;
    timer.intfl.write(|w| w.timer0().clear_bit_by_one());
    // Run callback
    if let Some(func) = TMR4_CALLBACK.get() {
        if !func() {
            // Break
            max32625::NVIC::mask(Interrupt::TMR4);
            timer.ctrl.modify(|_, w| w.enable0().clear_bit());
        }
    }
}
static TMR5_CALLBACK: NoneInit<fn() -> bool> = NoneInit::new();
#[interrupt]
fn TMR5() {
    // Clear flag
    let timer = unsafe { max32625::Peripherals::steal() }.TMR5;
    timer.intfl.write(|w| w.timer0().clear_bit_by_one());
    // Run callback
    if let Some(func) = TMR5_CALLBACK.get() {
        if !func() {
            // Break
            max32625::NVIC::mask(Interrupt::TMR5);
            timer.ctrl.modify(|_, w| w.enable0().clear_bit());
        }
    }
}
