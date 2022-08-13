use crate::endpoint::{Endpoint, EpDir};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use max32625::{ADC, CLKMAN, PWRMAN, PWRSEQ, USB};
use usb_device::{bus, class_prelude::*, UsbDirection};

pub const EP_COUNT: usize = 8;
pub const EP_BUFFER_COUNT: usize = EP_COUNT + 1;
pub const MAX_PACKET_SIZE: usize = 1023;
// Bigger than any packet could be
pub const EP_BUFFER_SIZE: usize = MAX_PACKET_SIZE + 1;
const EP_INIT: [Option<Endpoint>; 8] = [None, None, None, None, None, None, None, None];
/// Buffer Mapping
///
/// 0: EP0 Out
///
/// 1: EP0 In
///
/// 2: EP1
///
/// 3: EP2
///
/// 4: EP3
///
/// 5: EP4
///
/// 6: EP5
///
/// 7: EP6
///
/// 8: EP7
#[repr(align(4))]
pub struct EpBuffers(pub [[u8; EP_BUFFER_SIZE]; EP_BUFFER_COUNT]);
impl Index<usize> for EpBuffers {
    type Output = [u8; EP_BUFFER_SIZE];

    fn index(&self, index: usize) -> &Self::Output {
        &(self.0)[index]
    }
}
impl IndexMut<usize> for EpBuffers {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut (self.0)[index]
    }
}

impl From<UsbDirection> for EpDir {
    fn from(dir: UsbDirection) -> Self {
        match dir {
            UsbDirection::Out => EpDir::Out,
            UsbDirection::In => EpDir::In,
        }
    }
}

#[repr(C)]
#[derive(Default)]
struct EpDescriptor {
    buf0_desc: u32,
    buf0_pointer: u32,
    buf1_desc: u32,
    buf1_pointer: u32,
}

#[repr(C, align(512))]
#[derive(Default)]
pub struct EpDescriptors {
    ep0_out: EpDescriptor,
    ep0_in: EpDescriptor,
    eps: [EpDescriptor; EP_COUNT - 1],
}

impl EpDescriptors {
    pub const fn empty() -> Self {
        const EMPTY_EP_DESCRIPTOR: EpDescriptor = EpDescriptor {
            buf0_desc: 0,
            buf0_pointer: 0,
            buf1_desc: 0,
            buf1_pointer: 0,
        };
        EpDescriptors {
            ep0_out: EMPTY_EP_DESCRIPTOR,
            ep0_in: EMPTY_EP_DESCRIPTOR,
            eps: [EMPTY_EP_DESCRIPTOR; EP_COUNT - 1],
        }
    }
}

#[derive(PartialEq)]
enum UsbControlState {
    None,
    RecievedSetup,
    ReadSetup,
    WroteDataPacket,
}

pub struct Usb {
    usb: USB,
    control_state: UsbControlState,
    endpoints: [Option<Endpoint>; 8],
    buffers: &'static mut EpBuffers,
    descriptors: &'static mut EpDescriptors,
}

unsafe impl Send for Usb {}

impl Usb {
    pub fn new(
        usb: USB,
        pwrman: &PWRMAN,
        clkman: &CLKMAN,
        pwrseq: &PWRSEQ,
        adc: &ADC,
        buffers: &'static mut EpBuffers,
        descriptors: &'static mut EpDescriptors,
    ) -> Self {
        // Calibrate clock
        Self::calibrate_usb_clock(pwrseq, adc);
        // Set up hardware
        Self::set_up_hardware(&usb, pwrman, clkman);
        // Set buffer address
        // Divide by 512 (or shift over 9), as the register only cares about the bits after the 9th.
        // This is also why the struct must be 512 aligned.
        let addr = (descriptors as *const EpDescriptors as u32) >> 9;
        usb.ep_base.write(|w| w.ep_base().variant(addr));
        // Set up endpoints
        let endpoints = EP_INIT;
        // Build Struct
        let mut usb = Self {
            usb,
            control_state: UsbControlState::None,
            endpoints,
            buffers,
            descriptors,
        };
        // Init EP0's descriptors
        usb.init_descriptor(0);
        // Return USB
        usb
    }

