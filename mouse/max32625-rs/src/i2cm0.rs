#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Full Speed SCL Clock Settings"]
    pub fs_clk_div: crate::Reg<fs_clk_div::FS_CLK_DIV_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x0c - Timeout and Auto-Stop Settings"]
    pub timeout: crate::Reg<timeout::TIMEOUT_SPEC>,
    #[doc = "0x10 - I2C Master Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x14 - I2C Master Transaction Start and Status Flags"]
    pub trans: crate::Reg<trans::TRANS_SPEC>,
    #[doc = "0x18 - Interrupt Flags"]
    pub intfl: crate::Reg<intfl::INTFL_SPEC>,
    #[doc = "0x1c - Interrupt Enable/Disable Controls"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x28 - Bit-Bang Control Register"]
    pub bb: crate::Reg<bb::BB_SPEC>,
}
#[doc = "FS_CLK_DIV register accessor: an alias for `Reg<FS_CLK_DIV_SPEC>`"]
pub type FS_CLK_DIV = crate::Reg<fs_clk_div::FS_CLK_DIV_SPEC>;
#[doc = "Full Speed SCL Clock Settings"]
pub mod fs_clk_div;
#[doc = "TIMEOUT register accessor: an alias for `Reg<TIMEOUT_SPEC>`"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "Timeout and Auto-Stop Settings"]
pub mod timeout;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "I2C Master Control Register"]
pub mod ctrl;
#[doc = "TRANS register accessor: an alias for `Reg<TRANS_SPEC>`"]
pub type TRANS = crate::Reg<trans::TRANS_SPEC>;
#[doc = "I2C Master Transaction Start and Status Flags"]
pub mod trans;
#[doc = "INTFL register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable/Disable Controls"]
pub mod inten;
#[doc = "BB register accessor: an alias for `Reg<BB_SPEC>`"]
pub type BB = crate::Reg<bb::BB_SPEC>;
#[doc = "Bit-Bang Control Register"]
pub mod bb;
