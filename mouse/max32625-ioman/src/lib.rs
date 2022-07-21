#![no_std]
use max32625::IOMAN as IOMAN_RAW;

#[derive(Copy, Clone, Debug)]
pub enum IomanReq {
    WudReq0 = 0x4000_0C00,
    WudReq1 = 0x4000_0C04,
    AliReq0 = 0x4000_0C10,
    AliReq1 = 0x4000_0C14,
    SpixReq = 0x4000_0C28,
    Uart0Req = 0x4000_0C30,
    Uart1Req = 0x4000_0C38,
    Uart2Req = 0x4000_0C40,
    Uart3Req = 0x4000_0C48,
    I2cm0Req = 0x4000_0C50,
    I2cm1Req = 0x4000_0C58,
    I2cm2Req = 0x4000_0C60,
    I2csReq = 0x4000_0C68,
    Spim0Req = 0x4000_0C70,
    Spim1Req = 0x4000_0C78,
    Spim2Req = 0x4000_0C80,
    SpibReq = 0x4000_0C88,
    OwmReq = 0x4000_0C90,
    SpisReq = 0x4000_0C98,
}

impl IomanReq {
    fn get_ack_ptr(&self) -> *const u32 {
        match self {
            Self::WudReq0 => 0x4000_0C08 as _,
            Self::WudReq1 => 0x4000_0C0C as _,
            Self::AliReq0 => 0x4000_0C18 as _,
            Self::AliReq1 => 0x4000_0C1C as _,
            Self::SpixReq => 0x4000_0C2C as _,
            Self::Uart0Req => 0x4000_0C34 as _,
            Self::Uart1Req => 0x4000_0C3C as _,
            Self::Uart2Req => 0x4000_0C44 as _,
            Self::Uart3Req => 0x4000_0C4C as _,
            Self::I2cm0Req => 0x4000_0C54 as _,
            Self::I2cm1Req => 0x4000_0C5C as _,
            Self::I2cm2Req => 0x4000_0C64 as _,
            Self::I2csReq => 0x4000_0C6C as _,
            Self::Spim0Req => 0x4000_0C74 as _,
            Self::Spim1Req => 0x4000_0C7C as _,
            Self::Spim2Req => 0x4000_0C84 as _,
            Self::SpibReq => 0x4000_0C8C as _,
            Self::OwmReq => 0x4000_0C94 as _,
            Self::SpisReq => 0x4000_0C9C as _,
        }
    }
}

pub struct IOMAN {
    _ioman: IOMAN_RAW,
}

impl IOMAN {
    pub fn new(ioman: IOMAN_RAW) -> Self {
        Self { _ioman: ioman }
    }

    pub fn request_mode(&mut self, request: IomanReq) -> bool {
        // Request all
        let value = 0xFFFF_FFFF;
        unsafe { core::ptr::write_volatile(request as u32 as *mut u32, value) };
        // Check ack
        unsafe {
            core::ptr::read_volatile(request.get_ack_ptr())
                == core::ptr::read_volatile(request as u32 as *const u32)
        }
    }
}
