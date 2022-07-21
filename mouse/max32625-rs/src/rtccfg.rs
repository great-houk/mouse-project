#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Nano Oscillator Counter Read Register"]
    pub nano_cntr: crate::Reg<nano_cntr::NANO_CNTR_SPEC>,
    #[doc = "0x04 - RTC Clock Control Settings"]
    pub clk_ctrl: crate::Reg<clk_ctrl::CLK_CTRL_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - RTC Oscillator Control"]
    pub osc_ctrl: crate::Reg<osc_ctrl::OSC_CTRL_SPEC>,
}
#[doc = "NANO_CNTR register accessor: an alias for `Reg<NANO_CNTR_SPEC>`"]
pub type NANO_CNTR = crate::Reg<nano_cntr::NANO_CNTR_SPEC>;
#[doc = "Nano Oscillator Counter Read Register"]
pub mod nano_cntr;
#[doc = "CLK_CTRL register accessor: an alias for `Reg<CLK_CTRL_SPEC>`"]
pub type CLK_CTRL = crate::Reg<clk_ctrl::CLK_CTRL_SPEC>;
#[doc = "RTC Clock Control Settings"]
pub mod clk_ctrl;
#[doc = "OSC_CTRL register accessor: an alias for `Reg<OSC_CTRL_SPEC>`"]
pub type OSC_CTRL = crate::Reg<osc_ctrl::OSC_CTRL_SPEC>;
#[doc = "RTC Oscillator Control"]
pub mod osc_ctrl;
