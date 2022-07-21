#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES Control and Status"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - A write to this location triggers an erase of all AES memory locations."]
    pub erase_all: crate::Reg<erase_all::ERASE_ALL_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "AES Control and Status"]
pub mod ctrl;
#[doc = "ERASE_ALL register accessor: an alias for `Reg<ERASE_ALL_SPEC>`"]
pub type ERASE_ALL = crate::Reg<erase_all::ERASE_ALL_SPEC>;
#[doc = "A write to this location triggers an erase of all AES memory locations."]
pub mod erase_all;
