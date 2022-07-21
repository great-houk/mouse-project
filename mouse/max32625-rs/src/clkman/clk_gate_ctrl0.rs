#[doc = "Register `CLK_GATE_CTRL0` reader"]
pub struct R(crate::R<CLK_GATE_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_GATE_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_GATE_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_GATE_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_GATE_CTRL0` writer"]
pub struct W(crate::W<CLK_GATE_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_GATE_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK_GATE_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_GATE_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cm4_clk_gater` reader - Clock Gating Control for CM4 CPU"]
pub type CM4_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cm4_clk_gater` writer - Clock Gating Control for CM4 CPU"]
pub type CM4_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 0>;
#[doc = "Field `ahb32_clk_gater` reader - Clock Gating Control for AHB32"]
pub type AHB32_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ahb32_clk_gater` writer - Clock Gating Control for AHB32"]
pub type AHB32_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 2>;
#[doc = "Field `icache_clk_gater` reader - Clock Gating Control for Instruction Cache"]
pub type ICACHE_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `icache_clk_gater` writer - Clock Gating Control for Instruction Cache"]
pub type ICACHE_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 4>;
#[doc = "Field `flash_clk_gater` reader - Clock Gating Control for Flash Memory"]
pub type FLASH_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `flash_clk_gater` writer - Clock Gating Control for Flash Memory"]
pub type FLASH_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 6>;
#[doc = "Field `sram_clk_gater` reader - Clock Gating Control for SRAM"]
pub type SRAM_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sram_clk_gater` writer - Clock Gating Control for SRAM"]
pub type SRAM_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 8>;
#[doc = "Field `apb_bridge_clk_gater` reader - Clock Gating Control for AHB-to-APB Bridge"]
pub type APB_BRIDGE_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `apb_bridge_clk_gater` writer - Clock Gating Control for AHB-to-APB Bridge"]
pub type APB_BRIDGE_CLK_GATER_W<'a> =
    crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 10>;
#[doc = "Field `sysman_clk_gater` reader - Clock Gating Control for CLKMAN, PWRMAN, and IOMAN"]
pub type SYSMAN_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sysman_clk_gater` writer - Clock Gating Control for CLKMAN, PWRMAN, and IOMAN"]
pub type SYSMAN_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 12>;
#[doc = "Field `ptp_clk_gater` reader - Clock Gating Control for PTP Logic"]
pub type PTP_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ptp_clk_gater` writer - Clock Gating Control for PTP Logic"]
pub type PTP_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 14>;
#[doc = "Field `ssb_mux_clk_gater` reader - Clock Gating Control for SSB Mux"]
pub type SSB_MUX_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ssb_mux_clk_gater` writer - Clock Gating Control for SSB Mux"]
pub type SSB_MUX_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 16>;
#[doc = "Field `pad_clk_gater` reader - Clock Gating Control for Pad Mode Filter"]
pub type PAD_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_clk_gater` writer - Clock Gating Control for Pad Mode Filter"]
pub type PAD_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 18>;
#[doc = "Field `spix_clk_gater` reader - Clock Gating Control for SPI XIP"]
pub type SPIX_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spix_clk_gater` writer - Clock Gating Control for SPI XIP"]
pub type SPIX_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 20>;
#[doc = "Field `pmu_clk_gater` reader - Clock Gating Control for PMU"]
pub type PMU_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pmu_clk_gater` writer - Clock Gating Control for PMU"]
pub type PMU_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 22>;
#[doc = "Field `usb_clk_gater` reader - Clock Gating Control for USB"]
pub type USB_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_clk_gater` writer - Clock Gating Control for USB"]
pub type USB_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 24>;
#[doc = "Field `crc_clk_gater` reader - Clock Gating Control for CRC"]
pub type CRC_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `crc_clk_gater` writer - Clock Gating Control for CRC"]
pub type CRC_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 26>;
#[doc = "Field `tpu_clk_gater` reader - Clock Gating Control for TPU"]
pub type TPU_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tpu_clk_gater` writer - Clock Gating Control for TPU"]
pub type TPU_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 28>;
#[doc = "Field `watchdog0_clk_gater` reader - Clock Gating Control for Watchdog Timer 0"]
pub type WATCHDOG0_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `watchdog0_clk_gater` writer - Clock Gating Control for Watchdog Timer 0"]
pub type WATCHDOG0_CLK_GATER_W<'a> =
    crate::FieldWriter<'a, u32, CLK_GATE_CTRL0_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bits 0:1 - Clock Gating Control for CM4 CPU"]
    #[inline(always)]
    pub fn cm4_clk_gater(&self) -> CM4_CLK_GATER_R {
        CM4_CLK_GATER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Clock Gating Control for AHB32"]
    #[inline(always)]
    pub fn ahb32_clk_gater(&self) -> AHB32_CLK_GATER_R {
        AHB32_CLK_GATER_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Clock Gating Control for Instruction Cache"]
    #[inline(always)]
    pub fn icache_clk_gater(&self) -> ICACHE_CLK_GATER_R {
        ICACHE_CLK_GATER_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Clock Gating Control for Flash Memory"]
    #[inline(always)]
    pub fn flash_clk_gater(&self) -> FLASH_CLK_GATER_R {
        FLASH_CLK_GATER_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Clock Gating Control for SRAM"]
    #[inline(always)]
    pub fn sram_clk_gater(&self) -> SRAM_CLK_GATER_R {
        SRAM_CLK_GATER_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Clock Gating Control for AHB-to-APB Bridge"]
    #[inline(always)]
    pub fn apb_bridge_clk_gater(&self) -> APB_BRIDGE_CLK_GATER_R {
        APB_BRIDGE_CLK_GATER_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Clock Gating Control for CLKMAN, PWRMAN, and IOMAN"]
    #[inline(always)]
    pub fn sysman_clk_gater(&self) -> SYSMAN_CLK_GATER_R {
        SYSMAN_CLK_GATER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Clock Gating Control for PTP Logic"]
    #[inline(always)]
    pub fn ptp_clk_gater(&self) -> PTP_CLK_GATER_R {
        PTP_CLK_GATER_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Clock Gating Control for SSB Mux"]
    #[inline(always)]
    pub fn ssb_mux_clk_gater(&self) -> SSB_MUX_CLK_GATER_R {
        SSB_MUX_CLK_GATER_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Clock Gating Control for Pad Mode Filter"]
    #[inline(always)]
    pub fn pad_clk_gater(&self) -> PAD_CLK_GATER_R {
        PAD_CLK_GATER_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Clock Gating Control for SPI XIP"]
    #[inline(always)]
    pub fn spix_clk_gater(&self) -> SPIX_CLK_GATER_R {
        SPIX_CLK_GATER_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Clock Gating Control for PMU"]
    #[inline(always)]
    pub fn pmu_clk_gater(&self) -> PMU_CLK_GATER_R {
        PMU_CLK_GATER_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Clock Gating Control for USB"]
    #[inline(always)]
    pub fn usb_clk_gater(&self) -> USB_CLK_GATER_R {
        USB_CLK_GATER_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Clock Gating Control for CRC"]
    #[inline(always)]
    pub fn crc_clk_gater(&self) -> CRC_CLK_GATER_R {
        CRC_CLK_GATER_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Clock Gating Control for TPU"]
    #[inline(always)]
    pub fn tpu_clk_gater(&self) -> TPU_CLK_GATER_R {
        TPU_CLK_GATER_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Clock Gating Control for Watchdog Timer 0"]
    #[inline(always)]
    pub fn watchdog0_clk_gater(&self) -> WATCHDOG0_CLK_GATER_R {
        WATCHDOG0_CLK_GATER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Gating Control for CM4 CPU"]
    #[inline(always)]
    pub fn cm4_clk_gater(&mut self) -> CM4_CLK_GATER_W {
        CM4_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 2:3 - Clock Gating Control for AHB32"]
    #[inline(always)]
    pub fn ahb32_clk_gater(&mut self) -> AHB32_CLK_GATER_W {
        AHB32_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 4:5 - Clock Gating Control for Instruction Cache"]
    #[inline(always)]
    pub fn icache_clk_gater(&mut self) -> ICACHE_CLK_GATER_W {
        ICACHE_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 6:7 - Clock Gating Control for Flash Memory"]
    #[inline(always)]
    pub fn flash_clk_gater(&mut self) -> FLASH_CLK_GATER_W {
        FLASH_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock Gating Control for SRAM"]
    #[inline(always)]
    pub fn sram_clk_gater(&mut self) -> SRAM_CLK_GATER_W {
        SRAM_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 10:11 - Clock Gating Control for AHB-to-APB Bridge"]
    #[inline(always)]
    pub fn apb_bridge_clk_gater(&mut self) -> APB_BRIDGE_CLK_GATER_W {
        APB_BRIDGE_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 12:13 - Clock Gating Control for CLKMAN, PWRMAN, and IOMAN"]
    #[inline(always)]
    pub fn sysman_clk_gater(&mut self) -> SYSMAN_CLK_GATER_W {
        SYSMAN_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 14:15 - Clock Gating Control for PTP Logic"]
    #[inline(always)]
    pub fn ptp_clk_gater(&mut self) -> PTP_CLK_GATER_W {
        PTP_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 16:17 - Clock Gating Control for SSB Mux"]
    #[inline(always)]
    pub fn ssb_mux_clk_gater(&mut self) -> SSB_MUX_CLK_GATER_W {
        SSB_MUX_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 18:19 - Clock Gating Control for Pad Mode Filter"]
    #[inline(always)]
    pub fn pad_clk_gater(&mut self) -> PAD_CLK_GATER_W {
        PAD_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 20:21 - Clock Gating Control for SPI XIP"]
    #[inline(always)]
    pub fn spix_clk_gater(&mut self) -> SPIX_CLK_GATER_W {
        SPIX_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 22:23 - Clock Gating Control for PMU"]
    #[inline(always)]
    pub fn pmu_clk_gater(&mut self) -> PMU_CLK_GATER_W {
        PMU_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 24:25 - Clock Gating Control for USB"]
    #[inline(always)]
    pub fn usb_clk_gater(&mut self) -> USB_CLK_GATER_W {
        USB_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 26:27 - Clock Gating Control for CRC"]
    #[inline(always)]
    pub fn crc_clk_gater(&mut self) -> CRC_CLK_GATER_W {
        CRC_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 28:29 - Clock Gating Control for TPU"]
    #[inline(always)]
    pub fn tpu_clk_gater(&mut self) -> TPU_CLK_GATER_W {
        TPU_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 30:31 - Clock Gating Control for Watchdog Timer 0"]
    #[inline(always)]
    pub fn watchdog0_clk_gater(&mut self) -> WATCHDOG0_CLK_GATER_W {
        WATCHDOG0_CLK_GATER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic Clock Gating Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gate_ctrl0](index.html) module"]
pub struct CLK_GATE_CTRL0_SPEC;
impl crate::RegisterSpec for CLK_GATE_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_gate_ctrl0::R](R) reader structure"]
impl crate::Readable for CLK_GATE_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_gate_ctrl0::W](W) writer structure"]
impl crate::Writable for CLK_GATE_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_GATE_CTRL0 to value 0"]
impl crate::Resettable for CLK_GATE_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
