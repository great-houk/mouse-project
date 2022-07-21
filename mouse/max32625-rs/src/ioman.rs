#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Wakeup Detect Mode Request Register 0 (P0/P1/P2/P3)"]
    pub wud_req0: crate::Reg<wud_req0::WUD_REQ0_SPEC>,
    #[doc = "0x04 - Wakeup Detect Mode Request Register 1 (P4/P5/P6)"]
    pub wud_req1: crate::Reg<wud_req1::WUD_REQ1_SPEC>,
    #[doc = "0x08 - Wakeup Detect Mode Acknowledge Register 0 (P0/P1/P2/P3)"]
    pub wud_ack0: crate::Reg<wud_ack0::WUD_ACK0_SPEC>,
    #[doc = "0x0c - Wakeup Detect Mode Acknowledge Register 1 (P4/P5/P6)"]
    pub wud_ack1: crate::Reg<wud_ack1::WUD_ACK1_SPEC>,
    #[doc = "0x10 - Analog Input Request Register 0 (P0/P1/P2/P3)"]
    pub ali_req0: crate::Reg<ali_req0::ALI_REQ0_SPEC>,
    #[doc = "0x14 - Analog Input Request Register 1 (P4/P5/P6)"]
    pub ali_req1: crate::Reg<ali_req1::ALI_REQ1_SPEC>,
    #[doc = "0x18 - Analog Input Acknowledge Register 0 (P0/P1/P2/P3)"]
    pub ali_ack0: crate::Reg<ali_ack0::ALI_ACK0_SPEC>,
    #[doc = "0x1c - Analog Input Acknowledge Register 1 (P4/P5/P6)"]
    pub ali_ack1: crate::Reg<ali_ack1::ALI_ACK1_SPEC>,
    #[doc = "0x20 - Analog I/O Connection Control Register 0"]
    pub ali_connect0: crate::Reg<ali_connect0::ALI_CONNECT0_SPEC>,
    #[doc = "0x24 - Analog I/O Connection Control Register 1"]
    pub ali_connect1: crate::Reg<ali_connect1::ALI_CONNECT1_SPEC>,
    #[doc = "0x28 - SPIX I/O Mode Request"]
    pub spix_req: crate::Reg<spix_req::SPIX_REQ_SPEC>,
    #[doc = "0x2c - SPIX I/O Mode Acknowledge"]
    pub spix_ack: crate::Reg<spix_ack::SPIX_ACK_SPEC>,
    #[doc = "0x30 - UART0 I/O Mode Request"]
    pub uart0_req: crate::Reg<uart0_req::UART0_REQ_SPEC>,
    #[doc = "0x34 - UART0 I/O Mode Acknowledge"]
    pub uart0_ack: crate::Reg<uart0_ack::UART0_ACK_SPEC>,
    #[doc = "0x38 - UART1 I/O Mode Request"]
    pub uart1_req: crate::Reg<uart1_req::UART1_REQ_SPEC>,
    #[doc = "0x3c - UART1 I/O Mode Acknowledge"]
    pub uart1_ack: crate::Reg<uart1_ack::UART1_ACK_SPEC>,
    #[doc = "0x40 - UART2 I/O Mode Request"]
    pub uart2_req: crate::Reg<uart2_req::UART2_REQ_SPEC>,
    #[doc = "0x44 - UART2 I/O Mode Acknowledge"]
    pub uart2_ack: crate::Reg<uart2_ack::UART2_ACK_SPEC>,
    #[doc = "0x48 - UART3 I/O Mode Request"]
    pub uart3_req: crate::Reg<uart3_req::UART3_REQ_SPEC>,
    #[doc = "0x4c - UART3 I/O Mode Acknowledge"]
    pub uart3_ack: crate::Reg<uart3_ack::UART3_ACK_SPEC>,
    #[doc = "0x50 - I2C Master 0 I/O Request"]
    pub i2cm0_req: crate::Reg<i2cm0_req::I2CM0_REQ_SPEC>,
    #[doc = "0x54 - I2C Master 0 I/O Acknowledge"]
    pub i2cm0_ack: crate::Reg<i2cm0_ack::I2CM0_ACK_SPEC>,
    #[doc = "0x58 - I2C Master 1 I/O Request"]
    pub i2cm1_req: crate::Reg<i2cm1_req::I2CM1_REQ_SPEC>,
    #[doc = "0x5c - I2C Master 1 I/O Acknowledge"]
    pub i2cm1_ack: crate::Reg<i2cm1_ack::I2CM1_ACK_SPEC>,
    #[doc = "0x60 - I2C Master 2 I/O Request"]
    pub i2cm2_req: crate::Reg<i2cm2_req::I2CM2_REQ_SPEC>,
    #[doc = "0x64 - I2C Master 2 I/O Acknowledge"]
    pub i2cm2_ack: crate::Reg<i2cm2_ack::I2CM2_ACK_SPEC>,
    #[doc = "0x68 - I2C Slave I/O Request"]
    pub i2cs_req: crate::Reg<i2cs_req::I2CS_REQ_SPEC>,
    #[doc = "0x6c - I2C Slave I/O Acknowledge"]
    pub i2cs_ack: crate::Reg<i2cs_ack::I2CS_ACK_SPEC>,
    #[doc = "0x70 - SPI Master 0 I/O Mode Request"]
    pub spi0_req: crate::Reg<spi0_req::SPI0_REQ_SPEC>,
    #[doc = "0x74 - SPI Master 0 I/O Mode Acknowledge"]
    pub spi0_ack: crate::Reg<spi0_ack::SPI0_ACK_SPEC>,
    #[doc = "0x78 - SPI Master 1 I/O Mode Request"]
    pub spi1_req: crate::Reg<spi1_req::SPI1_REQ_SPEC>,
    #[doc = "0x7c - SPI Master 1 I/O Mode Acknowledge"]
    pub spi1_ack: crate::Reg<spi1_ack::SPI1_ACK_SPEC>,
    #[doc = "0x80 - SPI Master 2 I/O Mode Request"]
    pub spi2_req: crate::Reg<spi2_req::SPI2_REQ_SPEC>,
    #[doc = "0x84 - SPI Master 2 I/O Mode Acknowledge"]
    pub spi2_ack: crate::Reg<spi2_ack::SPI2_ACK_SPEC>,
    #[doc = "0x88 - SPI Bridge I/O Mode Request"]
    pub spib_req: crate::Reg<spib_req::SPIB_REQ_SPEC>,
    #[doc = "0x8c - SPI Bridge I/O Mode Acknowledge"]
    pub spib_ack: crate::Reg<spib_ack::SPIB_ACK_SPEC>,
    #[doc = "0x90 - 1-Wire Master I/O Mode Request"]
    pub owm_req: crate::Reg<owm_req::OWM_REQ_SPEC>,
    #[doc = "0x94 - 1-Wire Master I/O Mode Acknowledge"]
    pub owm_ack: crate::Reg<owm_ack::OWM_ACK_SPEC>,
}
#[doc = "WUD_REQ0 register accessor: an alias for `Reg<WUD_REQ0_SPEC>`"]
pub type WUD_REQ0 = crate::Reg<wud_req0::WUD_REQ0_SPEC>;
#[doc = "Wakeup Detect Mode Request Register 0 (P0/P1/P2/P3)"]
pub mod wud_req0;
#[doc = "WUD_REQ1 register accessor: an alias for `Reg<WUD_REQ1_SPEC>`"]
pub type WUD_REQ1 = crate::Reg<wud_req1::WUD_REQ1_SPEC>;
#[doc = "Wakeup Detect Mode Request Register 1 (P4/P5/P6)"]
pub mod wud_req1;
#[doc = "WUD_ACK0 register accessor: an alias for `Reg<WUD_ACK0_SPEC>`"]
pub type WUD_ACK0 = crate::Reg<wud_ack0::WUD_ACK0_SPEC>;
#[doc = "Wakeup Detect Mode Acknowledge Register 0 (P0/P1/P2/P3)"]
pub mod wud_ack0;
#[doc = "WUD_ACK1 register accessor: an alias for `Reg<WUD_ACK1_SPEC>`"]
pub type WUD_ACK1 = crate::Reg<wud_ack1::WUD_ACK1_SPEC>;
#[doc = "Wakeup Detect Mode Acknowledge Register 1 (P4/P5/P6)"]
pub mod wud_ack1;
#[doc = "ALI_REQ0 register accessor: an alias for `Reg<ALI_REQ0_SPEC>`"]
pub type ALI_REQ0 = crate::Reg<ali_req0::ALI_REQ0_SPEC>;
#[doc = "Analog Input Request Register 0 (P0/P1/P2/P3)"]
pub mod ali_req0;
#[doc = "ALI_REQ1 register accessor: an alias for `Reg<ALI_REQ1_SPEC>`"]
pub type ALI_REQ1 = crate::Reg<ali_req1::ALI_REQ1_SPEC>;
#[doc = "Analog Input Request Register 1 (P4/P5/P6)"]
pub mod ali_req1;
#[doc = "ALI_ACK0 register accessor: an alias for `Reg<ALI_ACK0_SPEC>`"]
pub type ALI_ACK0 = crate::Reg<ali_ack0::ALI_ACK0_SPEC>;
#[doc = "Analog Input Acknowledge Register 0 (P0/P1/P2/P3)"]
pub mod ali_ack0;
#[doc = "ALI_ACK1 register accessor: an alias for `Reg<ALI_ACK1_SPEC>`"]
pub type ALI_ACK1 = crate::Reg<ali_ack1::ALI_ACK1_SPEC>;
#[doc = "Analog Input Acknowledge Register 1 (P4/P5/P6)"]
pub mod ali_ack1;
#[doc = "ALI_CONNECT0 register accessor: an alias for `Reg<ALI_CONNECT0_SPEC>`"]
pub type ALI_CONNECT0 = crate::Reg<ali_connect0::ALI_CONNECT0_SPEC>;
#[doc = "Analog I/O Connection Control Register 0"]
pub mod ali_connect0;
#[doc = "ALI_CONNECT1 register accessor: an alias for `Reg<ALI_CONNECT1_SPEC>`"]
pub type ALI_CONNECT1 = crate::Reg<ali_connect1::ALI_CONNECT1_SPEC>;
#[doc = "Analog I/O Connection Control Register 1"]
pub mod ali_connect1;
#[doc = "SPIX_REQ register accessor: an alias for `Reg<SPIX_REQ_SPEC>`"]
pub type SPIX_REQ = crate::Reg<spix_req::SPIX_REQ_SPEC>;
#[doc = "SPIX I/O Mode Request"]
pub mod spix_req;
#[doc = "SPIX_ACK register accessor: an alias for `Reg<SPIX_ACK_SPEC>`"]
pub type SPIX_ACK = crate::Reg<spix_ack::SPIX_ACK_SPEC>;
#[doc = "SPIX I/O Mode Acknowledge"]
pub mod spix_ack;
#[doc = "UART0_REQ register accessor: an alias for `Reg<UART0_REQ_SPEC>`"]
pub type UART0_REQ = crate::Reg<uart0_req::UART0_REQ_SPEC>;
#[doc = "UART0 I/O Mode Request"]
pub mod uart0_req;
#[doc = "UART0_ACK register accessor: an alias for `Reg<UART0_ACK_SPEC>`"]
pub type UART0_ACK = crate::Reg<uart0_ack::UART0_ACK_SPEC>;
#[doc = "UART0 I/O Mode Acknowledge"]
pub mod uart0_ack;
#[doc = "UART1_REQ register accessor: an alias for `Reg<UART1_REQ_SPEC>`"]
pub type UART1_REQ = crate::Reg<uart1_req::UART1_REQ_SPEC>;
#[doc = "UART1 I/O Mode Request"]
pub mod uart1_req;
#[doc = "UART1_ACK register accessor: an alias for `Reg<UART1_ACK_SPEC>`"]
pub type UART1_ACK = crate::Reg<uart1_ack::UART1_ACK_SPEC>;
#[doc = "UART1 I/O Mode Acknowledge"]
pub mod uart1_ack;
#[doc = "UART2_REQ register accessor: an alias for `Reg<UART2_REQ_SPEC>`"]
pub type UART2_REQ = crate::Reg<uart2_req::UART2_REQ_SPEC>;
#[doc = "UART2 I/O Mode Request"]
pub mod uart2_req;
#[doc = "UART2_ACK register accessor: an alias for `Reg<UART2_ACK_SPEC>`"]
pub type UART2_ACK = crate::Reg<uart2_ack::UART2_ACK_SPEC>;
#[doc = "UART2 I/O Mode Acknowledge"]
pub mod uart2_ack;
#[doc = "UART3_REQ register accessor: an alias for `Reg<UART3_REQ_SPEC>`"]
pub type UART3_REQ = crate::Reg<uart3_req::UART3_REQ_SPEC>;
#[doc = "UART3 I/O Mode Request"]
pub mod uart3_req;
#[doc = "UART3_ACK register accessor: an alias for `Reg<UART3_ACK_SPEC>`"]
pub type UART3_ACK = crate::Reg<uart3_ack::UART3_ACK_SPEC>;
#[doc = "UART3 I/O Mode Acknowledge"]
pub mod uart3_ack;
#[doc = "I2CM0_REQ register accessor: an alias for `Reg<I2CM0_REQ_SPEC>`"]
pub type I2CM0_REQ = crate::Reg<i2cm0_req::I2CM0_REQ_SPEC>;
#[doc = "I2C Master 0 I/O Request"]
pub mod i2cm0_req;
#[doc = "I2CM0_ACK register accessor: an alias for `Reg<I2CM0_ACK_SPEC>`"]
pub type I2CM0_ACK = crate::Reg<i2cm0_ack::I2CM0_ACK_SPEC>;
#[doc = "I2C Master 0 I/O Acknowledge"]
pub mod i2cm0_ack;
#[doc = "I2CM1_REQ register accessor: an alias for `Reg<I2CM1_REQ_SPEC>`"]
pub type I2CM1_REQ = crate::Reg<i2cm1_req::I2CM1_REQ_SPEC>;
#[doc = "I2C Master 1 I/O Request"]
pub mod i2cm1_req;
#[doc = "I2CM1_ACK register accessor: an alias for `Reg<I2CM1_ACK_SPEC>`"]
pub type I2CM1_ACK = crate::Reg<i2cm1_ack::I2CM1_ACK_SPEC>;
#[doc = "I2C Master 1 I/O Acknowledge"]
pub mod i2cm1_ack;
#[doc = "I2CM2_REQ register accessor: an alias for `Reg<I2CM2_REQ_SPEC>`"]
pub type I2CM2_REQ = crate::Reg<i2cm2_req::I2CM2_REQ_SPEC>;
#[doc = "I2C Master 2 I/O Request"]
pub mod i2cm2_req;
#[doc = "I2CM2_ACK register accessor: an alias for `Reg<I2CM2_ACK_SPEC>`"]
pub type I2CM2_ACK = crate::Reg<i2cm2_ack::I2CM2_ACK_SPEC>;
#[doc = "I2C Master 2 I/O Acknowledge"]
pub mod i2cm2_ack;
#[doc = "I2CS_REQ register accessor: an alias for `Reg<I2CS_REQ_SPEC>`"]
pub type I2CS_REQ = crate::Reg<i2cs_req::I2CS_REQ_SPEC>;
#[doc = "I2C Slave I/O Request"]
pub mod i2cs_req;
#[doc = "I2CS_ACK register accessor: an alias for `Reg<I2CS_ACK_SPEC>`"]
pub type I2CS_ACK = crate::Reg<i2cs_ack::I2CS_ACK_SPEC>;
#[doc = "I2C Slave I/O Acknowledge"]
pub mod i2cs_ack;
#[doc = "SPI0_REQ register accessor: an alias for `Reg<SPI0_REQ_SPEC>`"]
pub type SPI0_REQ = crate::Reg<spi0_req::SPI0_REQ_SPEC>;
#[doc = "SPI Master 0 I/O Mode Request"]
pub mod spi0_req;
#[doc = "SPI0_ACK register accessor: an alias for `Reg<SPI0_ACK_SPEC>`"]
pub type SPI0_ACK = crate::Reg<spi0_ack::SPI0_ACK_SPEC>;
#[doc = "SPI Master 0 I/O Mode Acknowledge"]
pub mod spi0_ack;
#[doc = "SPI1_REQ register accessor: an alias for `Reg<SPI1_REQ_SPEC>`"]
pub type SPI1_REQ = crate::Reg<spi1_req::SPI1_REQ_SPEC>;
#[doc = "SPI Master 1 I/O Mode Request"]
pub mod spi1_req;
#[doc = "SPI1_ACK register accessor: an alias for `Reg<SPI1_ACK_SPEC>`"]
pub type SPI1_ACK = crate::Reg<spi1_ack::SPI1_ACK_SPEC>;
#[doc = "SPI Master 1 I/O Mode Acknowledge"]
pub mod spi1_ack;
#[doc = "SPI2_REQ register accessor: an alias for `Reg<SPI2_REQ_SPEC>`"]
pub type SPI2_REQ = crate::Reg<spi2_req::SPI2_REQ_SPEC>;
#[doc = "SPI Master 2 I/O Mode Request"]
pub mod spi2_req;
#[doc = "SPI2_ACK register accessor: an alias for `Reg<SPI2_ACK_SPEC>`"]
pub type SPI2_ACK = crate::Reg<spi2_ack::SPI2_ACK_SPEC>;
#[doc = "SPI Master 2 I/O Mode Acknowledge"]
pub mod spi2_ack;
#[doc = "SPIB_REQ register accessor: an alias for `Reg<SPIB_REQ_SPEC>`"]
pub type SPIB_REQ = crate::Reg<spib_req::SPIB_REQ_SPEC>;
#[doc = "SPI Bridge I/O Mode Request"]
pub mod spib_req;
#[doc = "SPIB_ACK register accessor: an alias for `Reg<SPIB_ACK_SPEC>`"]
pub type SPIB_ACK = crate::Reg<spib_ack::SPIB_ACK_SPEC>;
#[doc = "SPI Bridge I/O Mode Acknowledge"]
pub mod spib_ack;
#[doc = "OWM_REQ register accessor: an alias for `Reg<OWM_REQ_SPEC>`"]
pub type OWM_REQ = crate::Reg<owm_req::OWM_REQ_SPEC>;
#[doc = "1-Wire Master I/O Mode Request"]
pub mod owm_req;
#[doc = "OWM_ACK register accessor: an alias for `Reg<OWM_ACK_SPEC>`"]
pub type OWM_ACK = crate::Reg<owm_ack::OWM_ACK_SPEC>;
#[doc = "1-Wire Master I/O Mode Acknowledge"]
pub mod owm_ack;
