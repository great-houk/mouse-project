use core::marker::PhantomData;
use max32625::{
    generic::{Reg, RegisterSpec},
    usb::{
        ep0::EP0_SPEC, ep1::EP1_SPEC, ep2::EP2_SPEC, ep3::EP3_SPEC, ep4::EP4_SPEC, ep5::EP5_SPEC,
        ep6::EP6_SPEC, ep7::EP7_SPEC,
    },
};

macro_rules! reg {
        ($name:ident {
            reset_value: u32,
            $($field_name:ident: $field_type:ty,)*
        }) => {
            mod $name {
                use super::*;

                #[derive(Debug, PartialEq, Clone)]
                pub struct Register {
                    pub reset_value: u32,
                    $(pub $field_name: RegisterField<$field_type>,)*
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct Reader {
                    bits: u32,
                    register: Register,
                }

                impl Reader {
                    pub fn new(bits: u32, register: &Register) -> Self {
                        Self {
                            bits, register: register.clone()
                        }
                    }

                    pub fn bits(&self) -> u32 {
                        self.bits
                    }

                    $(
                        #[allow(dead_code)]
                        pub fn $field_name(&self) -> $field_type {
                            self.register.$field_name.get_value(self.bits).unwrap()
                        }
                    )*
                }
                #[derive(Debug, PartialEq, Clone)]
                pub struct Writer  {
                    bits: u32,
                    register: Register,
                }

                impl Writer {
                    pub fn new(bits: u32, register: &Register) -> Self {
                        Self {
                            bits, register: register.clone()
                        }
                    }

                    pub fn bits(&mut self) -> u32 {
                        self.bits
                    }

                    $(
                        #[allow(dead_code)]
                        pub fn $field_name(&mut self, value: $field_type) -> &mut Self {
                            self.bits |= self.register.$field_name.format(value);
                            self
                        }
                    )*
                }
            }
        }
    }

pub trait EP: RegisterSpec<Ux = u32> {}
impl EP for EP0_SPEC {}
impl EP for EP1_SPEC {}
impl EP for EP2_SPEC {}
impl EP for EP3_SPEC {}
impl EP for EP4_SPEC {}
impl EP for EP5_SPEC {}
impl EP for EP6_SPEC {}
impl EP for EP7_SPEC {}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum EpDir {
    Disabled = 0b00,
    Out = 0b01,
    In = 0b10,
    Control = 0b11,
}
impl From<EpDir> for u32 {
    fn from(val: EpDir) -> Self {
        val as u32
    }
}
impl TryFrom<u32> for EpDir {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(EpDir::Disabled),
            0b01 => Ok(EpDir::Out),
            0b10 => Ok(EpDir::In),
            0b11 => Ok(EpDir::Control),
            _ => Err(value),
        }
    }
}
#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Enabled {
    True = 1,
    False = 0,
}
impl From<Enabled> for u32 {
    fn from(val: Enabled) -> Self {
        val as u32
    }
}
impl TryFrom<u32> for Enabled {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Enabled::False),
            1 => Ok(Enabled::True),
            _ => Err(value),
        }
    }
}
impl From<bool> for Enabled {
    fn from(value: bool) -> Self {
        if value {
            Enabled::True
        } else {
            Enabled::False
        }
    }
}
impl From<Enabled> for bool {
    fn from(value: Enabled) -> Self {
        value == Enabled::True
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegisterField<R: Into<u32> + TryFrom<u32>> {
    start: u8,
    len: u8,
    repr: PhantomData<R>,
}
impl<R: Into<u32> + TryFrom<u32>> RegisterField<R> {
    pub fn new(start: u8, len: u8) -> Self {
        Self {
            start,
            len,
            repr: PhantomData,
        }
    }

    pub fn format(&self, value: R) -> u32 {
        let mask = (1 << self.len) - 1;
        (value.into() & mask) << self.start
    }

    pub fn get_value(&self, register: u32) -> Result<R, R::Error> {
        let mask = (1 << self.len) - 1;
        ((register >> self.start) & mask).try_into()
    }
}

use endpoint::{Reader as EndpointReader, Register as EndpointRegister, Writer as EndpointWriter};
reg! {endpoint {
    reset_value: u32,
    dir: EpDir,
    buf2: Enabled,
    int_enable: Enabled,
    nak_enable: Enabled,
    dt: Enabled,
    stall: Enabled,
    st_stall: Enabled,
    st_ack: Enabled,
}}

#[derive(Debug, PartialEq, Clone)]
pub struct Endpoint {
    ptr: *mut u32,
    register: EndpointRegister,
}

impl Endpoint {
    /// SAFETY: Ensure that no other Endpoints are made with the same endpoint
    pub unsafe fn new(ep: &Reg<impl EP>) -> Self {
        let ptr = ep.as_ptr();
        let register = EndpointRegister {
            reset_value: 0b000000000000000000000__0_0_0__0__0_0_0_0__0__00__,
            dir: RegisterField::new(0, 2),
            buf2: RegisterField::new(3, 1),
            int_enable: RegisterField::new(4, 1),
            nak_enable: RegisterField::new(5, 1),
            dt: RegisterField::new(6, 1),
            stall: RegisterField::new(8, 1),
            st_stall: RegisterField::new(9, 1),
            st_ack: RegisterField::new(10, 1),
        };
        Self { ptr, register }
    }

    pub fn read(&self) -> EndpointReader {
        // SAFETY: This pointer is unique and valid
        let bits = unsafe { core::ptr::read_volatile(self.ptr) };
        EndpointReader::new(bits, &self.register)
    }

    pub fn write<'w, 's>(&mut self, f: impl Fn(&mut EndpointWriter) -> &mut EndpointWriter) {
        let bits = self.register.reset_value;
        let mut writer = EndpointWriter::new(bits, &self.register);
        let new_bits = f(&mut writer).bits();
        // SAFETY: This pointer is unique and valid
        unsafe { core::ptr::write_volatile(self.ptr, new_bits) };
    }

    pub fn modify(
        &mut self,
        f: impl Fn(EndpointReader, &mut EndpointWriter) -> &mut EndpointWriter,
    ) {
        let reader = self.read();
        let bits = reader.bits();
        let mut writer = EndpointWriter::new(bits, &self.register);
        let new_bits = f(reader, &mut writer).bits();
        // SAFETY: This pointer is unique and valid
        unsafe { core::ptr::write_volatile(self.ptr, new_bits) };
    }
}
