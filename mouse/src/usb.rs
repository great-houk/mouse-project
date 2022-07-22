use crate::{mouse_report::MouseReport, static_borrow::StaticBorrow, PID, VID};
use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};
use cortex_m::{
    asm,
    interrupt::{self as interruptm, CriticalSection, Mutex},
};
use max32625::{interrupt, ADC, CLKMAN, NVIC, PWRMAN, PWRSEQ, USB};
use max32625_usb::UsbBus;
use usb_device::{
    class_prelude::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    UsbError,
};
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};

static USB_ALLOCATOR: StaticBorrow<UsbBusAllocator<UsbBus>> = StaticBorrow::new();
static USB_BUS: Mutex<RefCell<Option<UsbDevice<UsbBus>>>> = Mutex::new(RefCell::new(None));
static USB_HID: Mutex<RefCell<Option<HIDClass<UsbBus>>>> = Mutex::new(RefCell::new(None));
static HID_REPORT: Mutex<RefCell<Option<&'static [u8]>>> = Mutex::new(RefCell::new(None));
static OUT_REPORT: Mutex<RefCell<Option<&'static [u8]>>> = Mutex::new(RefCell::new(None));
static mut IS_READY: bool = false;

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

        *USB_HID.borrow(cs).borrow_mut() =
            Some(HIDClass::new(bus_allocator, MouseReport::desc(), 1));

        *USB_BUS.borrow(cs).borrow_mut() = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(VID, PID))
                .manufacturer("lmaoooo")
                .product("Awesome Mouse 😎")
                .serial_number("PROTO V1")
                .max_power(500)
                .build(),
        );
    });
    // Enable interrupt
    unsafe {
        nvic.set_priority(interrupt::USB, 0);
        max32625::NVIC::unmask(max32625::Interrupt::USB);
    };
    // Wait for USB to complete setup
    while !is_ready() {
        asm::nop()
    }
}

pub fn is_ready() -> bool {
    unsafe { IS_READY }
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

pub fn can_pull_hid() -> bool {
    interruptm::free(|cs| OUT_REPORT.borrow(cs).borrow().is_some())
}

pub fn pull_hid<'a>(cs: &'a CriticalSection) -> Result<&'a [u8], UsbError> {
    if let Some(report) = OUT_REPORT.borrow(cs).borrow_mut().take() {
        Ok(report)
    } else {
        Err(UsbError::WouldBlock)
    }
}

#[interrupt]
fn USB() {
    interruptm::free(|cs| {
        if let (Some(usb_dev), Some(hid)) = (
            &mut *USB_BUS.borrow(cs).borrow_mut(),
            &mut *USB_HID.borrow(cs).borrow_mut(),
        ) {
            // Update setup status
            unsafe { IS_READY = usb_dev.state() == UsbDeviceState::Configured };
            // Poll device
            usb_dev.poll(&mut [hid]);
            // Push HID report
            if let Some(report) = HID_REPORT.borrow(cs).borrow_mut().take() {
                let _ = hid.push_raw_input(&report);
            }
            // Pull HID Report
            static mut BUF: [u8; 128] = [0; 128];
            if OUT_REPORT.borrow(cs).borrow().is_none() {
                if let Ok(info) = hid.pull_raw_report(unsafe { &mut BUF }) {
                    *OUT_REPORT.borrow(cs).borrow_mut() = Some(unsafe { &BUF[..info.len] })
                };
            }
        }
    });
}
