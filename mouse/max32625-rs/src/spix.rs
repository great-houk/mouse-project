#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPIX Master Configuration"]
    pub master_cfg: crate::Reg<master_cfg::MASTER_CFG_SPEC>,
    #[doc = "0x04 - SPIX Fetch Control"]
    pub fetch_ctrl: crate::Reg<fetch_ctrl::FETCH_CTRL_SPEC>,
    #[doc = "0x08 - SPIX Mode Control"]
    pub mode_ctrl: crate::Reg<mode_ctrl::MODE_CTRL_SPEC>,
    #[doc = "0x0c - SPIX Mode Data"]
    pub mode_data: crate::Reg<mode_data::MODE_DATA_SPEC>,
}
#[doc = "MASTER_CFG register accessor: an alias for `Reg<MASTER_CFG_SPEC>`"]
pub type MASTER_CFG = crate::Reg<master_cfg::MASTER_CFG_SPEC>;
#[doc = "SPIX Master Configuration"]
pub mod master_cfg;
#[doc = "FETCH_CTRL register accessor: an alias for `Reg<FETCH_CTRL_SPEC>`"]
pub type FETCH_CTRL = crate::Reg<fetch_ctrl::FETCH_CTRL_SPEC>;
#[doc = "SPIX Fetch Control"]
pub mod fetch_ctrl;
#[doc = "MODE_CTRL register accessor: an alias for `Reg<MODE_CTRL_SPEC>`"]
pub type MODE_CTRL = crate::Reg<mode_ctrl::MODE_CTRL_SPEC>;
#[doc = "SPIX Mode Control"]
pub mod mode_ctrl;
#[doc = "MODE_DATA register accessor: an alias for `Reg<MODE_DATA_SPEC>`"]
pub type MODE_DATA = crate::Reg<mode_data::MODE_DATA_SPEC>;
#[doc = "SPIX Mode Data"]
pub mod mode_data;
