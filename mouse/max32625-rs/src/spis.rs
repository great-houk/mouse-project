#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Slave General Control Register"]
    pub gen_ctrl: crate::Reg<gen_ctrl::GEN_CTRL_SPEC>,
    #[doc = "0x04 - SPI Master FIFO Control Register"]
    pub fifo_ctrl: crate::Reg<fifo_ctrl::FIFO_CTRL_SPEC>,
    #[doc = "0x08 - SPI Slave FIFO Status Information"]
    pub fifo_stat: crate::Reg<fifo_stat::FIFO_STAT_SPEC>,
    #[doc = "0x0c - SPI Slave Interrupt Flags"]
    pub intfl: crate::Reg<intfl::INTFL_SPEC>,
    #[doc = "0x10 - SPI Slave Interrupt Enable/Disable Settings"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
}
#[doc = "GEN_CTRL register accessor: an alias for `Reg<GEN_CTRL_SPEC>`"]
pub type GEN_CTRL = crate::Reg<gen_ctrl::GEN_CTRL_SPEC>;
#[doc = "SPI Slave General Control Register"]
pub mod gen_ctrl;
#[doc = "FIFO_CTRL register accessor: an alias for `Reg<FIFO_CTRL_SPEC>`"]
pub type FIFO_CTRL = crate::Reg<fifo_ctrl::FIFO_CTRL_SPEC>;
#[doc = "SPI Master FIFO Control Register"]
pub mod fifo_ctrl;
#[doc = "FIFO_STAT register accessor: an alias for `Reg<FIFO_STAT_SPEC>`"]
pub type FIFO_STAT = crate::Reg<fifo_stat::FIFO_STAT_SPEC>;
#[doc = "SPI Slave FIFO Status Information"]
pub mod fifo_stat;
#[doc = "INTFL register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "SPI Slave Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "SPI Slave Interrupt Enable/Disable Settings"]
pub mod inten;
