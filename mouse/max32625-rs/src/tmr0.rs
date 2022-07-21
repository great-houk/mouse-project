#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - 32 bit\\]
Current Count Value"]
    pub count32: crate::Reg<count32::COUNT32_SPEC>,
    #[doc = "0x08 - 32 bit\\]
Terminal Count Setting"]
    pub term_cnt32: crate::Reg<term_cnt32::TERM_CNT32_SPEC>,
    #[doc = "0x0c - 32 bit\\]
PWM Compare Setting or Capture/Measure Value"]
    pub pwm_cap32: crate::Reg<pwm_cap32::PWM_CAP32_SPEC>,
    #[doc = "0x10 - 16 bit\\]
Current Count Value, 16-bit Timer 0"]
    pub count16_0: crate::Reg<count16_0::COUNT16_0_SPEC>,
    #[doc = "0x14 - 16 bit\\]
Terminal Count Setting, 16-bit Timer 0"]
    pub term_cnt16_0: crate::Reg<term_cnt16_0::TERM_CNT16_0_SPEC>,
    #[doc = "0x18 - 16 bit\\]
Current Count Value, 16-bit Timer 1"]
    pub count16_1: crate::Reg<count16_1::COUNT16_1_SPEC>,
    #[doc = "0x1c - 16 bit\\]
Terminal Count Setting, 16-bit Timer 1"]
    pub term_cnt16_1: crate::Reg<term_cnt16_1::TERM_CNT16_1_SPEC>,
    #[doc = "0x20 - Timer Module Interrupt Flags"]
    pub intfl: crate::Reg<intfl::INTFL_SPEC>,
    #[doc = "0x24 - Timer Module Interrupt Enable/Disable Settings"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Timer Control Register"]
pub mod ctrl;
#[doc = "COUNT32 register accessor: an alias for `Reg<COUNT32_SPEC>`"]
pub type COUNT32 = crate::Reg<count32::COUNT32_SPEC>;
#[doc = "32 bit\\]
Current Count Value"]
pub mod count32;
#[doc = "TERM_CNT32 register accessor: an alias for `Reg<TERM_CNT32_SPEC>`"]
pub type TERM_CNT32 = crate::Reg<term_cnt32::TERM_CNT32_SPEC>;
#[doc = "32 bit\\]
Terminal Count Setting"]
pub mod term_cnt32;
#[doc = "PWM_CAP32 register accessor: an alias for `Reg<PWM_CAP32_SPEC>`"]
pub type PWM_CAP32 = crate::Reg<pwm_cap32::PWM_CAP32_SPEC>;
#[doc = "32 bit\\]
PWM Compare Setting or Capture/Measure Value"]
pub mod pwm_cap32;
#[doc = "COUNT16_0 register accessor: an alias for `Reg<COUNT16_0_SPEC>`"]
pub type COUNT16_0 = crate::Reg<count16_0::COUNT16_0_SPEC>;
#[doc = "16 bit\\]
Current Count Value, 16-bit Timer 0"]
pub mod count16_0;
#[doc = "TERM_CNT16_0 register accessor: an alias for `Reg<TERM_CNT16_0_SPEC>`"]
pub type TERM_CNT16_0 = crate::Reg<term_cnt16_0::TERM_CNT16_0_SPEC>;
#[doc = "16 bit\\]
Terminal Count Setting, 16-bit Timer 0"]
pub mod term_cnt16_0;
#[doc = "COUNT16_1 register accessor: an alias for `Reg<COUNT16_1_SPEC>`"]
pub type COUNT16_1 = crate::Reg<count16_1::COUNT16_1_SPEC>;
#[doc = "16 bit\\]
Current Count Value, 16-bit Timer 1"]
pub mod count16_1;
#[doc = "TERM_CNT16_1 register accessor: an alias for `Reg<TERM_CNT16_1_SPEC>`"]
pub type TERM_CNT16_1 = crate::Reg<term_cnt16_1::TERM_CNT16_1_SPEC>;
#[doc = "16 bit\\]
Terminal Count Setting, 16-bit Timer 1"]
pub mod term_cnt16_1;
#[doc = "INTFL register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Timer Module Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Timer Module Interrupt Enable/Disable Settings"]
pub mod inten;
