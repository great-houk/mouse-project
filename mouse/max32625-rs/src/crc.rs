#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC-16/CRC-32 Reseed Controls"]
    pub reseed: crate::Reg<reseed::RESEED_SPEC>,
    #[doc = "0x04 - Reseed Value for CRC-16 Calculations"]
    pub seed16: crate::Reg<seed16::SEED16_SPEC>,
    #[doc = "0x08 - Reseed Value for CRC-32 Calculations"]
    pub seed32: crate::Reg<seed32::SEED32_SPEC>,
}
#[doc = "RESEED register accessor: an alias for `Reg<RESEED_SPEC>`"]
pub type RESEED = crate::Reg<reseed::RESEED_SPEC>;
#[doc = "CRC-16/CRC-32 Reseed Controls"]
pub mod reseed;
#[doc = "SEED16 register accessor: an alias for `Reg<SEED16_SPEC>`"]
pub type SEED16 = crate::Reg<seed16::SEED16_SPEC>;
#[doc = "Reseed Value for CRC-16 Calculations"]
pub mod seed16;
#[doc = "SEED32 register accessor: an alias for `Reg<SEED32_SPEC>`"]
pub type SEED32 = crate::Reg<seed32::SEED32_SPEC>;
#[doc = "Reseed Value for CRC-32 Calculations"]
pub mod seed32;
