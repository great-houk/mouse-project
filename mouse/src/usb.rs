use crate::{mouse_report::MouseReport, static_borrow::StaticBorrow, PID, VID};
use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};
use cortex_m::{
    asm,
    interrupt::{self as interruptm, CriticalSection, Mutex},
};
use max32625::{interrupt, Interrupt, ADC, CLKMAN, NVIC, PWRMAN, PWRSEQ, USB};
use max32625_usb::UsbBus;
use usb_device::{
    class_prelude::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    UsbError,
};
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};
use usbd_serial::SerialPort;

static USB_ALLOCATOR: StaticBorrow<UsbBusAllocator<UsbBus>> = StaticBorrow::new();
static USB_BUS: Mutex<RefCell<Option<UsbDevice<UsbBus>>>> = Mutex::new(RefCell::new(None));
static USB_HID: Mutex<RefCell<Option<HIDClass<UsbBus>>>> = Mutex::new(RefCell::new(None));
static USB_SERIAL: Mutex<RefCell<Option<SerialPort<UsbBus>>>> = Mutex::new(RefCell::new(None));
static HID_REPORT: Mutex<RefCell<Option<&'static [u8]>>> = Mutex::new(RefCell::new(None));
static IN_REPORT: Mutex<RefCell<Option<&'static [u8]>>> = Mutex::new(RefCell::new(None));
static OUT_REPORT: Mutex<RefCell<Option<&'static [u8]>>> = Mutex::new(RefCell::new(None));
static IS_READY: AtomicBool = AtomicBool::new(false);
pub static mut POLLING_RATE: u8 = 1; // Number of ms per poll

pub fn setup_usb(
    usb: USB,
    pwrman: &PWRMAN,
    clkman: &CLKMAN,
    pwrseq: &PWRSEQ,
    adc: &ADC,
    nvic: &mut NVIC,
) {
    // Set up the USB device
    interruptm::free(|cs| {
        // SAFTEY: No interrupts are allowed, so nothing can access the static mut
        USB_ALLOCATOR.set(UsbBus::new(usb, pwrman, clkman, pwrseq, adc));
        let bus_allocator = USB_ALLOCATOR.borrow();

        *USB_SERIAL.borrow(cs).borrow_mut() = Some(SerialPort::new(bus_allocator));

        *USB_HID.borrow(cs).borrow_mut() =
            Some(HIDClass::new(bus_allocator, MouseReport::desc(), unsafe {
                POLLING_RATE
            }));

        *USB_BUS.borrow(cs).borrow_mut() = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(VID, PID))
                .manufacturer("Me")
                .product("Awesome Mouse 😎")
                .composite_with_iads()
                .serial_number("PROTO V1")
                .max_power(500)
                .build(),
        );
    });
    // Enable interrupt
    unsafe {
        nvic.set_priority(interrupt::USB, 0);
        NVIC::unmask(Interrupt::USB);
    };
    // Wait for USB to complete setup
    while !is_ready() {
        asm::nop()
    }
}

pub fn reset_usb() {
    // Disable interrupt
    NVIC::mask(Interrupt::USB);
    // Get all necessary singletons
    let periph = unsafe { max32625::Peripherals::steal() };
    let usb = periph.USB;
    let pwrman = &periph.PWRMAN;
    let clkman = &periph.CLKMAN;
    let pwrseq = &periph.PWRSEQ;
    let adc = &periph.ADC;
    let nvic = unsafe { &mut cortex_m::Peripherals::steal().NVIC };
    // Disconnect USB
    usb.cn.modify(|_, w| w.usb_en().clear_bit());
    // Delay a sec to give host time to understand what's happening
    cortex_m::asm::delay(100_000_000);
    // Set all static to none, so there's no dangling pointers
    interruptm::free(|cs| {
        *USB_BUS.borrow(cs).borrow_mut() = None;

        *USB_HID.borrow(cs).borrow_mut() = None;
        *USB_SERIAL.borrow(cs).borrow_mut() = None;
        // SAFTEY: No interrupts are allowed, so nothing can access the static mut
        unsafe { USB_ALLOCATOR.reset() };
    });
    // Setup USB again
    setup_usb(usb, pwrman, clkman, pwrseq, adc, nvic);
}

pub fn is_ready() -> bool {
    IS_READY.load(Ordering::Relaxed)
}

pub fn is_naking() -> bool {
    unsafe {
        // Check nak status
        let mut nak = false;
        max32625::Peripherals::steal().USB.dev_intfl.modify(|r, w| {
            nak = r.ep_nak().bit_is_set();
            w.ep_nak().clear_bit_by_one()
        });
        nak
    }
}

pub fn can_push_hid() -> bool {
    interruptm::free(|cs| HID_REPORT.borrow(cs).borrow().is_none())
}

pub fn push_hid(report: &'static [u8]) -> Result<(), UsbError> {
    interruptm::free(|cs| {
        if HID_REPORT.borrow(cs).borrow().is_some() {
            Err(UsbError::WouldBlock)
        } else {
            *HID_REPORT.borrow(cs).borrow_mut() = Some(report);
            Ok(())
        }
    })
}

pub fn can_pull_report() -> bool {
    interruptm::free(|cs| OUT_REPORT.borrow(cs).borrow().is_some())
}

pub fn pull_report<'a>(cs: &'a CriticalSection) -> Result<&'a [u8], UsbError> {
    if let Some(report) = OUT_REPORT.borrow(cs).borrow_mut().take() {
        Ok(report)
    } else {
        Err(UsbError::WouldBlock)
    }
}

pub fn can_push_report() -> bool {
    interruptm::free(|cs| IN_REPORT.borrow(cs).borrow().is_none())
}

pub fn push_report(report: &'static [u8]) -> Result<(), UsbError> {
    interruptm::free(|cs| {
        if IN_REPORT.borrow(cs).borrow().is_some() {
            Err(UsbError::WouldBlock)
        } else {
            *IN_REPORT.borrow(cs).borrow_mut() = Some(report);
            Ok(())
        }
    })
}

#[interrupt]
fn USB() {
    interruptm::free(|cs| {
        if let (Some(usb_dev), Some(hid), Some(serial)) = (
            &mut *USB_BUS.borrow(cs).borrow_mut(),
            &mut *USB_HID.borrow(cs).borrow_mut(),
            &mut *USB_SERIAL.borrow(cs).borrow_mut(),
        ) {
            // Update setup status
            IS_READY.store(
                usb_dev.state() == UsbDeviceState::Configured,
                Ordering::Relaxed,
            );
            // Poll device
            usb_dev.poll(&mut [serial, hid]);
            // Pull HID Report from serial buffer
            static mut BUF: [u8; 128] = [0; 128];
            if OUT_REPORT.borrow(cs).borrow().is_none() {
                if let Ok(len) = serial.read(unsafe { &mut BUF }) {
                    *OUT_REPORT.borrow(cs).borrow_mut() = Some(unsafe { &BUF[..len] })
                };
            }
            // Write Response to Serial Buffer
            if let Some(buf) = &mut *IN_REPORT.borrow(cs).borrow_mut() {
                match serial.write(&buf) {
                    Ok(count) => *buf = &buf[count..],
                    Err(UsbError::WouldBlock) => {}
                    Err(_) => { /* There isn't really any proper recourse for an error here */ }
                }
            }
            match &*IN_REPORT.borrow(cs).borrow() {
                Some(buf) if buf.len() == 0 => {
                    *IN_REPORT.borrow(cs).borrow_mut() = None;
                }
                _ => {}
            }
        }
    });
}
