#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starting Descriptor Address"]
    pub dscadr: crate::Reg<dscadr::DSCADR_SPEC>,
    #[doc = "0x04 - Channel Configuration"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x08 - Channel Loop Counters"]
    pub loop_: crate::Reg<loop_::LOOP_SPEC>,
    #[doc = "0x0c - Current Descriptor DWORD 0 (OP)"]
    pub op: crate::Reg<op::OP_SPEC>,
    #[doc = "0x10 - Current Descriptor DWORD 1"]
    pub dsc1: crate::Reg<dsc1::DSC1_SPEC>,
    #[doc = "0x14 - Current Descriptor DWORD 2"]
    pub dsc2: crate::Reg<dsc2::DSC2_SPEC>,
    #[doc = "0x18 - Current Descriptor DWORD 3"]
    pub dsc3: crate::Reg<dsc3::DSC3_SPEC>,
    #[doc = "0x1c - Current Descriptor DWORD 4"]
    pub dsc4: crate::Reg<dsc4::DSC4_SPEC>,
}
#[doc = "DSCADR register accessor: an alias for `Reg<DSCADR_SPEC>`"]
pub type DSCADR = crate::Reg<dscadr::DSCADR_SPEC>;
#[doc = "Starting Descriptor Address"]
pub mod dscadr;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Channel Configuration"]
pub mod cfg;
#[doc = "LOOP register accessor: an alias for `Reg<LOOP_SPEC>`"]
pub type LOOP = crate::Reg<loop_::LOOP_SPEC>;
#[doc = "Channel Loop Counters"]
pub mod loop_;
#[doc = "OP register accessor: an alias for `Reg<OP_SPEC>`"]
pub type OP = crate::Reg<op::OP_SPEC>;
#[doc = "Current Descriptor DWORD 0 (OP)"]
pub mod op;
#[doc = "DSC1 register accessor: an alias for `Reg<DSC1_SPEC>`"]
pub type DSC1 = crate::Reg<dsc1::DSC1_SPEC>;
#[doc = "Current Descriptor DWORD 1"]
pub mod dsc1;
#[doc = "DSC2 register accessor: an alias for `Reg<DSC2_SPEC>`"]
pub type DSC2 = crate::Reg<dsc2::DSC2_SPEC>;
#[doc = "Current Descriptor DWORD 2"]
pub mod dsc2;
#[doc = "DSC3 register accessor: an alias for `Reg<DSC3_SPEC>`"]
pub type DSC3 = crate::Reg<dsc3::DSC3_SPEC>;
#[doc = "Current Descriptor DWORD 3"]
pub mod dsc3;
#[doc = "DSC4 register accessor: an alias for `Reg<DSC4_SPEC>`"]
pub type DSC4 = crate::Reg<dsc4::DSC4_SPEC>;
#[doc = "Current Descriptor DWORD 4"]
pub mod dsc4;
