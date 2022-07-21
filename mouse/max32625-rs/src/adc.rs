#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - ADC Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - ADC Output Data"]
    pub data: crate::Reg<data::DATA_SPEC>,
    #[doc = "0x0c - ADC Interrupt Control Register"]
    pub intr: crate::Reg<intr::INTR_SPEC>,
    #[doc = "0x10 - ADC Limit"]
    pub limit0: crate::Reg<limit0::LIMIT0_SPEC>,
    #[doc = "0x14 - ADC Limit 1"]
    pub limit1: crate::Reg<limit1::LIMIT1_SPEC>,
    #[doc = "0x18 - ADC Limit 2"]
    pub limit2: crate::Reg<limit2::LIMIT2_SPEC>,
    #[doc = "0x1c - ADC Limit 3"]
    pub limit3: crate::Reg<limit3::LIMIT3_SPEC>,
    #[doc = "0x20 - AFE Control Register"]
    pub afe_ctrl: crate::Reg<afe_ctrl::AFE_CTRL_SPEC>,
    #[doc = "0x24 - RO Trim Calibration Register 0"]
    pub ro_cal0: crate::Reg<ro_cal0::RO_CAL0_SPEC>,
    #[doc = "0x28 - RO Trim Calibration Register 1"]
    pub ro_cal1: crate::Reg<ro_cal1::RO_CAL1_SPEC>,
    #[doc = "0x2c - RO Trim Calibration Register 2"]
    pub ro_cal2: crate::Reg<ro_cal2::RO_CAL2_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "ADC Control"]
pub mod ctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "ADC Status"]
pub mod status;
#[doc = "DATA register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "ADC Output Data"]
pub mod data;
#[doc = "INTR register accessor: an alias for `Reg<INTR_SPEC>`"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "ADC Interrupt Control Register"]
pub mod intr;
#[doc = "LIMIT0 register accessor: an alias for `Reg<LIMIT0_SPEC>`"]
pub type LIMIT0 = crate::Reg<limit0::LIMIT0_SPEC>;
#[doc = "ADC Limit"]
pub mod limit0;
#[doc = "LIMIT1 register accessor: an alias for `Reg<LIMIT1_SPEC>`"]
pub type LIMIT1 = crate::Reg<limit1::LIMIT1_SPEC>;
#[doc = "ADC Limit 1"]
pub mod limit1;
#[doc = "LIMIT2 register accessor: an alias for `Reg<LIMIT2_SPEC>`"]
pub type LIMIT2 = crate::Reg<limit2::LIMIT2_SPEC>;
#[doc = "ADC Limit 2"]
pub mod limit2;
#[doc = "LIMIT3 register accessor: an alias for `Reg<LIMIT3_SPEC>`"]
pub type LIMIT3 = crate::Reg<limit3::LIMIT3_SPEC>;
#[doc = "ADC Limit 3"]
pub mod limit3;
#[doc = "AFE_CTRL register accessor: an alias for `Reg<AFE_CTRL_SPEC>`"]
pub type AFE_CTRL = crate::Reg<afe_ctrl::AFE_CTRL_SPEC>;
#[doc = "AFE Control Register"]
pub mod afe_ctrl;
#[doc = "RO_CAL0 register accessor: an alias for `Reg<RO_CAL0_SPEC>`"]
pub type RO_CAL0 = crate::Reg<ro_cal0::RO_CAL0_SPEC>;
#[doc = "RO Trim Calibration Register 0"]
pub mod ro_cal0;
#[doc = "RO_CAL1 register accessor: an alias for `Reg<RO_CAL1_SPEC>`"]
pub type RO_CAL1 = crate::Reg<ro_cal1::RO_CAL1_SPEC>;
#[doc = "RO Trim Calibration Register 1"]
pub mod ro_cal1;
#[doc = "RO_CAL2 register accessor: an alias for `Reg<RO_CAL2_SPEC>`"]
pub type RO_CAL2 = crate::Reg<ro_cal2::RO_CAL2_SPEC>;
#[doc = "RO Trim Calibration Register 2"]
pub mod ro_cal2;
