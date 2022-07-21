#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Slave Clock Divisor Control"]
    pub clk_div: crate::Reg<clk_div::CLK_DIV_SPEC>,
    #[doc = "0x04 - I2C Slave Device ID Register"]
    pub dev_id: crate::Reg<dev_id::DEV_ID_SPEC>,
    #[doc = "0x08 - I2CS Interrupt Flags"]
    pub intfl: crate::Reg<intfl::INTFL_SPEC>,
    #[doc = "0x0c - I2CS Interrupt Enable/Disable Controls"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x10 - I2CS Data Byte"]
    pub data_byte: crate::Reg<data_byte::DATA_BYTE_SPEC>,
}
#[doc = "CLK_DIV register accessor: an alias for `Reg<CLK_DIV_SPEC>`"]
pub type CLK_DIV = crate::Reg<clk_div::CLK_DIV_SPEC>;
#[doc = "I2C Slave Clock Divisor Control"]
pub mod clk_div;
#[doc = "DEV_ID register accessor: an alias for `Reg<DEV_ID_SPEC>`"]
pub type DEV_ID = crate::Reg<dev_id::DEV_ID_SPEC>;
#[doc = "I2C Slave Device ID Register"]
pub mod dev_id;
#[doc = "INTFL register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "I2CS Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "I2CS Interrupt Enable/Disable Controls"]
pub mod inten;
#[doc = "DATA_BYTE register accessor: an alias for `Reg<DATA_BYTE_SPEC>`"]
pub type DATA_BYTE = crate::Reg<data_byte::DATA_BYTE_SPEC>;
#[doc = "I2CS Data Byte"]
pub mod data_byte;
