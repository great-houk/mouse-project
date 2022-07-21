#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Clock Configuration"]
    pub clk_config: crate::Reg<clk_config::CLK_CONFIG_SPEC>,
    #[doc = "0x04 - System Clock Controls"]
    pub clk_ctrl: crate::Reg<clk_ctrl::CLK_CTRL_SPEC>,
    #[doc = "0x08 - Interrupt Flags"]
    pub intfl: crate::Reg<intfl::INTFL_SPEC>,
    #[doc = "0x0c - Interrupt Enable/Disable Controls"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x10 - Trim Calculation Controls"]
    pub trim_calc: crate::Reg<trim_calc::TRIM_CALC_SPEC>,
    #[doc = "0x14 - I2C Timer Control"]
    pub i2c_timer_ctrl: crate::Reg<i2c_timer_ctrl::I2C_TIMER_CTRL_SPEC>,
    #[doc = "0x18 - CM4 Start Clock on Interrupt Enable 0"]
    pub cm4_start_clk_en0: crate::Reg<cm4_start_clk_en0::CM4_START_CLK_EN0_SPEC>,
    #[doc = "0x1c - CM4 Start Clock on Interrupt Enable 1"]
    pub cm4_start_clk_en1: crate::Reg<cm4_start_clk_en1::CM4_START_CLK_EN1_SPEC>,
    #[doc = "0x20 - CM4 Start Clock on Interrupt Enable 2"]
    pub cm4_start_clk_en2: crate::Reg<cm4_start_clk_en2::CM4_START_CLK_EN2_SPEC>,
    _reserved9: [u8; 0x1c],
    #[doc = "0x40 - Control Settings for CLK0 - Cortex M4 Clock"]
    pub sys_clk_ctrl_0_cm4: crate::Reg<sys_clk_ctrl_0_cm4::SYS_CLK_CTRL_0_CM4_SPEC>,
    #[doc = "0x44 - Control Settings for CLK1 - Synchronizer Clock"]
    pub sys_clk_ctrl_1_sync: crate::Reg<sys_clk_ctrl_1_sync::SYS_CLK_CTRL_1_SYNC_SPEC>,
    #[doc = "0x48 - Control Settings for CLK2 - SPI XIP Clock"]
    pub sys_clk_ctrl_2_spix: crate::Reg<sys_clk_ctrl_2_spix::SYS_CLK_CTRL_2_SPIX_SPEC>,
    #[doc = "0x4c - Control Settings for CLK3 - PRNG Clock"]
    pub sys_clk_ctrl_3_prng: crate::Reg<sys_clk_ctrl_3_prng::SYS_CLK_CTRL_3_PRNG_SPEC>,
    #[doc = "0x50 - Control Settings for CLK4 - Watchdog Timer 0"]
    pub sys_clk_ctrl_4_wdt0: crate::Reg<sys_clk_ctrl_4_wdt0::SYS_CLK_CTRL_4_WDT0_SPEC>,
    #[doc = "0x54 - Control Settings for CLK5 - Watchdog Timer 1"]
    pub sys_clk_ctrl_5_wdt1: crate::Reg<sys_clk_ctrl_5_wdt1::SYS_CLK_CTRL_5_WDT1_SPEC>,
    #[doc = "0x58 - Control Settings for CLK6 - Clock for GPIO Ports"]
    pub sys_clk_ctrl_6_gpio: crate::Reg<sys_clk_ctrl_6_gpio::SYS_CLK_CTRL_6_GPIO_SPEC>,
    #[doc = "0x5c - Control Settings for CLK7 - Source Clock for All Pulse Trains"]
    pub sys_clk_ctrl_7_pt: crate::Reg<sys_clk_ctrl_7_pt::SYS_CLK_CTRL_7_PT_SPEC>,
    #[doc = "0x60 - Control Settings for CLK8 - Source Clock for All UARTs"]
    pub sys_clk_ctrl_8_uart: crate::Reg<sys_clk_ctrl_8_uart::SYS_CLK_CTRL_8_UART_SPEC>,
    #[doc = "0x64 - Control Settings for CLK9 - Source Clock for All I2C Masters"]
    pub sys_clk_ctrl_9_i2cm: crate::Reg<sys_clk_ctrl_9_i2cm::SYS_CLK_CTRL_9_I2CM_SPEC>,
    #[doc = "0x68 - Control Settings for CLK10 - Source Clock for I2C Slave"]
    pub sys_clk_ctrl_10_i2cs: crate::Reg<sys_clk_ctrl_10_i2cs::SYS_CLK_CTRL_10_I2CS_SPEC>,
    #[doc = "0x6c - Control Settings for CLK11 - SPI Master 0"]
    pub sys_clk_ctrl_11_spi0: crate::Reg<sys_clk_ctrl_11_spi0::SYS_CLK_CTRL_11_SPI0_SPEC>,
    #[doc = "0x70 - Control Settings for CLK12 - SPI Master 1"]
    pub sys_clk_ctrl_12_spi1: crate::Reg<sys_clk_ctrl_12_spi1::SYS_CLK_CTRL_12_SPI1_SPEC>,
    #[doc = "0x74 - Control Settings for CLK13 - SPI Master 2"]
    pub sys_clk_ctrl_13_spi2: crate::Reg<sys_clk_ctrl_13_spi2::SYS_CLK_CTRL_13_SPI2_SPEC>,
    #[doc = "0x78 - Control Settings for CLK14 - SPI Bridge Clock"]
    pub sys_clk_ctrl_14_spib: crate::Reg<sys_clk_ctrl_14_spib::SYS_CLK_CTRL_14_SPIB_SPEC>,
    #[doc = "0x7c - Control Settings for CLK15 - 1-Wire Master Clock"]
    pub sys_clk_ctrl_15_owm: crate::Reg<sys_clk_ctrl_15_owm::SYS_CLK_CTRL_15_OWM_SPEC>,
    #[doc = "0x80 - Control Settings for CLK16 - SPI Slave Clock"]
    pub sys_clk_ctrl_16_spis: crate::Reg<sys_clk_ctrl_16_spis::SYS_CLK_CTRL_16_SPIS_SPEC>,
    _reserved26: [u8; 0x7c],
    #[doc = "0x100 - Control Settings for Crypto Clock 0 - AES"]
    pub crypt_clk_ctrl_0_aes: crate::Reg<crypt_clk_ctrl_0_aes::CRYPT_CLK_CTRL_0_AES_SPEC>,
    #[doc = "0x104 - Control Settings for Crypto Clock 1 - MAA"]
    pub crypt_clk_ctrl_1_maa: crate::Reg<crypt_clk_ctrl_1_maa::CRYPT_CLK_CTRL_1_MAA_SPEC>,
    #[doc = "0x108 - Control Settings for Crypto Clock 2 - PRNG"]
    pub crypt_clk_ctrl_2_prng: crate::Reg<crypt_clk_ctrl_2_prng::CRYPT_CLK_CTRL_2_PRNG_SPEC>,
    _reserved29: [u8; 0x34],
    #[doc = "0x140 - Dynamic Clock Gating Control Register 0"]
    pub clk_gate_ctrl0: crate::Reg<clk_gate_ctrl0::CLK_GATE_CTRL0_SPEC>,
    #[doc = "0x144 - Dynamic Clock Gating Control Register 1"]
    pub clk_gate_ctrl1: crate::Reg<clk_gate_ctrl1::CLK_GATE_CTRL1_SPEC>,
    #[doc = "0x148 - Dynamic Clock Gating Control Register 2"]
    pub clk_gate_ctrl2: crate::Reg<clk_gate_ctrl2::CLK_GATE_CTRL2_SPEC>,
}
#[doc = "CLK_CONFIG register accessor: an alias for `Reg<CLK_CONFIG_SPEC>`"]
pub type CLK_CONFIG = crate::Reg<clk_config::CLK_CONFIG_SPEC>;
#[doc = "System Clock Configuration"]
pub mod clk_config;
#[doc = "CLK_CTRL register accessor: an alias for `Reg<CLK_CTRL_SPEC>`"]
pub type CLK_CTRL = crate::Reg<clk_ctrl::CLK_CTRL_SPEC>;
#[doc = "System Clock Controls"]
pub mod clk_ctrl;
#[doc = "INTFL register accessor: an alias for `Reg<INTFL_SPEC>`"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intfl;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable/Disable Controls"]
pub mod inten;
#[doc = "TRIM_CALC register accessor: an alias for `Reg<TRIM_CALC_SPEC>`"]
pub type TRIM_CALC = crate::Reg<trim_calc::TRIM_CALC_SPEC>;
#[doc = "Trim Calculation Controls"]
pub mod trim_calc;
#[doc = "I2C_TIMER_CTRL register accessor: an alias for `Reg<I2C_TIMER_CTRL_SPEC>`"]
pub type I2C_TIMER_CTRL = crate::Reg<i2c_timer_ctrl::I2C_TIMER_CTRL_SPEC>;
#[doc = "I2C Timer Control"]
pub mod i2c_timer_ctrl;
#[doc = "CM4_START_CLK_EN0 register accessor: an alias for `Reg<CM4_START_CLK_EN0_SPEC>`"]
pub type CM4_START_CLK_EN0 = crate::Reg<cm4_start_clk_en0::CM4_START_CLK_EN0_SPEC>;
#[doc = "CM4 Start Clock on Interrupt Enable 0"]
pub mod cm4_start_clk_en0;
#[doc = "CM4_START_CLK_EN1 register accessor: an alias for `Reg<CM4_START_CLK_EN1_SPEC>`"]
pub type CM4_START_CLK_EN1 = crate::Reg<cm4_start_clk_en1::CM4_START_CLK_EN1_SPEC>;
#[doc = "CM4 Start Clock on Interrupt Enable 1"]
pub mod cm4_start_clk_en1;
#[doc = "CM4_START_CLK_EN2 register accessor: an alias for `Reg<CM4_START_CLK_EN2_SPEC>`"]
pub type CM4_START_CLK_EN2 = crate::Reg<cm4_start_clk_en2::CM4_START_CLK_EN2_SPEC>;
#[doc = "CM4 Start Clock on Interrupt Enable 2"]
pub mod cm4_start_clk_en2;
#[doc = "SYS_CLK_CTRL_0_CM4 register accessor: an alias for `Reg<SYS_CLK_CTRL_0_CM4_SPEC>`"]
pub type SYS_CLK_CTRL_0_CM4 = crate::Reg<sys_clk_ctrl_0_cm4::SYS_CLK_CTRL_0_CM4_SPEC>;
#[doc = "Control Settings for CLK0 - Cortex M4 Clock"]
pub mod sys_clk_ctrl_0_cm4;
#[doc = "SYS_CLK_CTRL_1_SYNC register accessor: an alias for `Reg<SYS_CLK_CTRL_1_SYNC_SPEC>`"]
pub type SYS_CLK_CTRL_1_SYNC = crate::Reg<sys_clk_ctrl_1_sync::SYS_CLK_CTRL_1_SYNC_SPEC>;
#[doc = "Control Settings for CLK1 - Synchronizer Clock"]
pub mod sys_clk_ctrl_1_sync;
#[doc = "SYS_CLK_CTRL_2_SPIX register accessor: an alias for `Reg<SYS_CLK_CTRL_2_SPIX_SPEC>`"]
pub type SYS_CLK_CTRL_2_SPIX = crate::Reg<sys_clk_ctrl_2_spix::SYS_CLK_CTRL_2_SPIX_SPEC>;
#[doc = "Control Settings for CLK2 - SPI XIP Clock"]
pub mod sys_clk_ctrl_2_spix;
#[doc = "SYS_CLK_CTRL_3_PRNG register accessor: an alias for `Reg<SYS_CLK_CTRL_3_PRNG_SPEC>`"]
pub type SYS_CLK_CTRL_3_PRNG = crate::Reg<sys_clk_ctrl_3_prng::SYS_CLK_CTRL_3_PRNG_SPEC>;
#[doc = "Control Settings for CLK3 - PRNG Clock"]
pub mod sys_clk_ctrl_3_prng;
#[doc = "SYS_CLK_CTRL_4_WDT0 register accessor: an alias for `Reg<SYS_CLK_CTRL_4_WDT0_SPEC>`"]
pub type SYS_CLK_CTRL_4_WDT0 = crate::Reg<sys_clk_ctrl_4_wdt0::SYS_CLK_CTRL_4_WDT0_SPEC>;
#[doc = "Control Settings for CLK4 - Watchdog Timer 0"]
pub mod sys_clk_ctrl_4_wdt0;
#[doc = "SYS_CLK_CTRL_5_WDT1 register accessor: an alias for `Reg<SYS_CLK_CTRL_5_WDT1_SPEC>`"]
pub type SYS_CLK_CTRL_5_WDT1 = crate::Reg<sys_clk_ctrl_5_wdt1::SYS_CLK_CTRL_5_WDT1_SPEC>;
#[doc = "Control Settings for CLK5 - Watchdog Timer 1"]
pub mod sys_clk_ctrl_5_wdt1;
#[doc = "SYS_CLK_CTRL_6_GPIO register accessor: an alias for `Reg<SYS_CLK_CTRL_6_GPIO_SPEC>`"]
pub type SYS_CLK_CTRL_6_GPIO = crate::Reg<sys_clk_ctrl_6_gpio::SYS_CLK_CTRL_6_GPIO_SPEC>;
#[doc = "Control Settings for CLK6 - Clock for GPIO Ports"]
pub mod sys_clk_ctrl_6_gpio;
#[doc = "SYS_CLK_CTRL_7_PT register accessor: an alias for `Reg<SYS_CLK_CTRL_7_PT_SPEC>`"]
pub type SYS_CLK_CTRL_7_PT = crate::Reg<sys_clk_ctrl_7_pt::SYS_CLK_CTRL_7_PT_SPEC>;
#[doc = "Control Settings for CLK7 - Source Clock for All Pulse Trains"]
pub mod sys_clk_ctrl_7_pt;
#[doc = "SYS_CLK_CTRL_8_UART register accessor: an alias for `Reg<SYS_CLK_CTRL_8_UART_SPEC>`"]
pub type SYS_CLK_CTRL_8_UART = crate::Reg<sys_clk_ctrl_8_uart::SYS_CLK_CTRL_8_UART_SPEC>;
#[doc = "Control Settings for CLK8 - Source Clock for All UARTs"]
pub mod sys_clk_ctrl_8_uart;
#[doc = "SYS_CLK_CTRL_9_I2CM register accessor: an alias for `Reg<SYS_CLK_CTRL_9_I2CM_SPEC>`"]
pub type SYS_CLK_CTRL_9_I2CM = crate::Reg<sys_clk_ctrl_9_i2cm::SYS_CLK_CTRL_9_I2CM_SPEC>;
#[doc = "Control Settings for CLK9 - Source Clock for All I2C Masters"]
pub mod sys_clk_ctrl_9_i2cm;
#[doc = "SYS_CLK_CTRL_10_I2CS register accessor: an alias for `Reg<SYS_CLK_CTRL_10_I2CS_SPEC>`"]
pub type SYS_CLK_CTRL_10_I2CS = crate::Reg<sys_clk_ctrl_10_i2cs::SYS_CLK_CTRL_10_I2CS_SPEC>;
#[doc = "Control Settings for CLK10 - Source Clock for I2C Slave"]
pub mod sys_clk_ctrl_10_i2cs;
#[doc = "SYS_CLK_CTRL_11_SPI0 register accessor: an alias for `Reg<SYS_CLK_CTRL_11_SPI0_SPEC>`"]
pub type SYS_CLK_CTRL_11_SPI0 = crate::Reg<sys_clk_ctrl_11_spi0::SYS_CLK_CTRL_11_SPI0_SPEC>;
#[doc = "Control Settings for CLK11 - SPI Master 0"]
pub mod sys_clk_ctrl_11_spi0;
#[doc = "SYS_CLK_CTRL_12_SPI1 register accessor: an alias for `Reg<SYS_CLK_CTRL_12_SPI1_SPEC>`"]
pub type SYS_CLK_CTRL_12_SPI1 = crate::Reg<sys_clk_ctrl_12_spi1::SYS_CLK_CTRL_12_SPI1_SPEC>;
#[doc = "Control Settings for CLK12 - SPI Master 1"]
pub mod sys_clk_ctrl_12_spi1;
#[doc = "SYS_CLK_CTRL_13_SPI2 register accessor: an alias for `Reg<SYS_CLK_CTRL_13_SPI2_SPEC>`"]
pub type SYS_CLK_CTRL_13_SPI2 = crate::Reg<sys_clk_ctrl_13_spi2::SYS_CLK_CTRL_13_SPI2_SPEC>;
#[doc = "Control Settings for CLK13 - SPI Master 2"]
pub mod sys_clk_ctrl_13_spi2;
#[doc = "SYS_CLK_CTRL_14_SPIB register accessor: an alias for `Reg<SYS_CLK_CTRL_14_SPIB_SPEC>`"]
pub type SYS_CLK_CTRL_14_SPIB = crate::Reg<sys_clk_ctrl_14_spib::SYS_CLK_CTRL_14_SPIB_SPEC>;
#[doc = "Control Settings for CLK14 - SPI Bridge Clock"]
pub mod sys_clk_ctrl_14_spib;
#[doc = "SYS_CLK_CTRL_15_OWM register accessor: an alias for `Reg<SYS_CLK_CTRL_15_OWM_SPEC>`"]
pub type SYS_CLK_CTRL_15_OWM = crate::Reg<sys_clk_ctrl_15_owm::SYS_CLK_CTRL_15_OWM_SPEC>;
#[doc = "Control Settings for CLK15 - 1-Wire Master Clock"]
pub mod sys_clk_ctrl_15_owm;
#[doc = "SYS_CLK_CTRL_16_SPIS register accessor: an alias for `Reg<SYS_CLK_CTRL_16_SPIS_SPEC>`"]
pub type SYS_CLK_CTRL_16_SPIS = crate::Reg<sys_clk_ctrl_16_spis::SYS_CLK_CTRL_16_SPIS_SPEC>;
#[doc = "Control Settings for CLK16 - SPI Slave Clock"]
pub mod sys_clk_ctrl_16_spis;
#[doc = "CRYPT_CLK_CTRL_0_AES register accessor: an alias for `Reg<CRYPT_CLK_CTRL_0_AES_SPEC>`"]
pub type CRYPT_CLK_CTRL_0_AES = crate::Reg<crypt_clk_ctrl_0_aes::CRYPT_CLK_CTRL_0_AES_SPEC>;
#[doc = "Control Settings for Crypto Clock 0 - AES"]
pub mod crypt_clk_ctrl_0_aes;
#[doc = "CRYPT_CLK_CTRL_1_MAA register accessor: an alias for `Reg<CRYPT_CLK_CTRL_1_MAA_SPEC>`"]
pub type CRYPT_CLK_CTRL_1_MAA = crate::Reg<crypt_clk_ctrl_1_maa::CRYPT_CLK_CTRL_1_MAA_SPEC>;
#[doc = "Control Settings for Crypto Clock 1 - MAA"]
pub mod crypt_clk_ctrl_1_maa;
#[doc = "CRYPT_CLK_CTRL_2_PRNG register accessor: an alias for `Reg<CRYPT_CLK_CTRL_2_PRNG_SPEC>`"]
pub type CRYPT_CLK_CTRL_2_PRNG = crate::Reg<crypt_clk_ctrl_2_prng::CRYPT_CLK_CTRL_2_PRNG_SPEC>;
#[doc = "Control Settings for Crypto Clock 2 - PRNG"]
pub mod crypt_clk_ctrl_2_prng;
#[doc = "CLK_GATE_CTRL0 register accessor: an alias for `Reg<CLK_GATE_CTRL0_SPEC>`"]
pub type CLK_GATE_CTRL0 = crate::Reg<clk_gate_ctrl0::CLK_GATE_CTRL0_SPEC>;
#[doc = "Dynamic Clock Gating Control Register 0"]
pub mod clk_gate_ctrl0;
#[doc = "CLK_GATE_CTRL1 register accessor: an alias for `Reg<CLK_GATE_CTRL1_SPEC>`"]
pub type CLK_GATE_CTRL1 = crate::Reg<clk_gate_ctrl1::CLK_GATE_CTRL1_SPEC>;
#[doc = "Dynamic Clock Gating Control Register 1"]
pub mod clk_gate_ctrl1;
#[doc = "CLK_GATE_CTRL2 register accessor: an alias for `Reg<CLK_GATE_CTRL2_SPEC>`"]
pub type CLK_GATE_CTRL2 = crate::Reg<clk_gate_ctrl2::CLK_GATE_CTRL2_SPEC>;
#[doc = "Dynamic Clock Gating Control Register 2"]
pub mod clk_gate_ctrl2;
