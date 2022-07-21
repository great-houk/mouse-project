#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pulse Train Configuration"]
    pub rate_length: crate::Reg<rate_length::RATE_LENGTH_SPEC>,
    #[doc = "0x04 - Pulse Train Output Pattern"]
    pub train: crate::Reg<train::TRAIN_SPEC>,
    #[doc = "0x08 - Pulse Train Loop Count"]
    pub loop_: crate::Reg<loop_::LOOP_SPEC>,
}
#[doc = "RATE_LENGTH register accessor: an alias for `Reg<RATE_LENGTH_SPEC>`"]
pub type RATE_LENGTH = crate::Reg<rate_length::RATE_LENGTH_SPEC>;
#[doc = "Pulse Train Configuration"]
pub mod rate_length;
#[doc = "TRAIN register accessor: an alias for `Reg<TRAIN_SPEC>`"]
pub type TRAIN = crate::Reg<train::TRAIN_SPEC>;
#[doc = "Pulse Train Output Pattern"]
pub mod train;
#[doc = "LOOP register accessor: an alias for `Reg<LOOP_SPEC>`"]
pub type LOOP = crate::Reg<loop_::LOOP_SPEC>;
#[doc = "Pulse Train Loop Count"]
pub mod loop_;
