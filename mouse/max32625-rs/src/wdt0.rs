#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT0 - Watchdog Timer Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - WDT0 - Watchdog Clear Register (Feed Dog)"]
    pub clear: crate::Reg<clear::CLEAR_SPEC>,
    #[doc = "0x08 - WDT0 - Watchdog Interrupt and Reset Flags"]
    pub flags: crate::Reg<flags::FLAGS_SPEC>,
    #[doc = "0x0c - WDT0 - Interrupt/Reset Enable/Disable Controls"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - WDT0 - Register Setting Lock for WDT0_CTRL"]
    pub lock_ctrl: crate::Reg<lock_ctrl::LOCK_CTRL_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "WDT0 - Watchdog Timer Control Register"]
pub mod ctrl;
#[doc = "CLEAR register accessor: an alias for `Reg<CLEAR_SPEC>`"]
pub type CLEAR = crate::Reg<clear::CLEAR_SPEC>;
#[doc = "WDT0 - Watchdog Clear Register (Feed Dog)"]
pub mod clear;
#[doc = "FLAGS register accessor: an alias for `Reg<FLAGS_SPEC>`"]
pub type FLAGS = crate::Reg<flags::FLAGS_SPEC>;
#[doc = "WDT0 - Watchdog Interrupt and Reset Flags"]
pub mod flags;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "WDT0 - Interrupt/Reset Enable/Disable Controls"]
pub mod enable;
#[doc = "LOCK_CTRL register accessor: an alias for `Reg<LOCK_CTRL_SPEC>`"]
pub type LOCK_CTRL = crate::Reg<lock_ctrl::LOCK_CTRL_SPEC>;
#[doc = "WDT0 - Register Setting Lock for WDT0_CTRL"]
pub mod lock_ctrl;
