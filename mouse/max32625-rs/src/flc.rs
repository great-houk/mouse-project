#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Operation Address"]
    pub faddr: crate::Reg<faddr::FADDR_SPEC>,
    #[doc = "0x04 - Flash Clock Pulse Divisor"]
    pub fckdiv: crate::Reg<fckdiv::FCKDIV_SPEC>,
    #[doc = "0x08 - Flash Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    _reserved3: [u8; 0x18],
    #[doc = "0x24 - Flash Controller Interrupt Flags and Enable/Disable 0"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x30 - Flash Operation Data Register"]
    pub fdata: crate::Reg<fdata::FDATA_SPEC>,
    _reserved5: [u8; 0x1c],
    #[doc = "0x50 - Flash Performance Settings"]
    pub perform: crate::Reg<perform::PERFORM_SPEC>,
    #[doc = "0x54 - Flash Read Cycle Config"]
    pub tacc: crate::Reg<tacc::TACC_SPEC>,
    #[doc = "0x58 - Flash Write Cycle Config"]
    pub tprog: crate::Reg<tprog::TPROG_SPEC>,
    _reserved8: [u8; 0x24],
    #[doc = "0x80 - Security Status Flags"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x88 - Flash Controller Security Settings"]
    pub security: crate::Reg<security::SECURITY_SPEC>,
    _reserved10: [u8; 0x10],
    #[doc = "0x9c - Status Flags for DSB Operations"]
    pub bypass: crate::Reg<bypass::BYPASS_SPEC>,
    _reserved11: [u8; 0x60],
    #[doc = "0x100 - Used to set DSB Access code and Auto-Lock in info block"]
    pub user_option: crate::Reg<user_option::USER_OPTION_SPEC>,
    _reserved12: [u8; 0x3c],
    #[doc = "0x140 - Flash Control Register 2"]
    pub ctrl2: crate::Reg<ctrl2::CTRL2_SPEC>,
    #[doc = "0x144 - Interrupt Flags Register 1"]
    pub intfl1: crate::Reg<intfl1::INTFL1_SPEC>,
    #[doc = "0x148 - Interrupt Enable/Disable Register 1"]
    pub inten1: crate::Reg<inten1::INTEN1_SPEC>,
    _reserved15: [u8; 0x24],
    #[doc = "0x170 - Bootloader Control Register"]
    pub bl_ctrl: crate::Reg<bl_ctrl::BL_CTRL_SPEC>,
    #[doc = "0x174 - Cycle Count Tweak Register"]
    pub twk_cycl_cnt: crate::Reg<twk_cycl_cnt::TWK_CYCL_CNT_SPEC>,
    #[doc = "0x178 - PDM33 Register"]
    pub pdm33: crate::Reg<pdm33::PDM33_SPEC>,
    #[doc = "0x17c - Sleep Mode Register"]
    pub slm: crate::Reg<slm::SLM_SPEC>,
    _reserved19: [u8; 0x80],
    #[doc = "0x200 - Disable Flash Page Exec/Read Register 0"]
    pub disable_xr0: crate::Reg<disable_xr0::DISABLE_XR0_SPEC>,
    #[doc = "0x204 - Disable Flash Page Exec/Read Register 1"]
    pub disable_xr1: crate::Reg<disable_xr1::DISABLE_XR1_SPEC>,
    #[doc = "0x208 - Disable Flash Page Exec/Read Register 2"]
    pub disable_xr2: crate::Reg<disable_xr2::DISABLE_XR2_SPEC>,
    #[doc = "0x20c - Disable Flash Page Exec/Read Register 3"]
    pub disable_xr3: crate::Reg<disable_xr3::DISABLE_XR3_SPEC>,
    #[doc = "0x210 - Disable Flash Page Exec/Read Register 4"]
    pub disable_xr4: crate::Reg<disable_xr4::DISABLE_XR4_SPEC>,
    #[doc = "0x214 - Disable Flash Page Exec/Read Register 5"]
    pub disable_xr5: crate::Reg<disable_xr5::DISABLE_XR5_SPEC>,
    #[doc = "0x218 - Disable Flash Page Exec/Read Register 6"]
    pub disable_xr6: crate::Reg<disable_xr6::DISABLE_XR6_SPEC>,
    #[doc = "0x21c - Disable Flash Page Exec/Read Register 7"]
    pub disable_xr7: crate::Reg<disable_xr7::DISABLE_XR7_SPEC>,
    _reserved27: [u8; 0xe0],
    #[doc = "0x300 - Disable Flash Page Write/Erase Register 0"]
    pub disable_we0: crate::Reg<disable_we0::DISABLE_WE0_SPEC>,
    #[doc = "0x304 - Disable Flash Page Write/Erase Register 1"]
    pub disable_we1: crate::Reg<disable_we1::DISABLE_WE1_SPEC>,
    #[doc = "0x308 - Disable Flash Page Write/Erase Register 2"]
    pub disable_we2: crate::Reg<disable_we2::DISABLE_WE2_SPEC>,
    #[doc = "0x30c - Disable Flash Page Write/Erase Register 3"]
    pub disable_we3: crate::Reg<disable_we3::DISABLE_WE3_SPEC>,
    #[doc = "0x310 - Disable Flash Page Write/Erase Register 4"]
    pub disable_we4: crate::Reg<disable_we4::DISABLE_WE4_SPEC>,
    #[doc = "0x314 - Disable Flash Page Write/Erase Register 5"]
    pub disable_we5: crate::Reg<disable_we5::DISABLE_WE5_SPEC>,
    #[doc = "0x318 - Disable Flash Page Write/Erase Register 6"]
    pub disable_we6: crate::Reg<disable_we6::DISABLE_WE6_SPEC>,
    #[doc = "0x31c - Disable Flash Page Write/Erase Register 7"]
    pub disable_we7: crate::Reg<disable_we7::DISABLE_WE7_SPEC>,
}
#[doc = "FADDR register accessor: an alias for `Reg<FADDR_SPEC>`"]
pub type FADDR = crate::Reg<faddr::FADDR_SPEC>;
#[doc = "Flash Operation Address"]
pub mod faddr;
#[doc = "FCKDIV register accessor: an alias for `Reg<FCKDIV_SPEC>`"]
pub type FCKDIV = crate::Reg<fckdiv::FCKDIV_SPEC>;
#[doc = "Flash Clock Pulse Divisor"]
pub mod fckdiv;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Flash Control Register"]
pub mod ctrl;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Flash Controller Interrupt Flags and Enable/Disable 0"]
pub mod intr;
#[doc = "FDATA register accessor: an alias for `Reg<FDATA_SPEC>`"]
pub type FDATA = crate::Reg<fdata::FDATA_SPEC>;
#[doc = "Flash Operation Data Register"]
pub mod fdata;
#[doc = "PERFORM register accessor: an alias for `Reg<PERFORM_SPEC>`"]
pub type PERFORM = crate::Reg<perform::PERFORM_SPEC>;
#[doc = "Flash Performance Settings"]
pub mod perform;
#[doc = "TACC register accessor: an alias for `Reg<TACC_SPEC>`"]
pub type TACC = crate::Reg<tacc::TACC_SPEC>;
#[doc = "Flash Read Cycle Config"]
pub mod tacc;
#[doc = "TPROG register accessor: an alias for `Reg<TPROG_SPEC>`"]
pub type TPROG = crate::Reg<tprog::TPROG_SPEC>;
#[doc = "Flash Write Cycle Config"]
pub mod tprog;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Security Status Flags"]
pub mod status;
#[doc = "SECURITY register accessor: an alias for `Reg<SECURITY_SPEC>`"]
pub type SECURITY = crate::Reg<security::SECURITY_SPEC>;
#[doc = "Flash Controller Security Settings"]
pub mod security;
#[doc = "BYPASS register accessor: an alias for `Reg<BYPASS_SPEC>`"]
pub type BYPASS = crate::Reg<bypass::BYPASS_SPEC>;
#[doc = "Status Flags for DSB Operations"]
pub mod bypass;
#[doc = "USER_OPTION register accessor: an alias for `Reg<USER_OPTION_SPEC>`"]
pub type USER_OPTION = crate::Reg<user_option::USER_OPTION_SPEC>;
#[doc = "Used to set DSB Access code and Auto-Lock in info block"]
pub mod user_option;
#[doc = "CTRL2 register accessor: an alias for `Reg<CTRL2_SPEC>`"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Flash Control Register 2"]
pub mod ctrl2;
#[doc = "INTFL1 register accessor: an alias for `Reg<INTFL1_SPEC>`"]
pub type INTFL1 = crate::Reg<intfl1::INTFL1_SPEC>;
#[doc = "Interrupt Flags Register 1"]
pub mod intfl1;
#[doc = "INTEN1 register accessor: an alias for `Reg<INTEN1_SPEC>`"]
pub type INTEN1 = crate::Reg<inten1::INTEN1_SPEC>;
#[doc = "Interrupt Enable/Disable Register 1"]
pub mod inten1;
#[doc = "BL_CTRL register accessor: an alias for `Reg<BL_CTRL_SPEC>`"]
pub type BL_CTRL = crate::Reg<bl_ctrl::BL_CTRL_SPEC>;
#[doc = "Bootloader Control Register"]
pub mod bl_ctrl;
#[doc = "TWK_CYCL_CNT register accessor: an alias for `Reg<TWK_CYCL_CNT_SPEC>`"]
pub type TWK_CYCL_CNT = crate::Reg<twk_cycl_cnt::TWK_CYCL_CNT_SPEC>;
#[doc = "Cycle Count Tweak Register"]
pub mod twk_cycl_cnt;
#[doc = "PDM33 register accessor: an alias for `Reg<PDM33_SPEC>`"]
pub type PDM33 = crate::Reg<pdm33::PDM33_SPEC>;
#[doc = "PDM33 Register"]
pub mod pdm33;
#[doc = "SLM register accessor: an alias for `Reg<SLM_SPEC>`"]
pub type SLM = crate::Reg<slm::SLM_SPEC>;
#[doc = "Sleep Mode Register"]
pub mod slm;
#[doc = "DISABLE_XR0 register accessor: an alias for `Reg<DISABLE_XR0_SPEC>`"]
pub type DISABLE_XR0 = crate::Reg<disable_xr0::DISABLE_XR0_SPEC>;
#[doc = "Disable Flash Page Exec/Read Register 0"]
pub mod disable_xr0;
#[doc = "DISABLE_XR1 register accessor: an alias for `Reg<DISABLE_XR1_SPEC>`"]
pub type DISABLE_XR1 = crate::Reg<disable_xr1::DISABLE_XR1_SPEC>;
#[doc = "Disable Flash Page Exec/Read Register 1"]
pub mod disable_xr1;
#[doc = "DISABLE_XR2 register accessor: an alias for `Reg<DISABLE_XR2_SPEC>`"]
pub type DISABLE_XR2 = crate::Reg<disable_xr2::DISABLE_XR2_SPEC>;
#[doc = "Disable Flash Page Exec/Read Register 2"]
pub mod disable_xr2;
#[doc = "DISABLE_XR3 register accessor: an alias for `Reg<DISABLE_XR3_SPEC>`"]
pub type DISABLE_XR3 = crate::Reg<disable_xr3::DISABLE_XR3_SPEC>;
#[doc = "Disable Flash Page Exec/Read Register 3"]
pub mod disable_xr3;
#[doc = "DISABLE_XR4 register accessor: an alias for `Reg<DISABLE_XR4_SPEC>`"]
pub type DISABLE_XR4 = crate::Reg<disable_xr4::DISABLE_XR4_SPEC>;
#[doc = "Disable Flash Page Exec/Read Register 4"]
pub mod disable_xr4;
#[doc = "DISABLE_XR5 register accessor: an alias for `Reg<DISABLE_XR5_SPEC>`"]
pub type DISABLE_XR5 = crate::Reg<disable_xr5::DISABLE_XR5_SPEC>;
#[doc = "Disable Flash Page Exec/Read Register 5"]
pub mod disable_xr5;
#[doc = "DISABLE_XR6 register accessor: an alias for `Reg<DISABLE_XR6_SPEC>`"]
pub type DISABLE_XR6 = crate::Reg<disable_xr6::DISABLE_XR6_SPEC>;
#[doc = "Disable Flash Page Exec/Read Register 6"]
pub mod disable_xr6;
#[doc = "DISABLE_XR7 register accessor: an alias for `Reg<DISABLE_XR7_SPEC>`"]
pub type DISABLE_XR7 = crate::Reg<disable_xr7::DISABLE_XR7_SPEC>;
#[doc = "Disable Flash Page Exec/Read Register 7"]
pub mod disable_xr7;
#[doc = "DISABLE_WE0 register accessor: an alias for `Reg<DISABLE_WE0_SPEC>`"]
pub type DISABLE_WE0 = crate::Reg<disable_we0::DISABLE_WE0_SPEC>;
#[doc = "Disable Flash Page Write/Erase Register 0"]
pub mod disable_we0;
#[doc = "DISABLE_WE1 register accessor: an alias for `Reg<DISABLE_WE1_SPEC>`"]
pub type DISABLE_WE1 = crate::Reg<disable_we1::DISABLE_WE1_SPEC>;
#[doc = "Disable Flash Page Write/Erase Register 1"]
pub mod disable_we1;
#[doc = "DISABLE_WE2 register accessor: an alias for `Reg<DISABLE_WE2_SPEC>`"]
pub type DISABLE_WE2 = crate::Reg<disable_we2::DISABLE_WE2_SPEC>;
#[doc = "Disable Flash Page Write/Erase Register 2"]
pub mod disable_we2;
#[doc = "DISABLE_WE3 register accessor: an alias for `Reg<DISABLE_WE3_SPEC>`"]
pub type DISABLE_WE3 = crate::Reg<disable_we3::DISABLE_WE3_SPEC>;
#[doc = "Disable Flash Page Write/Erase Register 3"]
pub mod disable_we3;
#[doc = "DISABLE_WE4 register accessor: an alias for `Reg<DISABLE_WE4_SPEC>`"]
pub type DISABLE_WE4 = crate::Reg<disable_we4::DISABLE_WE4_SPEC>;
#[doc = "Disable Flash Page Write/Erase Register 4"]
pub mod disable_we4;
#[doc = "DISABLE_WE5 register accessor: an alias for `Reg<DISABLE_WE5_SPEC>`"]
pub type DISABLE_WE5 = crate::Reg<disable_we5::DISABLE_WE5_SPEC>;
#[doc = "Disable Flash Page Write/Erase Register 5"]
pub mod disable_we5;
#[doc = "DISABLE_WE6 register accessor: an alias for `Reg<DISABLE_WE6_SPEC>`"]
pub type DISABLE_WE6 = crate::Reg<disable_we6::DISABLE_WE6_SPEC>;
#[doc = "Disable Flash Page Write/Erase Register 6"]
pub mod disable_we6;
#[doc = "DISABLE_WE7 register accessor: an alias for `Reg<DISABLE_WE7_SPEC>`"]
pub type DISABLE_WE7 = crate::Reg<disable_we7::DISABLE_WE7_SPEC>;
#[doc = "Disable Flash Page Write/Erase Register 7"]
pub mod disable_we7;
