#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Timer Control"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - RTC Timer Count Value"]
    pub timer: crate::Reg<timer::TIMER_SPEC>,
    #[doc = "0x08 - RTC Time of Day Alarm 0 Compare Register"]
    pub comp0: crate::Reg<comp0::COMP0_SPEC>,
    #[doc = "0x0c - RTC Time of Day Alarm 1 Compare Register"]
    pub comp1: crate::Reg<comp1::COMP1_SPEC>,
    #[doc = "0x10 - CPU Interrupt and RTC Domain Flags"]
    pub flags: crate::Reg<flags::FLAGS_SPEC>,
    #[doc = "0x14 - RTC Timer Alarm Snooze Value"]
    pub snz_value: crate::Reg<snz_value::SNZ_VALUE_SPEC>,
    #[doc = "0x18 - Interrupt Enable Controls"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x1c - RTC Timer Prescale Setting"]
    pub prescale: crate::Reg<prescale::PRESCALE_SPEC>,
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - RTC Timer Prescale Compare Mask"]
    pub prescale_mask: crate::Reg<prescale_mask::PRESCALE_MASK_SPEC>,
    #[doc = "0x28 - RTC Timer Trim Controls"]
    pub trim_ctrl: crate::Reg<trim_ctrl::TRIM_CTRL_SPEC>,
    #[doc = "0x2c - RTC Timer Trim Adjustment Interval"]
    pub trim_value: crate::Reg<trim_value::TRIM_VALUE_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC Timer Control"]
pub mod ctrl;
#[doc = "TIMER register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "RTC Timer Count Value"]
pub mod timer;
#[doc = "COMP0 register accessor: an alias for `Reg<COMP0_SPEC>`"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "RTC Time of Day Alarm 0 Compare Register"]
pub mod comp0;
#[doc = "COMP1 register accessor: an alias for `Reg<COMP1_SPEC>`"]
pub type COMP1 = crate::Reg<comp1::COMP1_SPEC>;
#[doc = "RTC Time of Day Alarm 1 Compare Register"]
pub mod comp1;
#[doc = "FLAGS register accessor: an alias for `Reg<FLAGS_SPEC>`"]
pub type FLAGS = crate::Reg<flags::FLAGS_SPEC>;
#[doc = "CPU Interrupt and RTC Domain Flags"]
pub mod flags;
#[doc = "SNZ_VALUE register accessor: an alias for `Reg<SNZ_VALUE_SPEC>`"]
pub type SNZ_VALUE = crate::Reg<snz_value::SNZ_VALUE_SPEC>;
#[doc = "RTC Timer Alarm Snooze Value"]
pub mod snz_value;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable Controls"]
pub mod inten;
#[doc = "PRESCALE register accessor: an alias for `Reg<PRESCALE_SPEC>`"]
pub type PRESCALE = crate::Reg<prescale::PRESCALE_SPEC>;
#[doc = "RTC Timer Prescale Setting"]
pub mod prescale;
#[doc = "PRESCALE_MASK register accessor: an alias for `Reg<PRESCALE_MASK_SPEC>`"]
pub type PRESCALE_MASK = crate::Reg<prescale_mask::PRESCALE_MASK_SPEC>;
#[doc = "RTC Timer Prescale Compare Mask"]
pub mod prescale_mask;
#[doc = "TRIM_CTRL register accessor: an alias for `Reg<TRIM_CTRL_SPEC>`"]
pub type TRIM_CTRL = crate::Reg<trim_ctrl::TRIM_CTRL_SPEC>;
#[doc = "RTC Timer Trim Controls"]
pub mod trim_ctrl;
#[doc = "TRIM_VALUE register accessor: an alias for `Reg<TRIM_VALUE_SPEC>`"]
pub type TRIM_VALUE = crate::Reg<trim_value::TRIM_VALUE_SPEC>;
#[doc = "RTC Timer Trim Adjustment Interval"]
pub mod trim_value;
