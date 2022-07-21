#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Enable/Disable Controls for All Pulse Trains"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x04 - Global Resync (All Pulse Trains) Control"]
    pub resync: crate::Reg<resync::RESYNC_SPEC>,
    #[doc = "0x08 - Pulse Train Interrupt Flags"]
    pub intfl: crate::Reg<intfl::INTFL_SPEC>,
    #[doc = "0x0c - Pulse Train Interrupt Enable/Disable"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
}
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Global Enable/Disable Controls for All Pulse Trains"]
pub mod enable;
#[doc = "RESYNC register accessor: an alias for `Reg<RESYNC_SPEC>`"]
pub type RESYNC = crate::Reg<resync::RESYNC_SPEC>;
#[doc = "Global Resync (All Pulse Trains) Control"]
pub mod resync;
#[doc = "INTFL register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Pulse Train Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Pulse Train Interrupt Enable/Disable"]
pub mod inten;
