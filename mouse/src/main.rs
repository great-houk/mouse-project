#![no_std]
#![no_main]
use crate::mouse::MOUSE;
use crate::sensor_driver::Pmw3389Driver;
use crate::usb::{can_pull_hid, can_push_hid, setup_usb};
use cortex_m::asm;
use cortex_m::interrupt::{self as interruptm};
use cortex_m_rt::entry;
use max32625::{Interrupt, CLKMAN, FLC, ICC, PWRSEQ, RTCTMR};
use max32625_gpio::GPIO;
use max32625_timer_basic::Timer;
use mouse::Mouse;
use panic_semihosting as _;
use pmw3389_driver::Pmw3389;

mod flash;
mod mouse;
mod mouse_report;
mod sensor_driver;
mod static_borrow;
mod usb;

const VID: u16 = 0x16C0;
const PID: u16 = 0x27DD;

#[entry]
fn main() -> ! {
    // Wait a bit to let anything catastrophic stay debugable
    cortex_m::asm::delay(24_000_000);

    let peripherals = max32625::Peripherals::take().unwrap();
    let cortex_m_peripherals = cortex_m::Peripherals::take().unwrap();
    let usb = peripherals.USB;
    let sensor_spi = peripherals.SPIM1;
    let pwrman = peripherals.PWRMAN;
    let clkman = peripherals.CLKMAN;
    let icc = peripherals.ICC;
    let pwrseq = peripherals.PWRSEQ;
    let flc = peripherals.FLC;
    let rtctmr = peripherals.RTCTMR;
    let adc = peripherals.ADC;
    let mut gpio = GPIO::new(peripherals.GPIO, &clkman);
    let ioman = peripherals.IOMAN;
    let mut nvic = cortex_m_peripherals.NVIC;
    let main_timer = peripherals.TMR0;
    let sensor_timer = peripherals.TMR1;
    let secondary_timer = peripherals.TMR2;
    let scroll_wheel_timer = peripherals.TMR3;

    init_hardware(&clkman, &icc, &pwrseq, &flc, &rtctmr);

    setup_usb(usb, &pwrman, &clkman, &pwrseq, &adc, &mut nvic);

    let sensor_driver = Pmw3389Driver::new(sensor_spi, sensor_timer, &mut gpio, &ioman, &clkman);
    let sensor = interruptm::free(|_| Pmw3389::init(sensor_driver).unwrap());

    Mouse::new(
        &mut nvic,
        &mut gpio,
        &clkman,
        sensor,
        scroll_wheel_timer,
        adc,
        flc,
    )
    .unwrap();

    main_timer
        .setup_loop_us(500, half_ms_loop, &mut nvic)
        .unwrap();

    secondary_timer
        .setup_loop_ms(10, ten_ms_loop, &mut nvic)
        .unwrap();

    loop {
        asm::nop()
    }
}

pub fn half_ms_loop() -> bool {
    static mut BUF: [u8; 64] = [0; 64];

    if can_push_hid() {
        // SAFTTEY: BUF Only get's mutably borrowed once
        let report = interruptm::free(|cs| unsafe {
            MOUSE
                .borrow(cs)
                .borrow_mut()
                .as_mut()
                .unwrap()
                .get_mouse_report(&mut BUF)
        });
        // We can unwrap the error, since we know we can push
        usb::push_hid(report).unwrap();
    }
    if can_pull_hid() {
        interruptm::free(|cs| {
            let mut buf = [0; 5];
            // We can unwrap the error, since we know we can pull
            let report = usb::pull_hid(cs).unwrap();
            // Index 0 has 0x03, the report ID,
            // which we don't care about
            buf.copy_from_slice(&report[1..]);

            MOUSE
                .borrow(cs)
                .borrow_mut()
                .as_mut()
                .unwrap()
                .interpret_mouse_report(&buf);
        });
    }
    // Continue loop
    true
}

pub fn ten_ms_loop() -> bool {
    // Sanity check USB
    // If there's a nack still pending we should
    // manually trigger the interrupt.
    // It feels improper to do this instead of
    // allowing nacks to gen interrupts, but if
    // you allow it, debug mode generally needs >1ms
    // for making a packet, and nacks generating interrupts
    // give it at most an unterrupted 1ms, so it doesn't work.
    // 10ms is more than enough to form a proper packet and buffer it,
    // while also being barely noticable to the user.
    if usb::is_naking() {
        max32625::NVIC::pend(Interrupt::USB);
    }
    // Get battery level. This shouldn't change very fast, so leaving it be
    // for as long as possible works well.
    interruptm::free(|cs| {
        MOUSE
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .update_battery_level()
    });
    // Continue loop
    true
}

