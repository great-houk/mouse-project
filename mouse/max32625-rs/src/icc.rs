#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device ID Register"]
    pub id: crate::Reg<id::ID_SPEC>,
    #[doc = "0x04 - Memory Configuration Register"]
    pub mem_cfg: crate::Reg<mem_cfg::MEM_CFG_SPEC>,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - Control and Status"]
    pub ctrl_stat: crate::Reg<ctrl_stat::CTRL_STAT_SPEC>,
    _reserved3: [u8; 0x05fc],
    #[doc = "0x700 - Invalidate (Clear) Cache Control"]
    pub invdt_all: crate::Reg<invdt_all::INVDT_ALL_SPEC>,
}
#[doc = "ID register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Device ID Register"]
pub mod id;
#[doc = "MEM_CFG register accessor: an alias for `Reg<MEM_CFG_SPEC>`"]
pub type MEM_CFG = crate::Reg<mem_cfg::MEM_CFG_SPEC>;
#[doc = "Memory Configuration Register"]
pub mod mem_cfg;
#[doc = "CTRL_STAT register accessor: an alias for `Reg<CTRL_STAT_SPEC>`"]
pub type CTRL_STAT = crate::Reg<ctrl_stat::CTRL_STAT_SPEC>;
#[doc = "Control and Status"]
pub mod ctrl_stat;
#[doc = "INVDT_ALL register accessor: an alias for `Reg<INVDT_ALL_SPEC>`"]
pub type INVDT_ALL = crate::Reg<invdt_all::INVDT_ALL_SPEC>;
#[doc = "Invalidate (Clear) Cache Control"]
pub mod invdt_all;
