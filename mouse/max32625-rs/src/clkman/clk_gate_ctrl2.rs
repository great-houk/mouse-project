#[doc = "Register `CLK_GATE_CTRL2` reader"]
pub struct R(crate::R<CLK_GATE_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_GATE_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_GATE_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_GATE_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_GATE_CTRL2` writer"]
pub struct W(crate::W<CLK_GATE_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_GATE_CTRL2_SPEC>;
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
impl From<crate::W<CLK_GATE_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_GATE_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `i2cs_clk_gater` reader - Clock Gating Control for I2C Slave"]
pub type I2CS_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `i2cs_clk_gater` writer - Clock Gating Control for I2C Slave"]
pub type I2CS_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL2_SPEC, u8, u8, 2, 0>;
#[doc = "Field `spi0_clk_gater` reader - Clock Gating Control for SPI Master 0"]
pub type SPI0_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi0_clk_gater` writer - Clock Gating Control for SPI Master 0"]
pub type SPI0_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL2_SPEC, u8, u8, 2, 2>;
#[doc = "Field `spi1_clk_gater` reader - Clock Gating Control for SPI Master 1"]
pub type SPI1_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi1_clk_gater` writer - Clock Gating Control for SPI Master 1"]
pub type SPI1_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL2_SPEC, u8, u8, 2, 4>;
#[doc = "Field `spi2_clk_gater` reader - Clock Gating Control for SPI Master 2"]
pub type SPI2_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi2_clk_gater` writer - Clock Gating Control for SPI Master 2"]
pub type SPI2_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL2_SPEC, u8, u8, 2, 6>;
#[doc = "Field `spi_bridge_clk_gater` reader - Clock Gating Control for SPI Bridge"]
pub type SPI_BRIDGE_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_bridge_clk_gater` writer - Clock Gating Control for SPI Bridge"]
pub type SPI_BRIDGE_CLK_GATER_W<'a> =
    crate::FieldWriter<'a, u32, CLK_GATE_CTRL2_SPEC, u8, u8, 2, 8>;
#[doc = "Field `owm_clk_gater` reader - Clock Gating Control for 1-Wire Master (OWM)"]
pub type OWM_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `owm_clk_gater` writer - Clock Gating Control for 1-Wire Master (OWM)"]
pub type OWM_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL2_SPEC, u8, u8, 2, 10>;
#[doc = "Field `adc_clk_gater` reader - Clock Gating Control for ADC"]
pub type ADC_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adc_clk_gater` writer - Clock Gating Control for ADC"]
pub type ADC_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL2_SPEC, u8, u8, 2, 12>;
#[doc = "Field `spis_clk_gater` reader - Clock Gating Control for SPI Slave"]
pub type SPIS_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spis_clk_gater` writer - Clock Gating Control for SPI Slave"]
pub type SPIS_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL2_SPEC, u8, u8, 2, 14>;
impl R {
    #[doc = "Bits 0:1 - Clock Gating Control for I2C Slave"]
    #[inline(always)]
    pub fn i2cs_clk_gater(&self) -> I2CS_CLK_GATER_R {
        I2CS_CLK_GATER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Clock Gating Control for SPI Master 0"]
    #[inline(always)]
    pub fn spi0_clk_gater(&self) -> SPI0_CLK_GATER_R {
        SPI0_CLK_GATER_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Clock Gating Control for SPI Master 1"]
    #[inline(always)]
    pub fn spi1_clk_gater(&self) -> SPI1_CLK_GATER_R {
        SPI1_CLK_GATER_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Clock Gating Control for SPI Master 2"]
    #[inline(always)]
    pub fn spi2_clk_gater(&self) -> SPI2_CLK_GATER_R {
        SPI2_CLK_GATER_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Clock Gating Control for SPI Bridge"]
    #[inline(always)]
    pub fn spi_bridge_clk_gater(&self) -> SPI_BRIDGE_CLK_GATER_R {
        SPI_BRIDGE_CLK_GATER_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Clock Gating Control for 1-Wire Master (OWM)"]
    #[inline(always)]
    pub fn owm_clk_gater(&self) -> OWM_CLK_GATER_R {
        OWM_CLK_GATER_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Clock Gating Control for ADC"]
    #[inline(always)]
    pub fn adc_clk_gater(&self) -> ADC_CLK_GATER_R {
        ADC_CLK_GATER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Clock Gating Control for SPI Slave"]
    #[inline(always)]
    pub fn spis_clk_gater(&self) -> SPIS_CLK_GATER_R {
        SPIS_CLK_GATER_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Gating Control for I2C Slave"]
    #[inline(always)]
    pub fn i2cs_clk_gater(&mut self) -> I2CS_CLK_GATER_W {
        I2CS_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 2:3 - Clock Gating Control for SPI Master 0"]
    #[inline(always)]
    pub fn spi0_clk_gater(&mut self) -> SPI0_CLK_GATER_W {
        SPI0_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 4:5 - Clock Gating Control for SPI Master 1"]
    #[inline(always)]
    pub fn spi1_clk_gater(&mut self) -> SPI1_CLK_GATER_W {
        SPI1_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 6:7 - Clock Gating Control for SPI Master 2"]
    #[inline(always)]
    pub fn spi2_clk_gater(&mut self) -> SPI2_CLK_GATER_W {
        SPI2_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock Gating Control for SPI Bridge"]
    #[inline(always)]
    pub fn spi_bridge_clk_gater(&mut self) -> SPI_BRIDGE_CLK_GATER_W {
        SPI_BRIDGE_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 10:11 - Clock Gating Control for 1-Wire Master (OWM)"]
    #[inline(always)]
    pub fn owm_clk_gater(&mut self) -> OWM_CLK_GATER_W {
        OWM_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 12:13 - Clock Gating Control for ADC"]
    #[inline(always)]
    pub fn adc_clk_gater(&mut self) -> ADC_CLK_GATER_W {
        ADC_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 14:15 - Clock Gating Control for SPI Slave"]
    #[inline(always)]
    pub fn spis_clk_gater(&mut self) -> SPIS_CLK_GATER_W {
        SPIS_CLK_GATER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic Clock Gating Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gate_ctrl2](index.html) module"]
pub struct CLK_GATE_CTRL2_SPEC;
impl crate::RegisterSpec for CLK_GATE_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_gate_ctrl2::R](R) reader structure"]
impl crate::Readable for CLK_GATE_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_gate_ctrl2::W](W) writer structure"]
impl crate::Writable for CLK_GATE_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_GATE_CTRL2 to value 0"]
impl crate::Resettable for CLK_GATE_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
