#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAA Control, Configuration and Status"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - MAA Word (Operand) Size, Big/Little Endian Mode Select"]
    pub maws: crate::Reg<maws::MAWS_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "MAA Control, Configuration and Status"]
pub mod ctrl;
#[doc = "MAWS register accessor: an alias for `Reg<MAWS_SPEC>`"]
pub type MAWS = crate::Reg<maws::MAWS_SPEC>;
#[doc = "MAA Word (Operand) Size, Big/Little Endian Mode Select"]
pub mod maws;
