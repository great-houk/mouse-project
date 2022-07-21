#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - UART Baud Control Register"]
    pub baud: crate::Reg<baud::BAUD_SPEC>,
    #[doc = "0x08 - UART TX Fifo Control Register"]
    pub tx_fifo_ctrl: crate::Reg<tx_fifo_ctrl::TX_FIFO_CTRL_SPEC>,
    #[doc = "0x0c - UART RX Fifo Control Register"]
    pub rx_fifo_ctrl: crate::Reg<rx_fifo_ctrl::RX_FIFO_CTRL_SPEC>,
    #[doc = "0x10 - UART Multidrop Control Register"]
    pub md_ctrl: crate::Reg<md_ctrl::MD_CTRL_SPEC>,
    #[doc = "0x14 - UART Interrupt Flags"]
    pub intfl: crate::Reg<intfl::INTFL_SPEC>,
    #[doc = "0x18 - UART Interrupt Enable/Disable Controls"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "UART Control Register"]
pub mod ctrl;
#[doc = "BAUD register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "UART Baud Control Register"]
pub mod baud;
#[doc = "TX_FIFO_CTRL register accessor: an alias for `Reg<TX_FIFO_CTRL_SPEC>`"]
pub type TX_FIFO_CTRL = crate::Reg<tx_fifo_ctrl::TX_FIFO_CTRL_SPEC>;
#[doc = "UART TX Fifo Control Register"]
pub mod tx_fifo_ctrl;
#[doc = "RX_FIFO_CTRL register accessor: an alias for `Reg<RX_FIFO_CTRL_SPEC>`"]
pub type RX_FIFO_CTRL = crate::Reg<rx_fifo_ctrl::RX_FIFO_CTRL_SPEC>;
#[doc = "UART RX Fifo Control Register"]
pub mod rx_fifo_ctrl;
#[doc = "MD_CTRL register accessor: an alias for `Reg<MD_CTRL_SPEC>`"]
pub type MD_CTRL = crate::Reg<md_ctrl::MD_CTRL_SPEC>;
#[doc = "UART Multidrop Control Register"]
pub mod md_ctrl;
#[doc = "INTFL register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "UART Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "UART Interrupt Enable/Disable Controls"]
pub mod inten;
