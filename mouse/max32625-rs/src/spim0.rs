#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Master Configuration Register"]
    pub mstr_cfg: crate::Reg<mstr_cfg::MSTR_CFG_SPEC>,
    #[doc = "0x04 - Polarity Control for SS and SR Signals"]
    pub ss_sr_polarity: crate::Reg<ss_sr_polarity::SS_SR_POLARITY_SPEC>,
    #[doc = "0x08 - SPI Master General Control Register"]
    pub gen_ctrl: crate::Reg<gen_ctrl::GEN_CTRL_SPEC>,
    #[doc = "0x0c - SPI Master FIFO Control Register"]
    pub fifo_ctrl: crate::Reg<fifo_ctrl::FIFO_CTRL_SPEC>,
    #[doc = "0x10 - SPI Master Special Mode Controls"]
    pub spcl_ctrl: crate::Reg<spcl_ctrl::SPCL_CTRL_SPEC>,
    #[doc = "0x14 - SPI Master Interrupt Flags"]
    pub intfl: crate::Reg<intfl::INTFL_SPEC>,
    #[doc = "0x18 - SPI Master Interrupt Enable/Disable Settings"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
}
#[doc = "MSTR_CFG register accessor: an alias for `Reg<MSTR_CFG_SPEC>`"]
pub type MSTR_CFG = crate::Reg<mstr_cfg::MSTR_CFG_SPEC>;
#[doc = "SPI Master Configuration Register"]
pub mod mstr_cfg;
#[doc = "SS_SR_POLARITY register accessor: an alias for `Reg<SS_SR_POLARITY_SPEC>`"]
pub type SS_SR_POLARITY = crate::Reg<ss_sr_polarity::SS_SR_POLARITY_SPEC>;
#[doc = "Polarity Control for SS and SR Signals"]
pub mod ss_sr_polarity;
#[doc = "GEN_CTRL register accessor: an alias for `Reg<GEN_CTRL_SPEC>`"]
pub type GEN_CTRL = crate::Reg<gen_ctrl::GEN_CTRL_SPEC>;
#[doc = "SPI Master General Control Register"]
pub mod gen_ctrl;
#[doc = "FIFO_CTRL register accessor: an alias for `Reg<FIFO_CTRL_SPEC>`"]
pub type FIFO_CTRL = crate::Reg<fifo_ctrl::FIFO_CTRL_SPEC>;
#[doc = "SPI Master FIFO Control Register"]
pub mod fifo_ctrl;
#[doc = "SPCL_CTRL register accessor: an alias for `Reg<SPCL_CTRL_SPEC>`"]
pub type SPCL_CTRL = crate::Reg<spcl_ctrl::SPCL_CTRL_SPEC>;
#[doc = "SPI Master Special Mode Controls"]
pub mod spcl_ctrl;
#[doc = "INTFL register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "SPI Master Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "SPI Master Interrupt Enable/Disable Settings"]
pub mod inten;
