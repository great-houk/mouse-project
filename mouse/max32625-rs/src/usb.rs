#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Control Register"]
    pub cn: crate::Reg<cn::CN_SPEC>,
    _reserved1: [u8; 0x01fc],
    #[doc = "0x200 - USB Device Address Register"]
    pub dev_addr: crate::Reg<dev_addr::DEV_ADDR_SPEC>,
    #[doc = "0x204 - USB Device Control Register"]
    pub dev_cn: crate::Reg<dev_cn::DEV_CN_SPEC>,
    #[doc = "0x208 - USB Device Interrupt"]
    pub dev_intfl: crate::Reg<dev_intfl::DEV_INTFL_SPEC>,
    #[doc = "0x20c - USB Device Interrupt Enable"]
    pub dev_inten: crate::Reg<dev_inten::DEV_INTEN_SPEC>,
    _reserved5: [u8; 0x10],
    #[doc = "0x220 - USB Endpoint Descriptor Table Base Address"]
    pub ep_base: crate::Reg<ep_base::EP_BASE_SPEC>,
    #[doc = "0x224 - USB Current Endpoint Buffer Register"]
    pub cur_buf: crate::Reg<cur_buf::CUR_BUF_SPEC>,
    #[doc = "0x228 - USB IN Endpoint Buffer Owner Register"]
    pub in_owner: crate::Reg<in_owner::IN_OWNER_SPEC>,
    #[doc = "0x22c - USB OUT Endpoint Buffer Owner Register"]
    pub out_owner: crate::Reg<out_owner::OUT_OWNER_SPEC>,
    #[doc = "0x230 - USB IN Endpoint Buffer Available Interrupt"]
    pub in_int: crate::Reg<in_int::IN_INT_SPEC>,
    #[doc = "0x234 - USB OUT Endpoint Data Available Interrupt"]
    pub out_int: crate::Reg<out_int::OUT_INT_SPEC>,
    #[doc = "0x238 - USB IN Endpoint NAK Interrupt"]
    pub nak_int: crate::Reg<nak_int::NAK_INT_SPEC>,
    #[doc = "0x23c - USB DMA Error Interrupt"]
    pub dma_err_int: crate::Reg<dma_err_int::DMA_ERR_INT_SPEC>,
    #[doc = "0x240 - USB Buffer Overflow Interrupt"]
    pub buf_ovr_int: crate::Reg<buf_ovr_int::BUF_OVR_INT_SPEC>,
    _reserved14: [u8; 0x1c],
    #[doc = "0x260 - USB SETUP Packet Bytes 0 to 3"]
    pub setup0: crate::Reg<setup0::SETUP0_SPEC>,
    #[doc = "0x264 - USB SETUP Packet Bytes 4 to 7"]
    pub setup1: crate::Reg<setup1::SETUP1_SPEC>,
    _reserved16: [u8; 0x18],
    #[doc = "0x280 - USB Endpoint 0 Control Register"]
    pub ep0: crate::Reg<ep0::EP0_SPEC>,
    #[doc = "0x284 - USB Endpoint 1 Control Register"]
    pub ep1: crate::Reg<ep1::EP1_SPEC>,
    #[doc = "0x288 - USB Endpoint 2 Control Register"]
    pub ep2: crate::Reg<ep2::EP2_SPEC>,
    #[doc = "0x28c - USB Endpoint 3 Control Register"]
    pub ep3: crate::Reg<ep3::EP3_SPEC>,
    #[doc = "0x290 - USB Endpoint 4 Control Register"]
    pub ep4: crate::Reg<ep4::EP4_SPEC>,
    #[doc = "0x294 - USB Endpoint 5 Control Register"]
    pub ep5: crate::Reg<ep5::EP5_SPEC>,
    #[doc = "0x298 - USB Endpoint 6 Control Register"]
    pub ep6: crate::Reg<ep6::EP6_SPEC>,
    #[doc = "0x29c - USB Endpoint 7 Control Register"]
    pub ep7: crate::Reg<ep7::EP7_SPEC>,
}
#[doc = "CN register accessor: an alias for `Reg<CN_SPEC>`"]
pub type CN = crate::Reg<cn::CN_SPEC>;
#[doc = "USB Control Register"]
pub mod cn;
#[doc = "DEV_ADDR register accessor: an alias for `Reg<DEV_ADDR_SPEC>`"]
pub type DEV_ADDR = crate::Reg<dev_addr::DEV_ADDR_SPEC>;
#[doc = "USB Device Address Register"]
pub mod dev_addr;
#[doc = "DEV_CN register accessor: an alias for `Reg<DEV_CN_SPEC>`"]
pub type DEV_CN = crate::Reg<dev_cn::DEV_CN_SPEC>;
#[doc = "USB Device Control Register"]
pub mod dev_cn;
#[doc = "DEV_INTFL register accessor: an alias for `Reg<DEV_INTFL_SPEC>`"]
pub type DEV_INTFL = crate::Reg<dev_intfl::DEV_INTFL_SPEC>;
#[doc = "USB Device Interrupt"]
pub mod dev_intfl;
#[doc = "DEV_INTEN register accessor: an alias for `Reg<DEV_INTEN_SPEC>`"]
pub type DEV_INTEN = crate::Reg<dev_inten::DEV_INTEN_SPEC>;
#[doc = "USB Device Interrupt Enable"]
pub mod dev_inten;
#[doc = "EP_BASE register accessor: an alias for `Reg<EP_BASE_SPEC>`"]
pub type EP_BASE = crate::Reg<ep_base::EP_BASE_SPEC>;
#[doc = "USB Endpoint Descriptor Table Base Address"]
pub mod ep_base;
#[doc = "CUR_BUF register accessor: an alias for `Reg<CUR_BUF_SPEC>`"]
pub type CUR_BUF = crate::Reg<cur_buf::CUR_BUF_SPEC>;
#[doc = "USB Current Endpoint Buffer Register"]
pub mod cur_buf;
#[doc = "IN_OWNER register accessor: an alias for `Reg<IN_OWNER_SPEC>`"]
pub type IN_OWNER = crate::Reg<in_owner::IN_OWNER_SPEC>;
#[doc = "USB IN Endpoint Buffer Owner Register"]
pub mod in_owner;
#[doc = "OUT_OWNER register accessor: an alias for `Reg<OUT_OWNER_SPEC>`"]
pub type OUT_OWNER = crate::Reg<out_owner::OUT_OWNER_SPEC>;
#[doc = "USB OUT Endpoint Buffer Owner Register"]
pub mod out_owner;
#[doc = "IN_INT register accessor: an alias for `Reg<IN_INT_SPEC>`"]
pub type IN_INT = crate::Reg<in_int::IN_INT_SPEC>;
#[doc = "USB IN Endpoint Buffer Available Interrupt"]
pub mod in_int;
#[doc = "OUT_INT register accessor: an alias for `Reg<OUT_INT_SPEC>`"]
pub type OUT_INT = crate::Reg<out_int::OUT_INT_SPEC>;
#[doc = "USB OUT Endpoint Data Available Interrupt"]
pub mod out_int;
#[doc = "NAK_INT register accessor: an alias for `Reg<NAK_INT_SPEC>`"]
pub type NAK_INT = crate::Reg<nak_int::NAK_INT_SPEC>;
#[doc = "USB IN Endpoint NAK Interrupt"]
pub mod nak_int;
#[doc = "DMA_ERR_INT register accessor: an alias for `Reg<DMA_ERR_INT_SPEC>`"]
pub type DMA_ERR_INT = crate::Reg<dma_err_int::DMA_ERR_INT_SPEC>;
#[doc = "USB DMA Error Interrupt"]
pub mod dma_err_int;
#[doc = "BUF_OVR_INT register accessor: an alias for `Reg<BUF_OVR_INT_SPEC>`"]
pub type BUF_OVR_INT = crate::Reg<buf_ovr_int::BUF_OVR_INT_SPEC>;
#[doc = "USB Buffer Overflow Interrupt"]
pub mod buf_ovr_int;
#[doc = "SETUP0 register accessor: an alias for `Reg<SETUP0_SPEC>`"]
pub type SETUP0 = crate::Reg<setup0::SETUP0_SPEC>;
#[doc = "USB SETUP Packet Bytes 0 to 3"]
pub mod setup0;
#[doc = "SETUP1 register accessor: an alias for `Reg<SETUP1_SPEC>`"]
pub type SETUP1 = crate::Reg<setup1::SETUP1_SPEC>;
#[doc = "USB SETUP Packet Bytes 4 to 7"]
pub mod setup1;
#[doc = "EP0 register accessor: an alias for `Reg<EP0_SPEC>`"]
pub type EP0 = crate::Reg<ep0::EP0_SPEC>;
#[doc = "USB Endpoint 0 Control Register"]
pub mod ep0;
#[doc = "EP1 register accessor: an alias for `Reg<EP1_SPEC>`"]
pub type EP1 = crate::Reg<ep1::EP1_SPEC>;
#[doc = "USB Endpoint 1 Control Register"]
pub mod ep1;
#[doc = "EP2 register accessor: an alias for `Reg<EP2_SPEC>`"]
pub type EP2 = crate::Reg<ep2::EP2_SPEC>;
#[doc = "USB Endpoint 2 Control Register"]
pub mod ep2;
#[doc = "EP3 register accessor: an alias for `Reg<EP3_SPEC>`"]
pub type EP3 = crate::Reg<ep3::EP3_SPEC>;
#[doc = "USB Endpoint 3 Control Register"]
pub mod ep3;
#[doc = "EP4 register accessor: an alias for `Reg<EP4_SPEC>`"]
pub type EP4 = crate::Reg<ep4::EP4_SPEC>;
#[doc = "USB Endpoint 4 Control Register"]
pub mod ep4;
#[doc = "EP5 register accessor: an alias for `Reg<EP5_SPEC>`"]
pub type EP5 = crate::Reg<ep5::EP5_SPEC>;
#[doc = "USB Endpoint 5 Control Register"]
pub mod ep5;
#[doc = "EP6 register accessor: an alias for `Reg<EP6_SPEC>`"]
pub type EP6 = crate::Reg<ep6::EP6_SPEC>;
#[doc = "USB Endpoint 6 Control Register"]
pub mod ep6;
#[doc = "EP7 register accessor: an alias for `Reg<EP7_SPEC>`"]
pub type EP7 = crate::Reg<ep7::EP7_SPEC>;
#[doc = "USB Endpoint 7 Control Register"]
pub mod ep7;
