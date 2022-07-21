#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port P0 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p0: crate::Reg<rst_mode_p0::RST_MODE_P0_SPEC>,
    #[doc = "0x04 - Port P1 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p1: crate::Reg<rst_mode_p1::RST_MODE_P1_SPEC>,
    #[doc = "0x08 - Port P2 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p2: crate::Reg<rst_mode_p2::RST_MODE_P2_SPEC>,
    #[doc = "0x0c - Port P3 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p3: crate::Reg<rst_mode_p3::RST_MODE_P3_SPEC>,
    #[doc = "0x10 - Port P4 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p4: crate::Reg<rst_mode_p4::RST_MODE_P4_SPEC>,
    #[doc = "0x14 - Port P5 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p5: crate::Reg<rst_mode_p5::RST_MODE_P5_SPEC>,
    #[doc = "0x18 - Port P6 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p6: crate::Reg<rst_mode_p6::RST_MODE_P6_SPEC>,
    #[doc = "0x1c - Port P7 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p7: crate::Reg<rst_mode_p7::RST_MODE_P7_SPEC>,
    #[doc = "0x20 - Port P8 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p8: crate::Reg<rst_mode_p8::RST_MODE_P8_SPEC>,
    _reserved9: [u8; 0x1c],
    #[doc = "0x40 - Port P0 Free for GPIO Operation Flags"]
    pub free_p0: crate::Reg<free_p0::FREE_P0_SPEC>,
    #[doc = "0x44 - Port P1 Free for GPIO Operation Flags"]
    pub free_p1: crate::Reg<free_p1::FREE_P1_SPEC>,
    #[doc = "0x48 - Port P2 Free for GPIO Operation Flags"]
    pub free_p2: crate::Reg<free_p2::FREE_P2_SPEC>,
    #[doc = "0x4c - Port P3 Free for GPIO Operation Flags"]
    pub free_p3: crate::Reg<free_p3::FREE_P3_SPEC>,
    #[doc = "0x50 - Port P4 Free for GPIO Operation Flags"]
    pub free_p4: crate::Reg<free_p4::FREE_P4_SPEC>,
    #[doc = "0x54 - Port P5 Free for GPIO Operation Flags"]
    pub free_p5: crate::Reg<free_p5::FREE_P5_SPEC>,
    #[doc = "0x58 - Port P6 Free for GPIO Operation Flags"]
    pub free_p6: crate::Reg<free_p6::FREE_P6_SPEC>,
    #[doc = "0x5c - Port P7 Free for GPIO Operation Flags"]
    pub free_p7: crate::Reg<free_p7::FREE_P7_SPEC>,
    #[doc = "0x60 - Port P8 Free for GPIO Operation Flags"]
    pub free_p8: crate::Reg<free_p8::FREE_P8_SPEC>,
    _reserved18: [u8; 0x1c],
    #[doc = "0x80 - Port P0 GPIO Output Drive Mode"]
    pub out_mode_p0: crate::Reg<out_mode_p0::OUT_MODE_P0_SPEC>,
    #[doc = "0x84 - Port P1 GPIO Output Drive Mode"]
    pub out_mode_p1: crate::Reg<out_mode_p1::OUT_MODE_P1_SPEC>,
    #[doc = "0x88 - Port P2 GPIO Output Drive Mode"]
    pub out_mode_p2: crate::Reg<out_mode_p2::OUT_MODE_P2_SPEC>,
    #[doc = "0x8c - Port P3 GPIO Output Drive Mode"]
    pub out_mode_p3: crate::Reg<out_mode_p3::OUT_MODE_P3_SPEC>,
    #[doc = "0x90 - Port P4 GPIO Output Drive Mode"]
    pub out_mode_p4: crate::Reg<out_mode_p4::OUT_MODE_P4_SPEC>,
    #[doc = "0x94 - Port P5 GPIO Output Drive Mode"]
    pub out_mode_p5: crate::Reg<out_mode_p5::OUT_MODE_P5_SPEC>,
    #[doc = "0x98 - Port P6 GPIO Output Drive Mode"]
    pub out_mode_p6: crate::Reg<out_mode_p6::OUT_MODE_P6_SPEC>,
    #[doc = "0x9c - Port P7 GPIO Output Drive Mode"]
    pub out_mode_p7: crate::Reg<out_mode_p7::OUT_MODE_P7_SPEC>,
    #[doc = "0xa0 - Port P8 GPIO Output Drive Mode"]
    pub out_mode_p8: crate::Reg<out_mode_p8::OUT_MODE_P8_SPEC>,
    _reserved27: [u8; 0x1c],
    #[doc = "0xc0 - Port P0 GPIO Output Value"]
    pub out_val_p0: crate::Reg<out_val_p0::OUT_VAL_P0_SPEC>,
    #[doc = "0xc4 - Port P1 GPIO Output Value"]
    pub out_val_p1: crate::Reg<out_val_p1::OUT_VAL_P1_SPEC>,
    #[doc = "0xc8 - Port P2 GPIO Output Value"]
    pub out_val_p2: crate::Reg<out_val_p2::OUT_VAL_P2_SPEC>,
    #[doc = "0xcc - Port P3 GPIO Output Value"]
    pub out_val_p3: crate::Reg<out_val_p3::OUT_VAL_P3_SPEC>,
    #[doc = "0xd0 - Port P4 GPIO Output Value"]
    pub out_val_p4: crate::Reg<out_val_p4::OUT_VAL_P4_SPEC>,
    #[doc = "0xd4 - Port P5 GPIO Output Value"]
    pub out_val_p5: crate::Reg<out_val_p5::OUT_VAL_P5_SPEC>,
    #[doc = "0xd8 - Port P6 GPIO Output Value"]
    pub out_val_p6: crate::Reg<out_val_p6::OUT_VAL_P6_SPEC>,
    #[doc = "0xdc - Port P7 GPIO Output Value"]
    pub out_val_p7: crate::Reg<out_val_p7::OUT_VAL_P7_SPEC>,
    #[doc = "0xe0 - Port P8 GPIO Output Value"]
    pub out_val_p8: crate::Reg<out_val_p8::OUT_VAL_P8_SPEC>,
    _reserved36: [u8; 0x1c],
    #[doc = "0x100 - Port P0 GPIO Function Select"]
    pub func_sel_p0: crate::Reg<func_sel_p0::FUNC_SEL_P0_SPEC>,
    #[doc = "0x104 - Port P1 GPIO Function Select"]
    pub func_sel_p1: crate::Reg<func_sel_p1::FUNC_SEL_P1_SPEC>,
    #[doc = "0x108 - Port P2 GPIO Function Select"]
    pub func_sel_p2: crate::Reg<func_sel_p2::FUNC_SEL_P2_SPEC>,
    #[doc = "0x10c - Port P3 GPIO Function Select"]
    pub func_sel_p3: crate::Reg<func_sel_p3::FUNC_SEL_P3_SPEC>,
    #[doc = "0x110 - Port P4 GPIO Function Select"]
    pub func_sel_p4: crate::Reg<func_sel_p4::FUNC_SEL_P4_SPEC>,
    #[doc = "0x114 - Port P5 GPIO Function Select"]
    pub func_sel_p5: crate::Reg<func_sel_p5::FUNC_SEL_P5_SPEC>,
    #[doc = "0x118 - Port P6 GPIO Function Select"]
    pub func_sel_p6: crate::Reg<func_sel_p6::FUNC_SEL_P6_SPEC>,
    #[doc = "0x11c - Port P7 GPIO Function Select"]
    pub func_sel_p7: crate::Reg<func_sel_p7::FUNC_SEL_P7_SPEC>,
    #[doc = "0x120 - Port P8 GPIO Function Select"]
    pub func_sel_p8: crate::Reg<func_sel_p8::FUNC_SEL_P8_SPEC>,
    _reserved45: [u8; 0x1c],
    #[doc = "0x140 - Port P0 GPIO Input Monitoring Mode"]
    pub in_mode_p0: crate::Reg<in_mode_p0::IN_MODE_P0_SPEC>,
    #[doc = "0x144 - Port P1 GPIO Input Monitoring Mode"]
    pub in_mode_p1: crate::Reg<in_mode_p1::IN_MODE_P1_SPEC>,
    #[doc = "0x148 - Port P2 GPIO Input Monitoring Mode"]
    pub in_mode_p2: crate::Reg<in_mode_p2::IN_MODE_P2_SPEC>,
    #[doc = "0x14c - Port P3 GPIO Input Monitoring Mode"]
    pub in_mode_p3: crate::Reg<in_mode_p3::IN_MODE_P3_SPEC>,
    #[doc = "0x150 - Port P4 GPIO Input Monitoring Mode"]
    pub in_mode_p4: crate::Reg<in_mode_p4::IN_MODE_P4_SPEC>,
    #[doc = "0x154 - Port P5 GPIO Input Monitoring Mode"]
    pub in_mode_p5: crate::Reg<in_mode_p5::IN_MODE_P5_SPEC>,
    #[doc = "0x158 - Port P6 GPIO Input Monitoring Mode"]
    pub in_mode_p6: crate::Reg<in_mode_p6::IN_MODE_P6_SPEC>,
    #[doc = "0x15c - Port P7 GPIO Input Monitoring Mode"]
    pub in_mode_p7: crate::Reg<in_mode_p7::IN_MODE_P7_SPEC>,
    #[doc = "0x160 - Port P8 GPIO Input Monitoring Mode"]
    pub in_mode_p8: crate::Reg<in_mode_p8::IN_MODE_P8_SPEC>,
    _reserved54: [u8; 0x1c],
    #[doc = "0x180 - Port P0 GPIO Input Value"]
    pub in_val_p0: crate::Reg<in_val_p0::IN_VAL_P0_SPEC>,
    #[doc = "0x184 - Port P1 GPIO Input Value"]
    pub in_val_p1: crate::Reg<in_val_p1::IN_VAL_P1_SPEC>,
    #[doc = "0x188 - Port P2 GPIO Input Value"]
    pub in_val_p2: crate::Reg<in_val_p2::IN_VAL_P2_SPEC>,
    #[doc = "0x18c - Port P3 GPIO Input Value"]
    pub in_val_p3: crate::Reg<in_val_p3::IN_VAL_P3_SPEC>,
    #[doc = "0x190 - Port P4 GPIO Input Value"]
    pub in_val_p4: crate::Reg<in_val_p4::IN_VAL_P4_SPEC>,
    #[doc = "0x194 - Port P5 GPIO Input Value"]
    pub in_val_p5: crate::Reg<in_val_p5::IN_VAL_P5_SPEC>,
    #[doc = "0x198 - Port P6 GPIO Input Value"]
    pub in_val_p6: crate::Reg<in_val_p6::IN_VAL_P6_SPEC>,
    #[doc = "0x19c - Port P7 GPIO Input Value"]
    pub in_val_p7: crate::Reg<in_val_p7::IN_VAL_P7_SPEC>,
    #[doc = "0x1a0 - Port P8 GPIO Input Value"]
    pub in_val_p8: crate::Reg<in_val_p8::IN_VAL_P8_SPEC>,
    _reserved63: [u8; 0x1c],
    #[doc = "0x1c0 - Port P0 Interrupt Detection Mode"]
    pub int_mode_p0: crate::Reg<int_mode_p0::INT_MODE_P0_SPEC>,
    #[doc = "0x1c4 - Port P1 Interrupt Detection Mode"]
    pub int_mode_p1: crate::Reg<int_mode_p1::INT_MODE_P1_SPEC>,
    #[doc = "0x1c8 - Port P2 Interrupt Detection Mode"]
    pub int_mode_p2: crate::Reg<int_mode_p2::INT_MODE_P2_SPEC>,
    #[doc = "0x1cc - Port P3 Interrupt Detection Mode"]
    pub int_mode_p3: crate::Reg<int_mode_p3::INT_MODE_P3_SPEC>,
    #[doc = "0x1d0 - Port P4 Interrupt Detection Mode"]
    pub int_mode_p4: crate::Reg<int_mode_p4::INT_MODE_P4_SPEC>,
    #[doc = "0x1d4 - Port P5 Interrupt Detection Mode"]
    pub int_mode_p5: crate::Reg<int_mode_p5::INT_MODE_P5_SPEC>,
    #[doc = "0x1d8 - Port P6 Interrupt Detection Mode"]
    pub int_mode_p6: crate::Reg<int_mode_p6::INT_MODE_P6_SPEC>,
    #[doc = "0x1dc - Port P7 Interrupt Detection Mode"]
    pub int_mode_p7: crate::Reg<int_mode_p7::INT_MODE_P7_SPEC>,
    #[doc = "0x1e0 - Port P8 Interrupt Detection Mode"]
    pub int_mode_p8: crate::Reg<int_mode_p8::INT_MODE_P8_SPEC>,
    _reserved72: [u8; 0x1c],
    #[doc = "0x200 - Port P0 Interrupt Flags"]
    pub intfl_p0: crate::Reg<intfl_p0::INTFL_P0_SPEC>,
    #[doc = "0x204 - Port P1 Interrupt Flags"]
    pub intfl_p1: crate::Reg<intfl_p1::INTFL_P1_SPEC>,
    #[doc = "0x208 - Port P2 Interrupt Flags"]
    pub intfl_p2: crate::Reg<intfl_p2::INTFL_P2_SPEC>,
    #[doc = "0x20c - Port P3 Interrupt Flags"]
    pub intfl_p3: crate::Reg<intfl_p3::INTFL_P3_SPEC>,
    #[doc = "0x210 - Port P4 Interrupt Flags"]
    pub intfl_p4: crate::Reg<intfl_p4::INTFL_P4_SPEC>,
    #[doc = "0x214 - Port P5 Interrupt Flags"]
    pub intfl_p5: crate::Reg<intfl_p5::INTFL_P5_SPEC>,
    #[doc = "0x218 - Port P6 Interrupt Flags"]
    pub intfl_p6: crate::Reg<intfl_p6::INTFL_P6_SPEC>,
    #[doc = "0x21c - Port P7 Interrupt Flags"]
    pub intfl_p7: crate::Reg<intfl_p7::INTFL_P7_SPEC>,
    #[doc = "0x220 - Port P8 Interrupt Flags"]
    pub intfl_p8: crate::Reg<intfl_p8::INTFL_P8_SPEC>,
    _reserved81: [u8; 0x1c],
    #[doc = "0x240 - Port P0 Interrupt Enables"]
    pub inten_p0: crate::Reg<inten_p0::INTEN_P0_SPEC>,
    #[doc = "0x244 - Port P1 Interrupt Enables"]
    pub inten_p1: crate::Reg<inten_p1::INTEN_P1_SPEC>,
    #[doc = "0x248 - Port P2 Interrupt Enables"]
    pub inten_p2: crate::Reg<inten_p2::INTEN_P2_SPEC>,
    #[doc = "0x24c - Port P3 Interrupt Enables"]
    pub inten_p3: crate::Reg<inten_p3::INTEN_P3_SPEC>,
    #[doc = "0x250 - Port P4 Interrupt Enables"]
    pub inten_p4: crate::Reg<inten_p4::INTEN_P4_SPEC>,
    #[doc = "0x254 - Port P5 Interrupt Enables"]
    pub inten_p5: crate::Reg<inten_p5::INTEN_P5_SPEC>,
    #[doc = "0x258 - Port P6 Interrupt Enables"]
    pub inten_p6: crate::Reg<inten_p6::INTEN_P6_SPEC>,
    #[doc = "0x25c - Port P7 Interrupt Enables"]
    pub inten_p7: crate::Reg<inten_p7::INTEN_P7_SPEC>,
    #[doc = "0x260 - Port P8 Interrupt Enables"]
    pub inten_p8: crate::Reg<inten_p8::INTEN_P8_SPEC>,
}
#[doc = "RST_MODE_P0 register accessor: an alias for `Reg<RST_MODE_P0_SPEC>`"]
pub type RST_MODE_P0 = crate::Reg<rst_mode_p0::RST_MODE_P0_SPEC>;
#[doc = "Port P0 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p0;
#[doc = "RST_MODE_P1 register accessor: an alias for `Reg<RST_MODE_P1_SPEC>`"]
pub type RST_MODE_P1 = crate::Reg<rst_mode_p1::RST_MODE_P1_SPEC>;
#[doc = "Port P1 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p1;
#[doc = "RST_MODE_P2 register accessor: an alias for `Reg<RST_MODE_P2_SPEC>`"]
pub type RST_MODE_P2 = crate::Reg<rst_mode_p2::RST_MODE_P2_SPEC>;
#[doc = "Port P2 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p2;
#[doc = "RST_MODE_P3 register accessor: an alias for `Reg<RST_MODE_P3_SPEC>`"]
pub type RST_MODE_P3 = crate::Reg<rst_mode_p3::RST_MODE_P3_SPEC>;
#[doc = "Port P3 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p3;
#[doc = "RST_MODE_P4 register accessor: an alias for `Reg<RST_MODE_P4_SPEC>`"]
pub type RST_MODE_P4 = crate::Reg<rst_mode_p4::RST_MODE_P4_SPEC>;
#[doc = "Port P4 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p4;
#[doc = "RST_MODE_P5 register accessor: an alias for `Reg<RST_MODE_P5_SPEC>`"]
pub type RST_MODE_P5 = crate::Reg<rst_mode_p5::RST_MODE_P5_SPEC>;
#[doc = "Port P5 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p5;
#[doc = "RST_MODE_P6 register accessor: an alias for `Reg<RST_MODE_P6_SPEC>`"]
pub type RST_MODE_P6 = crate::Reg<rst_mode_p6::RST_MODE_P6_SPEC>;
#[doc = "Port P6 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p6;
#[doc = "RST_MODE_P7 register accessor: an alias for `Reg<RST_MODE_P7_SPEC>`"]
pub type RST_MODE_P7 = crate::Reg<rst_mode_p7::RST_MODE_P7_SPEC>;
#[doc = "Port P7 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p7;
#[doc = "RST_MODE_P8 register accessor: an alias for `Reg<RST_MODE_P8_SPEC>`"]
pub type RST_MODE_P8 = crate::Reg<rst_mode_p8::RST_MODE_P8_SPEC>;
#[doc = "Port P8 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p8;
#[doc = "FREE_P0 register accessor: an alias for `Reg<FREE_P0_SPEC>`"]
pub type FREE_P0 = crate::Reg<free_p0::FREE_P0_SPEC>;
#[doc = "Port P0 Free for GPIO Operation Flags"]
pub mod free_p0;
#[doc = "FREE_P1 register accessor: an alias for `Reg<FREE_P1_SPEC>`"]
pub type FREE_P1 = crate::Reg<free_p1::FREE_P1_SPEC>;
#[doc = "Port P1 Free for GPIO Operation Flags"]
pub mod free_p1;
#[doc = "FREE_P2 register accessor: an alias for `Reg<FREE_P2_SPEC>`"]
pub type FREE_P2 = crate::Reg<free_p2::FREE_P2_SPEC>;
#[doc = "Port P2 Free for GPIO Operation Flags"]
pub mod free_p2;
#[doc = "FREE_P3 register accessor: an alias for `Reg<FREE_P3_SPEC>`"]
pub type FREE_P3 = crate::Reg<free_p3::FREE_P3_SPEC>;
#[doc = "Port P3 Free for GPIO Operation Flags"]
pub mod free_p3;
#[doc = "FREE_P4 register accessor: an alias for `Reg<FREE_P4_SPEC>`"]
pub type FREE_P4 = crate::Reg<free_p4::FREE_P4_SPEC>;
#[doc = "Port P4 Free for GPIO Operation Flags"]
pub mod free_p4;
#[doc = "FREE_P5 register accessor: an alias for `Reg<FREE_P5_SPEC>`"]
pub type FREE_P5 = crate::Reg<free_p5::FREE_P5_SPEC>;
#[doc = "Port P5 Free for GPIO Operation Flags"]
pub mod free_p5;
#[doc = "FREE_P6 register accessor: an alias for `Reg<FREE_P6_SPEC>`"]
pub type FREE_P6 = crate::Reg<free_p6::FREE_P6_SPEC>;
#[doc = "Port P6 Free for GPIO Operation Flags"]
pub mod free_p6;
#[doc = "FREE_P7 register accessor: an alias for `Reg<FREE_P7_SPEC>`"]
pub type FREE_P7 = crate::Reg<free_p7::FREE_P7_SPEC>;
#[doc = "Port P7 Free for GPIO Operation Flags"]
pub mod free_p7;
#[doc = "FREE_P8 register accessor: an alias for `Reg<FREE_P8_SPEC>`"]
pub type FREE_P8 = crate::Reg<free_p8::FREE_P8_SPEC>;
#[doc = "Port P8 Free for GPIO Operation Flags"]
pub mod free_p8;
#[doc = "OUT_MODE_P0 register accessor: an alias for `Reg<OUT_MODE_P0_SPEC>`"]
pub type OUT_MODE_P0 = crate::Reg<out_mode_p0::OUT_MODE_P0_SPEC>;
#[doc = "Port P0 GPIO Output Drive Mode"]
pub mod out_mode_p0;
#[doc = "OUT_MODE_P1 register accessor: an alias for `Reg<OUT_MODE_P1_SPEC>`"]
pub type OUT_MODE_P1 = crate::Reg<out_mode_p1::OUT_MODE_P1_SPEC>;
#[doc = "Port P1 GPIO Output Drive Mode"]
pub mod out_mode_p1;
#[doc = "OUT_MODE_P2 register accessor: an alias for `Reg<OUT_MODE_P2_SPEC>`"]
pub type OUT_MODE_P2 = crate::Reg<out_mode_p2::OUT_MODE_P2_SPEC>;
#[doc = "Port P2 GPIO Output Drive Mode"]
pub mod out_mode_p2;
#[doc = "OUT_MODE_P3 register accessor: an alias for `Reg<OUT_MODE_P3_SPEC>`"]
pub type OUT_MODE_P3 = crate::Reg<out_mode_p3::OUT_MODE_P3_SPEC>;
#[doc = "Port P3 GPIO Output Drive Mode"]
pub mod out_mode_p3;
#[doc = "OUT_MODE_P4 register accessor: an alias for `Reg<OUT_MODE_P4_SPEC>`"]
pub type OUT_MODE_P4 = crate::Reg<out_mode_p4::OUT_MODE_P4_SPEC>;
#[doc = "Port P4 GPIO Output Drive Mode"]
pub mod out_mode_p4;
#[doc = "OUT_MODE_P5 register accessor: an alias for `Reg<OUT_MODE_P5_SPEC>`"]
pub type OUT_MODE_P5 = crate::Reg<out_mode_p5::OUT_MODE_P5_SPEC>;
#[doc = "Port P5 GPIO Output Drive Mode"]
pub mod out_mode_p5;
#[doc = "OUT_MODE_P6 register accessor: an alias for `Reg<OUT_MODE_P6_SPEC>`"]
pub type OUT_MODE_P6 = crate::Reg<out_mode_p6::OUT_MODE_P6_SPEC>;
#[doc = "Port P6 GPIO Output Drive Mode"]
pub mod out_mode_p6;
#[doc = "OUT_MODE_P7 register accessor: an alias for `Reg<OUT_MODE_P7_SPEC>`"]
pub type OUT_MODE_P7 = crate::Reg<out_mode_p7::OUT_MODE_P7_SPEC>;
#[doc = "Port P7 GPIO Output Drive Mode"]
pub mod out_mode_p7;
#[doc = "OUT_MODE_P8 register accessor: an alias for `Reg<OUT_MODE_P8_SPEC>`"]
pub type OUT_MODE_P8 = crate::Reg<out_mode_p8::OUT_MODE_P8_SPEC>;
#[doc = "Port P8 GPIO Output Drive Mode"]
pub mod out_mode_p8;
#[doc = "OUT_VAL_P0 register accessor: an alias for `Reg<OUT_VAL_P0_SPEC>`"]
pub type OUT_VAL_P0 = crate::Reg<out_val_p0::OUT_VAL_P0_SPEC>;
#[doc = "Port P0 GPIO Output Value"]
pub mod out_val_p0;
#[doc = "OUT_VAL_P1 register accessor: an alias for `Reg<OUT_VAL_P1_SPEC>`"]
pub type OUT_VAL_P1 = crate::Reg<out_val_p1::OUT_VAL_P1_SPEC>;
#[doc = "Port P1 GPIO Output Value"]
pub mod out_val_p1;
#[doc = "OUT_VAL_P2 register accessor: an alias for `Reg<OUT_VAL_P2_SPEC>`"]
pub type OUT_VAL_P2 = crate::Reg<out_val_p2::OUT_VAL_P2_SPEC>;
#[doc = "Port P2 GPIO Output Value"]
pub mod out_val_p2;
#[doc = "OUT_VAL_P3 register accessor: an alias for `Reg<OUT_VAL_P3_SPEC>`"]
pub type OUT_VAL_P3 = crate::Reg<out_val_p3::OUT_VAL_P3_SPEC>;
#[doc = "Port P3 GPIO Output Value"]
pub mod out_val_p3;
#[doc = "OUT_VAL_P4 register accessor: an alias for `Reg<OUT_VAL_P4_SPEC>`"]
pub type OUT_VAL_P4 = crate::Reg<out_val_p4::OUT_VAL_P4_SPEC>;
#[doc = "Port P4 GPIO Output Value"]
pub mod out_val_p4;
#[doc = "OUT_VAL_P5 register accessor: an alias for `Reg<OUT_VAL_P5_SPEC>`"]
pub type OUT_VAL_P5 = crate::Reg<out_val_p5::OUT_VAL_P5_SPEC>;
#[doc = "Port P5 GPIO Output Value"]
pub mod out_val_p5;
#[doc = "OUT_VAL_P6 register accessor: an alias for `Reg<OUT_VAL_P6_SPEC>`"]
pub type OUT_VAL_P6 = crate::Reg<out_val_p6::OUT_VAL_P6_SPEC>;
#[doc = "Port P6 GPIO Output Value"]
pub mod out_val_p6;
#[doc = "OUT_VAL_P7 register accessor: an alias for `Reg<OUT_VAL_P7_SPEC>`"]
pub type OUT_VAL_P7 = crate::Reg<out_val_p7::OUT_VAL_P7_SPEC>;
#[doc = "Port P7 GPIO Output Value"]
pub mod out_val_p7;
#[doc = "OUT_VAL_P8 register accessor: an alias for `Reg<OUT_VAL_P8_SPEC>`"]
pub type OUT_VAL_P8 = crate::Reg<out_val_p8::OUT_VAL_P8_SPEC>;
#[doc = "Port P8 GPIO Output Value"]
pub mod out_val_p8;
#[doc = "FUNC_SEL_P0 register accessor: an alias for `Reg<FUNC_SEL_P0_SPEC>`"]
pub type FUNC_SEL_P0 = crate::Reg<func_sel_p0::FUNC_SEL_P0_SPEC>;
#[doc = "Port P0 GPIO Function Select"]
pub mod func_sel_p0;
#[doc = "FUNC_SEL_P1 register accessor: an alias for `Reg<FUNC_SEL_P1_SPEC>`"]
pub type FUNC_SEL_P1 = crate::Reg<func_sel_p1::FUNC_SEL_P1_SPEC>;
#[doc = "Port P1 GPIO Function Select"]
pub mod func_sel_p1;
#[doc = "FUNC_SEL_P2 register accessor: an alias for `Reg<FUNC_SEL_P2_SPEC>`"]
pub type FUNC_SEL_P2 = crate::Reg<func_sel_p2::FUNC_SEL_P2_SPEC>;
#[doc = "Port P2 GPIO Function Select"]
pub mod func_sel_p2;
#[doc = "FUNC_SEL_P3 register accessor: an alias for `Reg<FUNC_SEL_P3_SPEC>`"]
pub type FUNC_SEL_P3 = crate::Reg<func_sel_p3::FUNC_SEL_P3_SPEC>;
#[doc = "Port P3 GPIO Function Select"]
pub mod func_sel_p3;
#[doc = "FUNC_SEL_P4 register accessor: an alias for `Reg<FUNC_SEL_P4_SPEC>`"]
pub type FUNC_SEL_P4 = crate::Reg<func_sel_p4::FUNC_SEL_P4_SPEC>;
#[doc = "Port P4 GPIO Function Select"]
pub mod func_sel_p4;
#[doc = "FUNC_SEL_P5 register accessor: an alias for `Reg<FUNC_SEL_P5_SPEC>`"]
pub type FUNC_SEL_P5 = crate::Reg<func_sel_p5::FUNC_SEL_P5_SPEC>;
#[doc = "Port P5 GPIO Function Select"]
pub mod func_sel_p5;
#[doc = "FUNC_SEL_P6 register accessor: an alias for `Reg<FUNC_SEL_P6_SPEC>`"]
pub type FUNC_SEL_P6 = crate::Reg<func_sel_p6::FUNC_SEL_P6_SPEC>;
#[doc = "Port P6 GPIO Function Select"]
pub mod func_sel_p6;
#[doc = "FUNC_SEL_P7 register accessor: an alias for `Reg<FUNC_SEL_P7_SPEC>`"]
pub type FUNC_SEL_P7 = crate::Reg<func_sel_p7::FUNC_SEL_P7_SPEC>;
#[doc = "Port P7 GPIO Function Select"]
pub mod func_sel_p7;
#[doc = "FUNC_SEL_P8 register accessor: an alias for `Reg<FUNC_SEL_P8_SPEC>`"]
pub type FUNC_SEL_P8 = crate::Reg<func_sel_p8::FUNC_SEL_P8_SPEC>;
#[doc = "Port P8 GPIO Function Select"]
pub mod func_sel_p8;
#[doc = "IN_MODE_P0 register accessor: an alias for `Reg<IN_MODE_P0_SPEC>`"]
pub type IN_MODE_P0 = crate::Reg<in_mode_p0::IN_MODE_P0_SPEC>;
#[doc = "Port P0 GPIO Input Monitoring Mode"]
pub mod in_mode_p0;
#[doc = "IN_MODE_P1 register accessor: an alias for `Reg<IN_MODE_P1_SPEC>`"]
pub type IN_MODE_P1 = crate::Reg<in_mode_p1::IN_MODE_P1_SPEC>;
#[doc = "Port P1 GPIO Input Monitoring Mode"]
pub mod in_mode_p1;
#[doc = "IN_MODE_P2 register accessor: an alias for `Reg<IN_MODE_P2_SPEC>`"]
pub type IN_MODE_P2 = crate::Reg<in_mode_p2::IN_MODE_P2_SPEC>;
#[doc = "Port P2 GPIO Input Monitoring Mode"]
pub mod in_mode_p2;
#[doc = "IN_MODE_P3 register accessor: an alias for `Reg<IN_MODE_P3_SPEC>`"]
pub type IN_MODE_P3 = crate::Reg<in_mode_p3::IN_MODE_P3_SPEC>;
#[doc = "Port P3 GPIO Input Monitoring Mode"]
pub mod in_mode_p3;
#[doc = "IN_MODE_P4 register accessor: an alias for `Reg<IN_MODE_P4_SPEC>`"]
pub type IN_MODE_P4 = crate::Reg<in_mode_p4::IN_MODE_P4_SPEC>;
#[doc = "Port P4 GPIO Input Monitoring Mode"]
pub mod in_mode_p4;
#[doc = "IN_MODE_P5 register accessor: an alias for `Reg<IN_MODE_P5_SPEC>`"]
pub type IN_MODE_P5 = crate::Reg<in_mode_p5::IN_MODE_P5_SPEC>;
#[doc = "Port P5 GPIO Input Monitoring Mode"]
pub mod in_mode_p5;
#[doc = "IN_MODE_P6 register accessor: an alias for `Reg<IN_MODE_P6_SPEC>`"]
pub type IN_MODE_P6 = crate::Reg<in_mode_p6::IN_MODE_P6_SPEC>;
#[doc = "Port P6 GPIO Input Monitoring Mode"]
pub mod in_mode_p6;
#[doc = "IN_MODE_P7 register accessor: an alias for `Reg<IN_MODE_P7_SPEC>`"]
pub type IN_MODE_P7 = crate::Reg<in_mode_p7::IN_MODE_P7_SPEC>;
#[doc = "Port P7 GPIO Input Monitoring Mode"]
pub mod in_mode_p7;
#[doc = "IN_MODE_P8 register accessor: an alias for `Reg<IN_MODE_P8_SPEC>`"]
pub type IN_MODE_P8 = crate::Reg<in_mode_p8::IN_MODE_P8_SPEC>;
#[doc = "Port P8 GPIO Input Monitoring Mode"]
pub mod in_mode_p8;
#[doc = "IN_VAL_P0 register accessor: an alias for `Reg<IN_VAL_P0_SPEC>`"]
pub type IN_VAL_P0 = crate::Reg<in_val_p0::IN_VAL_P0_SPEC>;
#[doc = "Port P0 GPIO Input Value"]
pub mod in_val_p0;
#[doc = "IN_VAL_P1 register accessor: an alias for `Reg<IN_VAL_P1_SPEC>`"]
pub type IN_VAL_P1 = crate::Reg<in_val_p1::IN_VAL_P1_SPEC>;
#[doc = "Port P1 GPIO Input Value"]
pub mod in_val_p1;
#[doc = "IN_VAL_P2 register accessor: an alias for `Reg<IN_VAL_P2_SPEC>`"]
pub type IN_VAL_P2 = crate::Reg<in_val_p2::IN_VAL_P2_SPEC>;
#[doc = "Port P2 GPIO Input Value"]
pub mod in_val_p2;
#[doc = "IN_VAL_P3 register accessor: an alias for `Reg<IN_VAL_P3_SPEC>`"]
pub type IN_VAL_P3 = crate::Reg<in_val_p3::IN_VAL_P3_SPEC>;
#[doc = "Port P3 GPIO Input Value"]
pub mod in_val_p3;
#[doc = "IN_VAL_P4 register accessor: an alias for `Reg<IN_VAL_P4_SPEC>`"]
pub type IN_VAL_P4 = crate::Reg<in_val_p4::IN_VAL_P4_SPEC>;
#[doc = "Port P4 GPIO Input Value"]
pub mod in_val_p4;
#[doc = "IN_VAL_P5 register accessor: an alias for `Reg<IN_VAL_P5_SPEC>`"]
pub type IN_VAL_P5 = crate::Reg<in_val_p5::IN_VAL_P5_SPEC>;
#[doc = "Port P5 GPIO Input Value"]
pub mod in_val_p5;
#[doc = "IN_VAL_P6 register accessor: an alias for `Reg<IN_VAL_P6_SPEC>`"]
pub type IN_VAL_P6 = crate::Reg<in_val_p6::IN_VAL_P6_SPEC>;
#[doc = "Port P6 GPIO Input Value"]
pub mod in_val_p6;
#[doc = "IN_VAL_P7 register accessor: an alias for `Reg<IN_VAL_P7_SPEC>`"]
pub type IN_VAL_P7 = crate::Reg<in_val_p7::IN_VAL_P7_SPEC>;
#[doc = "Port P7 GPIO Input Value"]
pub mod in_val_p7;
#[doc = "IN_VAL_P8 register accessor: an alias for `Reg<IN_VAL_P8_SPEC>`"]
pub type IN_VAL_P8 = crate::Reg<in_val_p8::IN_VAL_P8_SPEC>;
#[doc = "Port P8 GPIO Input Value"]
pub mod in_val_p8;
#[doc = "INT_MODE_P0 register accessor: an alias for `Reg<INT_MODE_P0_SPEC>`"]
pub type INT_MODE_P0 = crate::Reg<int_mode_p0::INT_MODE_P0_SPEC>;
#[doc = "Port P0 Interrupt Detection Mode"]
pub mod int_mode_p0;
#[doc = "INT_MODE_P1 register accessor: an alias for `Reg<INT_MODE_P1_SPEC>`"]
pub type INT_MODE_P1 = crate::Reg<int_mode_p1::INT_MODE_P1_SPEC>;
#[doc = "Port P1 Interrupt Detection Mode"]
pub mod int_mode_p1;
#[doc = "INT_MODE_P2 register accessor: an alias for `Reg<INT_MODE_P2_SPEC>`"]
pub type INT_MODE_P2 = crate::Reg<int_mode_p2::INT_MODE_P2_SPEC>;
#[doc = "Port P2 Interrupt Detection Mode"]
pub mod int_mode_p2;
#[doc = "INT_MODE_P3 register accessor: an alias for `Reg<INT_MODE_P3_SPEC>`"]
pub type INT_MODE_P3 = crate::Reg<int_mode_p3::INT_MODE_P3_SPEC>;
#[doc = "Port P3 Interrupt Detection Mode"]
pub mod int_mode_p3;
#[doc = "INT_MODE_P4 register accessor: an alias for `Reg<INT_MODE_P4_SPEC>`"]
pub type INT_MODE_P4 = crate::Reg<int_mode_p4::INT_MODE_P4_SPEC>;
#[doc = "Port P4 Interrupt Detection Mode"]
pub mod int_mode_p4;
#[doc = "INT_MODE_P5 register accessor: an alias for `Reg<INT_MODE_P5_SPEC>`"]
pub type INT_MODE_P5 = crate::Reg<int_mode_p5::INT_MODE_P5_SPEC>;
#[doc = "Port P5 Interrupt Detection Mode"]
pub mod int_mode_p5;
#[doc = "INT_MODE_P6 register accessor: an alias for `Reg<INT_MODE_P6_SPEC>`"]
pub type INT_MODE_P6 = crate::Reg<int_mode_p6::INT_MODE_P6_SPEC>;
#[doc = "Port P6 Interrupt Detection Mode"]
pub mod int_mode_p6;
#[doc = "INT_MODE_P7 register accessor: an alias for `Reg<INT_MODE_P7_SPEC>`"]
pub type INT_MODE_P7 = crate::Reg<int_mode_p7::INT_MODE_P7_SPEC>;
#[doc = "Port P7 Interrupt Detection Mode"]
pub mod int_mode_p7;
#[doc = "INT_MODE_P8 register accessor: an alias for `Reg<INT_MODE_P8_SPEC>`"]
pub type INT_MODE_P8 = crate::Reg<int_mode_p8::INT_MODE_P8_SPEC>;
#[doc = "Port P8 Interrupt Detection Mode"]
pub mod int_mode_p8;
#[doc = "INTFL_P0 register accessor: an alias for `Reg<INTFL_P0_SPEC>`"]
pub type INTFL_P0 = crate::Reg<intfl_p0::INTFL_P0_SPEC>;
#[doc = "Port P0 Interrupt Flags"]
pub mod intfl_p0;
#[doc = "INTFL_P1 register accessor: an alias for `Reg<INTFL_P1_SPEC>`"]
pub type INTFL_P1 = crate::Reg<intfl_p1::INTFL_P1_SPEC>;
#[doc = "Port P1 Interrupt Flags"]
pub mod intfl_p1;
#[doc = "INTFL_P2 register accessor: an alias for `Reg<INTFL_P2_SPEC>`"]
pub type INTFL_P2 = crate::Reg<intfl_p2::INTFL_P2_SPEC>;
#[doc = "Port P2 Interrupt Flags"]
pub mod intfl_p2;
#[doc = "INTFL_P3 register accessor: an alias for `Reg<INTFL_P3_SPEC>`"]
pub type INTFL_P3 = crate::Reg<intfl_p3::INTFL_P3_SPEC>;
#[doc = "Port P3 Interrupt Flags"]
pub mod intfl_p3;
#[doc = "INTFL_P4 register accessor: an alias for `Reg<INTFL_P4_SPEC>`"]
pub type INTFL_P4 = crate::Reg<intfl_p4::INTFL_P4_SPEC>;
#[doc = "Port P4 Interrupt Flags"]
pub mod intfl_p4;
#[doc = "INTFL_P5 register accessor: an alias for `Reg<INTFL_P5_SPEC>`"]
pub type INTFL_P5 = crate::Reg<intfl_p5::INTFL_P5_SPEC>;
#[doc = "Port P5 Interrupt Flags"]
pub mod intfl_p5;
#[doc = "INTFL_P6 register accessor: an alias for `Reg<INTFL_P6_SPEC>`"]
pub type INTFL_P6 = crate::Reg<intfl_p6::INTFL_P6_SPEC>;
#[doc = "Port P6 Interrupt Flags"]
pub mod intfl_p6;
#[doc = "INTFL_P7 register accessor: an alias for `Reg<INTFL_P7_SPEC>`"]
pub type INTFL_P7 = crate::Reg<intfl_p7::INTFL_P7_SPEC>;
#[doc = "Port P7 Interrupt Flags"]
pub mod intfl_p7;
#[doc = "INTFL_P8 register accessor: an alias for `Reg<INTFL_P8_SPEC>`"]
pub type INTFL_P8 = crate::Reg<intfl_p8::INTFL_P8_SPEC>;
#[doc = "Port P8 Interrupt Flags"]
pub mod intfl_p8;
#[doc = "INTEN_P0 register accessor: an alias for `Reg<INTEN_P0_SPEC>`"]
pub type INTEN_P0 = crate::Reg<inten_p0::INTEN_P0_SPEC>;
#[doc = "Port P0 Interrupt Enables"]
pub mod inten_p0;
#[doc = "INTEN_P1 register accessor: an alias for `Reg<INTEN_P1_SPEC>`"]
pub type INTEN_P1 = crate::Reg<inten_p1::INTEN_P1_SPEC>;
#[doc = "Port P1 Interrupt Enables"]
pub mod inten_p1;
#[doc = "INTEN_P2 register accessor: an alias for `Reg<INTEN_P2_SPEC>`"]
pub type INTEN_P2 = crate::Reg<inten_p2::INTEN_P2_SPEC>;
#[doc = "Port P2 Interrupt Enables"]
pub mod inten_p2;
#[doc = "INTEN_P3 register accessor: an alias for `Reg<INTEN_P3_SPEC>`"]
pub type INTEN_P3 = crate::Reg<inten_p3::INTEN_P3_SPEC>;
#[doc = "Port P3 Interrupt Enables"]
pub mod inten_p3;
#[doc = "INTEN_P4 register accessor: an alias for `Reg<INTEN_P4_SPEC>`"]
pub type INTEN_P4 = crate::Reg<inten_p4::INTEN_P4_SPEC>;
#[doc = "Port P4 Interrupt Enables"]
pub mod inten_p4;
#[doc = "INTEN_P5 register accessor: an alias for `Reg<INTEN_P5_SPEC>`"]
pub type INTEN_P5 = crate::Reg<inten_p5::INTEN_P5_SPEC>;
#[doc = "Port P5 Interrupt Enables"]
pub mod inten_p5;
#[doc = "INTEN_P6 register accessor: an alias for `Reg<INTEN_P6_SPEC>`"]
pub type INTEN_P6 = crate::Reg<inten_p6::INTEN_P6_SPEC>;
#[doc = "Port P6 Interrupt Enables"]
pub mod inten_p6;
#[doc = "INTEN_P7 register accessor: an alias for `Reg<INTEN_P7_SPEC>`"]
pub type INTEN_P7 = crate::Reg<inten_p7::INTEN_P7_SPEC>;
#[doc = "Port P7 Interrupt Enables"]
pub mod inten_p7;
#[doc = "INTEN_P8 register accessor: an alias for `Reg<INTEN_P8_SPEC>`"]
pub type INTEN_P8 = crate::Reg<inten_p8::INTEN_P8_SPEC>;
#[doc = "Port P8 Interrupt Enables"]
pub mod inten_p8;
