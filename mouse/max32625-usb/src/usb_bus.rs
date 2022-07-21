use core::cell::{RefCell, UnsafeCell};
use cortex_m::interrupt::{self as interruptm, Mutex};
use max32625::{ADC, CLKMAN, PWRMAN, PWRSEQ, USB};
use usb_device::bus::{self, UsbBusAllocator};
use usb_device::{class_prelude::*, UsbDirection};

use crate::usb::{EpBuffers, EpDescriptors, Usb, EP_BUFFER_COUNT, EP_BUFFER_SIZE, EP_COUNT};

static BUFFER: StaticMutBorrow<EpBuffers> =
    StaticMutBorrow::new(EpBuffers([[0; EP_BUFFER_SIZE]; EP_BUFFER_COUNT]));
static DESCRIPTORS: StaticMutBorrow<EpDescriptors> = StaticMutBorrow::new(EpDescriptors::empty());

struct StaticMutBorrow<T: 'static> {
    inner: UnsafeCell<T>,
}

impl<T: 'static> StaticMutBorrow<T> {
    pub const fn new(t: T) -> Self {
        StaticMutBorrow {
            inner: UnsafeCell::new(t),
        }
    }

    /// Saftey: Can Only use Once
    pub unsafe fn borrow(&self) -> &'static mut T {
        &mut *self.inner.get()
    }
}

unsafe impl<T: 'static> Send for StaticMutBorrow<T> {}
unsafe impl<T: 'static> Sync for StaticMutBorrow<T> {}

pub struct UsbBus {
    usb: Mutex<RefCell<Usb>>,
}

impl UsbBus {
    pub fn new(
        usb: USB,
        pwrman: &PWRMAN,
        clkman: &CLKMAN,
        pwrseq: &PWRSEQ,
        adc: &ADC,
    ) -> UsbBusAllocator<Self> {
        // Make USB
        let usb = unsafe {
            Usb::new(
                usb,
                pwrman,
                clkman,
                pwrseq,
                adc,
                BUFFER.borrow(),
                DESCRIPTORS.borrow(),
            )
        };
        // Return
        UsbBusAllocator::new(Self {
            usb: Mutex::new(RefCell::new(usb)),
        })
    }

    fn with_ref<R>(&self, f: impl FnOnce(&Usb) -> R) -> R {
        interruptm::free(|cs| {
            let usb = self.usb.borrow(cs);
            let usb = usb.borrow();
            f(&*usb)
        })
    }

    fn with_mut<R>(&self, f: impl FnOnce(&mut Usb) -> R) -> R {
        interruptm::free(|cs| {
            let usb = self.usb.borrow(cs);
            let mut usb = usb.borrow_mut();
            f(&mut *usb)
        })
    }
}

impl bus::UsbBus for UsbBus {
    const QUIRK_SET_ADDRESS_BEFORE_STATUS: bool = false;

    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        _max_packet_size: u16,
        _interval: u8,
    ) -> usb_device::Result<EndpointAddress> {
        self.with_mut(|usb| {
            if let Some(addr) = ep_addr {
                usb.alloc_ep(ep_dir, addr.index(), ep_type)
            } else {
                // Find address
                if ep_type == EndpointType::Control {
                    usb.alloc_ep(ep_dir, 0, ep_type)
                } else {
                    for index in 1..EP_COUNT {
                        if !usb.endpoint_is_allocated(index) {
                            return usb.alloc_ep(ep_dir, index, ep_type);
                        }
                    }
                    Err(UsbError::EndpointOverflow)
                }
            }
        })
    }

    fn enable(&mut self) {
        self.with_mut(|usb| usb.enable());
    }

    fn reset(&self) {
        self.with_mut(|usb| usb.reset());
    }

    fn set_device_address(&self, _addr: u8) {
        /* Do nothing */
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> usb_device::Result<usize> {
        self.with_mut(|usb| usb.write(ep_addr.index(), buf))
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> usb_device::Result<usize> {
        self.with_mut(|usb| usb.read(ep_addr.index(), buf))
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        self.with_mut(|usb| usb.set_stalled(ep_addr.index(), stalled));
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        self.with_ref(|usb| usb.is_stalled(ep_addr.index()))
    }

    fn suspend(&self) {
        /* Do nothing */
    }

    fn resume(&self) {
        /* Do nothing */
    }

    fn poll(&self) -> bus::PollResult {
        self.with_mut(|usb| usb.poll())
    }

    fn force_reset(&self) -> usb_device::Result<()> {
        self.with_mut(|usb| usb.force_reset());
        Ok(())
    }
}