    pub fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        index: usize,
        ep_type: EndpointType,
    ) -> Result<EndpointAddress, UsbError> {
        // Check if it's a valid index
        if index >= EP_COUNT {
            return Err(UsbError::InvalidEndpoint);
        }
        // Check if the index is valid, given the type
        if ep_type == EndpointType::Control && index != 0 {
            return Err(UsbError::EndpointOverflow);
        }
        // Check if we've already allocated it
        // We don't care if EP0 gets allocated multiple times, since
        // it goes both ways.
        if index != 0 && self.endpoint_is_allocated(index) {
            return Err(UsbError::InvalidEndpoint);
        }
        // Make endpoint
        self.make_endpoint(index);
        // Set up descriptor
        self.init_descriptor(index);
        // Enable and set Endpoint up
        self.endpoints[index].as_mut().unwrap().write(|w| {
            w.dir(ep_dir.into())
                .int_enable(true.into())
                .nak_enable(true.into())
        });
        // Set endpoint to read, if it's an OUT, so that a packet is ready asap
        if ep_dir == UsbDirection::Out {
            self.out_owner(index);
        }
        Ok(EndpointAddress::from_parts(index, ep_dir))
    }

    pub fn enable(&mut self) {
        // Enable interrupt generators
        self.dev_inten.write(|w| {
            w.brst()
                .variant(true)
                .setup()
                .variant(true)
                .ep_in()
                .variant(true)
                .ep_out()
                .variant(true)
                .dma_err()
                .variant(true)
        });
        // Allow Interrupts from EP0
        self.ep0.modify(|_, w| w.ep_int_en().variant(true));
        // Enable Pullup
        self.dev_cn.modify(|_, w| w.connect().variant(true));
    }

    pub fn reset(&mut self) {
        for endpoint in self.endpoints.iter_mut().flatten() {
            // Reset the state of all enabled endpoints
            endpoint.modify(|_, w| {
                w.dt(true.into())
                    .stall(false.into())
                    .st_ack(false.into())
                    .st_stall(false.into())
            });
        }
        // Reset buffer owner state
        // Out should be waiting to read
        // In should be owned by software, ready to write
        self.out_owner.write(|w| w.buf0_owner().variant(0xFF));
        // Reset the state of the control transfer tracker
        self.control_state = UsbControlState::None;
        // Clear interrupts
        self.clear_reset_flag();
    }

    pub fn write(&mut self, index: usize, buf: &[u8]) -> usb_device::Result<usize> {
        // Make sure we allocated this EP
        if !self.endpoint_is_allocated(index) {
            return Err(UsbError::InvalidEndpoint);
        }
        // Make sure buffer is big enough
        if buf.len() > MAX_PACKET_SIZE {
            return Err(UsbError::BufferOverflow);
        }
        // Write to the endpoint like normal
        // Check if a previous packet is pending
        if !self.in_owner(index) {
            // Test for weird case where windows doesn't read all of the previous setup data stage,
            // so a random byte gets transmitted before the next setup data stage. This overwrites
            // that byte with what it should be.
            if index == 0 && self.control_state == UsbControlState::ReadSetup {
                self.control_state = UsbControlState::WroteDataPacket;
            } else {
                // // Since the owner bit was one for this EP, the hardware hasn't written it yet
                return Err(UsbError::WouldBlock);
            }
        }
        // Write buffer to USB buffer
        self.buffers[index + 1][..buf.len()].copy_from_slice(buf);
        // Set endpoint description
        match index {
            0 => self.descriptors.ep0_in.buf0_desc = buf.len() as u32,
            ind => self.descriptors.eps[ind - 1].buf0_desc = buf.len() as u32,
        }
        // Give control to hardware DMA
        self.set_in_owner(index);
        // Return amount wrote
        Ok(buf.len())
    }

    pub fn read(&mut self, index: usize, buf: &mut [u8]) -> usb_device::Result<usize> {
        // Make sure we allocated this EP
        if !self.endpoint_is_allocated(index) {
            return Err(UsbError::InvalidEndpoint);
        }
        // Check if we're writing to endpoint 0, because we have to
        // act differently in order to read/respond to setup packets
        if index == 0 && self.control_state == UsbControlState::RecievedSetup {
            // Return setup packet
            let setup = [
                self.setup0.read().byte0().bits(),
                self.setup0.read().byte1().bits(),
                self.setup0.read().byte2().bits(),
                self.setup0.read().byte3().bits(),
                self.setup1.read().byte4().bits(),
                self.setup1.read().byte5().bits(),
                self.setup1.read().byte6().bits(),
                self.setup1.read().byte7().bits(),
            ];
            // Make sure buffer is big enough and copy
            if buf.len() < setup.len() {
                return Err(UsbError::BufferOverflow);
            }
            buf[..setup.len()].copy_from_slice(&setup);
            // Set state
            self.control_state = UsbControlState::ReadSetup;
            //

            return Ok(setup.len());
        }
        // Read like normal
        // Check if there is a thing to read
        if !self.out_owner(index) {
            return Err(UsbError::WouldBlock);
        }
        // Get recived len
        let len = match index {
            0 => self.descriptors.ep0_out.buf0_desc,
            ind => self.descriptors.eps[ind - 1].buf0_desc,
        } as usize;
        // Check len against buffer
        if len as usize > buf.len() {
            return Err(UsbError::BufferOverflow);
        }
        // Copy to buffer
        let buffer_ind = if index == 0 { 0 } else { index + 1 };
        buf[..len].copy_from_slice(&self.buffers[buffer_ind][..len]);
        // Tell the USB to continue reading
        // Set descriptors to max possible length
        match index {
            0 => self.descriptors.ep0_out.buf0_desc = MAX_PACKET_SIZE as u32,
            ind => self.descriptors.eps[ind - 1].buf0_desc = MAX_PACKET_SIZE as u32,
        }
        // Give control to hardware DMA again
        self.set_out_owner(index);
        // Return read results
        Ok(len)
    }

    pub fn set_stalled(&mut self, index: usize, stalled: bool) {
        if index == 0 {
            self.endpoints[0]
                .as_mut()
                .unwrap()
                .modify(|_, w| w.stall(stalled.into()).st_stall(stalled.into()));
        } else {
            self.endpoints[index]
                .as_mut()
                .unwrap()
                .modify(|_, w| w.stall(stalled.into()));
        }
    }

    pub fn is_stalled(&self, index: usize) -> bool {
        self.endpoints[index]
            .as_ref()
            .unwrap()
            .read()
            .stall()
            .into()
    }

    pub fn poll(&mut self) -> bus::PollResult {
        // Get poll result
        let ret = if self.dev_intfl.read().brst().bit() {
            // The host has called for a bus reset
            bus::PollResult::Reset
        } else if self.dev_intfl.read().ep_in().bit()
            || self.dev_intfl.read().ep_out().bit()
            || self.dev_intfl.read().setup().bit()
            || self.out_owner.read().buf0_owner().bits() != 0xF
        {
            // There has been some packet activity
            let ep_out = self.out_int.read().bits() as u16
                | (!self.out_owner.read().buf0_owner().bits()) as u16;
            let ep_in_complete = self.in_int.read().bits() as u16;
            let ep_setup = self.dev_intfl.read().setup().bit() as u16;
            // Keep track of state, and accept control transfer.
            // This library doesn't do any accepting, so we will by default
            // Also, reset the owner of the in buffer, since some hosts don't always read
            // all of the data in a control transfer.
            if ep_setup > 0 {
                self.control_state = UsbControlState::RecievedSetup;
                self.ep0.modify(|_, w| w.ep_st_ack().set_bit());
            }
            //
            bus::PollResult::Data {
                ep_out,
                ep_in_complete,
                ep_setup,
            }
        } else {
            bus::PollResult::None
        };
        // Clear interrupts
        self.dev_intfl.modify(|r, w| unsafe { w.bits(r.bits()) });
        self.in_int.modify(|r, w| unsafe { w.bits(r.bits()) });
        self.out_int.modify(|r, w| unsafe { w.bits(r.bits()) });
        self.nak_int.modify(|r, w| unsafe { w.bits(r.bits()) });
        self.dma_err_int.modify(|r, w| unsafe { w.bits(r.bits()) });
        self.buf_ovr_int.modify(|r, w| unsafe { w.bits(r.bits()) });
        // Return
        ret
    }

    pub fn force_reset(&mut self) {
        // Disable Pullup
        self.dev_cn.modify(|_, w| w.connect().variant(false));
        // Enable Pullup
        self.dev_cn.modify(|_, w| w.connect().variant(true));
    }

    pub fn endpoint_is_allocated(&self, index: usize) -> bool {
        self.endpoints[index] != None
    }

    fn clear_reset_flag(&mut self) {
        self.dev_intfl.write(|w| w.brst().clear_bit_by_one());
    }

    /// Returns true if software owns the buffer
    fn in_owner(&self, index: usize) -> bool {
        let mask = 1 << index;
        (self.in_owner.read().buf0_owner().bits() & mask) == 0
    }

    /// Sets the owner to hardware
    fn set_in_owner(&self, index: usize) {
        let mask = 1 << index;
        self.in_owner
            .modify(|r, w| w.buf0_owner().variant(r.buf0_owner().bits() | mask));
        // Clear nack for index
        self.nak_int.write(|w| match index {
            0 => w.nak0().clear_bit_by_one(),
            1 => w.nak1().clear_bit_by_one(),
            2 => w.nak2().clear_bit_by_one(),
            3 => w.nak3().clear_bit_by_one(),
            4 => w.nak4().clear_bit_by_one(),
            5 => w.nak5().clear_bit_by_one(),
            6 => w.nak6().clear_bit_by_one(),
            7 => w.nak7().clear_bit_by_one(),
            _ => panic!("Invalid Index!"),
        })
    }

    /// Returns true if software owns the buffer
    fn out_owner(&self, index: usize) -> bool {
        let mask = 1 << index;
        (self.out_owner.read().buf0_owner().bits() & mask) == 0
    }

    /// Sets the owner to hardware
    fn set_out_owner(&self, index: usize) {
        let mask = 1 << index;
        self.out_owner
            .modify(|r, w| w.buf0_owner().variant(r.buf0_owner().bits() | mask));
    }

    fn make_endpoint(&mut self, index: usize) {
        self.endpoints[index] = match index {
            0 => Some(unsafe { Endpoint::new(&self.usb.ep0) }),
            1 => Some(unsafe { Endpoint::new(&self.usb.ep1) }),
            2 => Some(unsafe { Endpoint::new(&self.usb.ep2) }),
            3 => Some(unsafe { Endpoint::new(&self.usb.ep3) }),
            4 => Some(unsafe { Endpoint::new(&self.usb.ep4) }),
            5 => Some(unsafe { Endpoint::new(&self.usb.ep5) }),
            6 => Some(unsafe { Endpoint::new(&self.usb.ep6) }),
            7 => Some(unsafe { Endpoint::new(&self.usb.ep7) }),
            _ => unreachable!("Invalid Endpoint Index {index}"),
        };
    }

    fn init_descriptor(&mut self, index: usize) {
        if index == 0 {
            // Set up both of EP0's buffer descriptors to the proper pointers and max sizes
            self.descriptors.ep0_out.buf0_pointer =
                &self.buffers[0] as *const [u8] as *const () as u32;
            self.descriptors.ep0_out.buf0_desc = MAX_PACKET_SIZE as u32;
            //
            self.descriptors.ep0_in.buf0_pointer =
                &self.buffers[1] as *const [u8] as *const () as u32;
            self.descriptors.ep0_in.buf0_desc = MAX_PACKET_SIZE as u32;
        } else {
            // Set up the pointer
            self.descriptors.eps[index - 1].buf0_pointer =
                &self.buffers[index + 1] as *const [u8] as *const () as u32;
            // Set up the description
            self.descriptors.eps[index - 1].buf0_desc = MAX_PACKET_SIZE as u32;
        }
    }

    fn set_up_hardware(usb: &USB, pwrman: &PWRMAN, clkman: &CLKMAN) {
        // Enable USB Power and Clock
        pwrman.pwr_rst_ctrl.modify(|_, w| w.usb_powered().set_bit());
        clkman
            .clk_ctrl
            .modify(|_, w| w.usb_clock_enable().set_bit());
        // Enable Hardware USB Controller
        usb.cn.write(|w| w.usb_en().clear_bit());
        usb.cn.write(|w| w.usb_en().set_bit());
        // Disable all interrupts
        usb.dev_inten.reset();
        // Reset Hardware
        usb.dev_cn.write(|w| w.urst().set_bit());
        usb.dev_cn
            .write(|w| w.urst().clear_bit().fifo_mode().set_bit());
    }

    fn calibrate_usb_clock(pwrseq: &PWRSEQ, adc: &ADC) {
        // Enable 32kHz RTC (if not already enabled)
        let mut enabled = false;
        pwrseq.reg0.modify(|r, w| {
            enabled = r.pwr_rtcen_run().bit();
            w.pwr_rtcen_run().set_bit()
        });
        // Enable the RO Completion interrupt
        adc.intr.modify(|_, w| w.ro_cal_done_ie().set_bit());
        // Clear RO Calibration complete interrupt
        adc.intr
            .modify(|_, w| w.ro_cal_done_if().clear_bit_by_one());
        // Read Relaxation Oscillator Flash Trim shadow register
        let init_trim = pwrseq.reg6.read().pwr_trim_osc_vref().bits();
        // Write InitTrim to freq. cal. initial condition register
        adc.ro_cal1.modify(|_, w| w.trm_init().variant(init_trim));
        // Load Initial Trim to active frequency trim register
        adc.ro_cal0.modify(|_, w| w.ro_cal_load().set_bit());
        // Enable frequency loop to control RO trim
        adc.ro_cal0.modify(|_, w| w.ro_cal_en().set_bit());
        // Run Frequency Calibration in Atomic mode
        adc.ro_cal0.modify(|_, w| w.ro_cal_atomic().set_bit());
        // Detect interrupt flag
        while !adc.intr.read().ro_cal_done_if().bit() {}
        // Stop Frequency Calibration
        adc.ro_cal0.modify(|_, w| w.ro_cal_run().clear_bit());
        // Read the final frequency trim value
        let final_trim = adc.ro_cal0.read().ro_trm().bits();
        // Write Final trim to Sys RO Flash Trim shadow register
        pwrseq
            .reg6
            .modify(|_, w| w.pwr_trim_osc_vref().variant(final_trim));
        // Set RTC Enable to previous state
        if !enabled {
            pwrseq.reg0.modify(|_, w| w.pwr_rtcen_run().clear_bit());
        }
    }
}

impl Deref for Usb {
    type Target = USB;

    fn deref(&self) -> &Self::Target {
        &self.usb
    }
}

impl DerefMut for Usb {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.usb
    }
}
