#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - TPU Secure Key Storage Register 0 (Cleared on Tamper Detect)"]
    pub sks0: crate::Reg<sks0::SKS0_SPEC>,
    #[doc = "0x14 - TPU Secure Key Storage Register 1 (Cleared on Tamper Detect)"]
    pub sks1: crate::Reg<sks1::SKS1_SPEC>,
    #[doc = "0x18 - TPU Secure Key Storage Register 2 (Cleared on Tamper Detect)"]
    pub sks2: crate::Reg<sks2::SKS2_SPEC>,
    #[doc = "0x1c - TPU Secure Key Storage Register 3 (Cleared on Tamper Detect)"]
    pub sks3: crate::Reg<sks3::SKS3_SPEC>,
}
#[doc = "SKS0 register accessor: an alias for `Reg<SKS0_SPEC>`"]
pub type SKS0 = crate::Reg<sks0::SKS0_SPEC>;
#[doc = "TPU Secure Key Storage Register 0 (Cleared on Tamper Detect)"]
pub mod sks0;
#[doc = "SKS1 register accessor: an alias for `Reg<SKS1_SPEC>`"]
pub type SKS1 = crate::Reg<sks1::SKS1_SPEC>;
#[doc = "TPU Secure Key Storage Register 1 (Cleared on Tamper Detect)"]
pub mod sks1;
#[doc = "SKS2 register accessor: an alias for `Reg<SKS2_SPEC>`"]
pub type SKS2 = crate::Reg<sks2::SKS2_SPEC>;
#[doc = "TPU Secure Key Storage Register 2 (Cleared on Tamper Detect)"]
pub mod sks2;
#[doc = "SKS3 register accessor: an alias for `Reg<SKS3_SPEC>`"]
pub type SKS3 = crate::Reg<sks3::SKS3_SPEC>;
#[doc = "TPU Secure Key Storage Register 3 (Cleared on Tamper Detect)"]
pub mod sks3;