fn init_hardware(clkman: &CLKMAN, icc: &ICC, pwrseq: &PWRSEQ, flc: &FLC, rtctmr: &RTCTMR) {
    // Switch to maximum internal clock speed
    clkman
        .clk_ctrl
        .modify(|_, w| w.system_source_select().variant(1));
    // Enabled cache to improve throughput
    icc.ctrl_stat.modify(|_, w| w.enable().set_bit());
    // Copy power trim values
    const TRIM_FOR_PWR_REG5_PTR: *const u32 = 0x4000_1034 as *const u32;
    const TRIM_FOR_PWR_REG6_PTR: *const u32 = 0x4000_1038 as *const u32;
    const TRIM_FOR_PWR_REG7_PTR: *const u32 = 0x4000_103C as *const u32;
    // Read all 32 bits from the addresses
    let trim_for_pwr_reg5 = unsafe { core::ptr::read_volatile(TRIM_FOR_PWR_REG5_PTR) };
    let trim_for_pwr_reg6 = unsafe { core::ptr::read_volatile(TRIM_FOR_PWR_REG6_PTR) };
    let trim_for_pwr_reg7 = unsafe { core::ptr::read_volatile(TRIM_FOR_PWR_REG7_PTR) };

    pwrseq
        .reg5
        .modify(|_, w| unsafe { w.bits(trim_for_pwr_reg5) });
    pwrseq
        .reg6
        .modify(|_, w| unsafe { w.bits(trim_for_pwr_reg6) });
    let high = trim_for_pwr_reg7;
    let low = pwrseq.reg7.read().bits();
    let highb = high & 0xFFFF_0000;
    let lowb = low & 0x0000_FFFF;
    let bits = highb | lowb;
    pwrseq.reg7.write(|w| unsafe { w.bits(bits) });
    // Improve flash access timing
    flc.perform.modify(|_, w| {
        w.en_back2back_rds()
            .set_bit()
            .en_merge_grab_gnt()
            .set_bit()
            .auto_tacc()
            .set_bit()
            .auto_clkdiv()
            .set_bit()
    });
    // // Do not allow direct write operations to internal flash via the AHB bus
    // // Write 3 to set register to 1
    // flc.security.modify(|_, w| w.disable_ahb_wr().variant(3));
    // Improve RTC register access timeing
    rtctmr
        .ctrl
        .modify(|_, w| w.use_async_flags().set_bit().aggressive_rst().set_bit());
    // Enable fast read of RTC timer value
    pwrseq.rtc_ctrl2.modify(|_, w| {
        w.timer_async_rd()
            .clear_bit()
            .timer_async_wr()
            .set_bit()
            .timer_auto_update()
            .set_bit()
    });
    // Select values for optimal performance
    pwrseq.reg3.modify(|_, w| w.pwr_failsel().variant(1));
    // This bit must be clear before entry into LP0 or LP1, and turn on retention register
    pwrseq.reg0.modify(|_, w| {
        w.pwr_first_boot()
            .clear_bit()
            .pwr_retregen_run()
            .set_bit()
            .pwr_retregen_slp()
            .set_bit()
    });
    // Adjust controller settings for fasted wake-up time
    pwrseq.retn_ctrl0.modify(|_, w| {
        w.rc_rel_ccg_early()
            .set_bit()
            .rc_poll_flash()
            .set_bit()
            .rc_use_flc_twk()
            .clear_bit()
    });
    //
    pwrseq.retn_ctrl1.modify(|_, w| w.rtc_twk().variant(1));
    // Improve wake-up time
    // Set powerfail/bootfail retry cylce time to 8ms
    // Set power stabilization delay for wakeup to max value of 5.10us
    pwrseq.reg3.modify(|_, w| {
        w.pwr_rosel()
            .variant(1)
            .pwr_failsel()
            .variant(1)
            .pwr_fltrrosel()
            .variant(7)
    });
    // Enable RTOS Mode: Enable 32kHz clock synchonizer to SysTick external clock input
    clkman.clk_ctrl.modify(|_, w| w.rtos_mode().set_bit());
    // Set this so all bits of pwr_msk_flags are active low to mask the corresponding flags
    pwrseq.pwr_misc.write(|w| w.invert_4_mask_bits().set_bit());
}
