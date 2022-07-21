#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Reset Control and Status"]
    pub pwr_rst_ctrl: crate::Reg<pwr_rst_ctrl::PWR_RST_CTRL_SPEC>,
    #[doc = "0x04 - Interrupt Flags"]
    pub intfl: crate::Reg<intfl::INTFL_SPEC>,
    #[doc = "0x08 - Interrupt Enable/Disable Controls"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x0c - SVM Event Status Flags (read-only)"]
    pub svm_events: crate::Reg<svm_events::SVM_EVENTS_SPEC>,
    #[doc = "0x10 - Wake-Up Detect Control"]
    pub wud_ctrl: crate::Reg<wud_ctrl::WUD_CTRL_SPEC>,
    #[doc = "0x14 - WUD Pulse To Mode Bit 0"]
    pub wud_pulse0: crate::Reg<wud_pulse0::WUD_PULSE0_SPEC>,
    #[doc = "0x18 - WUD Pulse To Mode Bit 1"]
    pub wud_pulse1: crate::Reg<wud_pulse1::WUD_PULSE1_SPEC>,
    #[doc = "0x1c - Wake-up Detect Status for P0/P1/P2/P3"]
    pub wud_seen0: crate::Reg<wud_seen0::WUD_SEEN0_SPEC>,
    #[doc = "0x20 - Wake-up Detect Status for P4/P5/P6/P7"]
    pub wud_seen1: crate::Reg<wud_seen1::WUD_SEEN1_SPEC>,
    _reserved9: [u8; 0x10],
    #[doc = "0x34 - SRAM Margin Adjustment"]
    pub margin_ctrl: crate::Reg<margin_ctrl::MARGIN_CTRL_SPEC>,
    #[doc = "0x38 - Die Type ID Register"]
    pub die_type: crate::Reg<die_type::DIE_TYPE_SPEC>,
    #[doc = "0x3c - Base Part Number"]
    pub base_part_num: crate::Reg<base_part_num::BASE_PART_NUM_SPEC>,
    #[doc = "0x40 - Mask ID Register 0"]
    pub mask_id0: crate::Reg<mask_id0::MASK_ID0_SPEC>,
    #[doc = "0x44 - Mask ID Register 1"]
    pub mask_id1: crate::Reg<mask_id1::MASK_ID1_SPEC>,
    #[doc = "0x48 - Peripheral Reset Control Register"]
    pub peripheral_reset: crate::Reg<peripheral_reset::PERIPHERAL_RESET_SPEC>,
}
#[doc = "PWR_RST_CTRL register accessor: an alias for `Reg<PWR_RST_CTRL_SPEC>`"]
pub type PWR_RST_CTRL = crate::Reg<pwr_rst_ctrl::PWR_RST_CTRL_SPEC>;
#[doc = "Power Reset Control and Status"]
pub mod pwr_rst_ctrl;
#[doc = "INTFL register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable/Disable Controls"]
pub mod inten;
#[doc = "SVM_EVENTS register accessor: an alias for `Reg<SVM_EVENTS_SPEC>`"]
pub type SVM_EVENTS = crate::Reg<svm_events::SVM_EVENTS_SPEC>;
#[doc = "SVM Event Status Flags (read-only)"]
pub mod svm_events;
#[doc = "WUD_CTRL register accessor: an alias for `Reg<WUD_CTRL_SPEC>`"]
pub type WUD_CTRL = crate::Reg<wud_ctrl::WUD_CTRL_SPEC>;
#[doc = "Wake-Up Detect Control"]
pub mod wud_ctrl;
#[doc = "WUD_PULSE0 register accessor: an alias for `Reg<WUD_PULSE0_SPEC>`"]
pub type WUD_PULSE0 = crate::Reg<wud_pulse0::WUD_PULSE0_SPEC>;
#[doc = "WUD Pulse To Mode Bit 0"]
pub mod wud_pulse0;
#[doc = "WUD_PULSE1 register accessor: an alias for `Reg<WUD_PULSE1_SPEC>`"]
pub type WUD_PULSE1 = crate::Reg<wud_pulse1::WUD_PULSE1_SPEC>;
#[doc = "WUD Pulse To Mode Bit 1"]
pub mod wud_pulse1;
#[doc = "WUD_SEEN0 register accessor: an alias for `Reg<WUD_SEEN0_SPEC>`"]
pub type WUD_SEEN0 = crate::Reg<wud_seen0::WUD_SEEN0_SPEC>;
#[doc = "Wake-up Detect Status for P0/P1/P2/P3"]
pub mod wud_seen0;
#[doc = "WUD_SEEN1 register accessor: an alias for `Reg<WUD_SEEN1_SPEC>`"]
pub type WUD_SEEN1 = crate::Reg<wud_seen1::WUD_SEEN1_SPEC>;
#[doc = "Wake-up Detect Status for P4/P5/P6/P7"]
pub mod wud_seen1;
#[doc = "MARGIN_CTRL register accessor: an alias for `Reg<MARGIN_CTRL_SPEC>`"]
pub type MARGIN_CTRL = crate::Reg<margin_ctrl::MARGIN_CTRL_SPEC>;
#[doc = "SRAM Margin Adjustment"]
pub mod margin_ctrl;
#[doc = "DIE_TYPE register accessor: an alias for `Reg<DIE_TYPE_SPEC>`"]
pub type DIE_TYPE = crate::Reg<die_type::DIE_TYPE_SPEC>;
#[doc = "Die Type ID Register"]
pub mod die_type;
#[doc = "BASE_PART_NUM register accessor: an alias for `Reg<BASE_PART_NUM_SPEC>`"]
pub type BASE_PART_NUM = crate::Reg<base_part_num::BASE_PART_NUM_SPEC>;
#[doc = "Base Part Number"]
pub mod base_part_num;
#[doc = "MASK_ID0 register accessor: an alias for `Reg<MASK_ID0_SPEC>`"]
pub type MASK_ID0 = crate::Reg<mask_id0::MASK_ID0_SPEC>;
#[doc = "Mask ID Register 0"]
pub mod mask_id0;
#[doc = "MASK_ID1 register accessor: an alias for `Reg<MASK_ID1_SPEC>`"]
pub type MASK_ID1 = crate::Reg<mask_id1::MASK_ID1_SPEC>;
#[doc = "Mask ID Register 1"]
pub mod mask_id1;
#[doc = "PERIPHERAL_RESET register accessor: an alias for `Reg<PERIPHERAL_RESET_SPEC>`"]
pub type PERIPHERAL_RESET = crate::Reg<peripheral_reset::PERIPHERAL_RESET_SPEC>;
#[doc = "Peripheral Reset Control Register"]
pub mod peripheral_reset;
