#[doc = "Register `MSTR_CFG` reader"]
pub struct R(crate::R<MSTR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTR_CFG` writer"]
pub struct W(crate::W<MSTR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTR_CFG_SPEC>;
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
impl From<crate::W<MSTR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slave_sel` reader - SPI Slave Select"]
pub type SLAVE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `slave_sel` writer - SPI Slave Select"]
pub type SLAVE_SEL_W<'a> = crate::FieldWriter<'a, u32, MSTR_CFG_SPEC, u8, u8, 3, 0>;
#[doc = "Field `three_wire_mode` reader - 3-Wire Mode"]
pub type THREE_WIRE_MODE_R = crate::BitReader<bool>;
#[doc = "Field `three_wire_mode` writer - 3-Wire Mode"]
pub type THREE_WIRE_MODE_W<'a> = crate::BitWriter<'a, u32, MSTR_CFG_SPEC, bool, 3>;
#[doc = "Field `spi_mode` reader - SPI Mode"]
pub type SPI_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_mode` writer - SPI Mode"]
pub type SPI_MODE_W<'a> = crate::FieldWriter<'a, u32, MSTR_CFG_SPEC, u8, u8, 2, 4>;
#[doc = "Field `page_size` reader - Page Size"]
pub type PAGE_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `page_size` writer - Page Size"]
pub type PAGE_SIZE_W<'a> = crate::FieldWriter<'a, u32, MSTR_CFG_SPEC, u8, u8, 2, 6>;
#[doc = "Field `sck_hi_clk` reader - SCK High Clocks"]
pub type SCK_HI_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sck_hi_clk` writer - SCK High Clocks"]
pub type SCK_HI_CLK_W<'a> = crate::FieldWriter<'a, u32, MSTR_CFG_SPEC, u8, u8, 4, 8>;
#[doc = "Field `sck_lo_clk` reader - SCK Low Clocks"]
pub type SCK_LO_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sck_lo_clk` writer - SCK Low Clocks"]
pub type SCK_LO_CLK_W<'a> = crate::FieldWriter<'a, u32, MSTR_CFG_SPEC, u8, u8, 4, 12>;
#[doc = "Field `act_delay` reader - SS Active Timing"]
pub type ACT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `act_delay` writer - SS Active Timing"]
pub type ACT_DELAY_W<'a> = crate::FieldWriter<'a, u32, MSTR_CFG_SPEC, u8, u8, 2, 16>;
#[doc = "Field `inact_delay` reader - SS Inactive Timing"]
pub type INACT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `inact_delay` writer - SS Inactive Timing"]
pub type INACT_DELAY_W<'a> = crate::FieldWriter<'a, u32, MSTR_CFG_SPEC, u8, u8, 2, 18>;
#[doc = "Field `alt_sck_hi_clk` reader - Alt SCK High Clocks"]
pub type ALT_SCK_HI_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `alt_sck_hi_clk` writer - Alt SCK High Clocks"]
pub type ALT_SCK_HI_CLK_W<'a> = crate::FieldWriter<'a, u32, MSTR_CFG_SPEC, u8, u8, 4, 20>;
#[doc = "Field `alt_sck_lo_clk` reader - Alt SCK Low Clocks"]
pub type ALT_SCK_LO_CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `alt_sck_lo_clk` writer - Alt SCK Low Clocks"]
pub type ALT_SCK_LO_CLK_W<'a> = crate::FieldWriter<'a, u32, MSTR_CFG_SPEC, u8, u8, 4, 24>;
impl R {
    #[doc = "Bits 0:2 - SPI Slave Select"]
    #[inline(always)]
    pub fn slave_sel(&self) -> SLAVE_SEL_R {
        SLAVE_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3-Wire Mode"]
    #[inline(always)]
    pub fn three_wire_mode(&self) -> THREE_WIRE_MODE_R {
        THREE_WIRE_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SPI Mode"]
    #[inline(always)]
    pub fn spi_mode(&self) -> SPI_MODE_R {
        SPI_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Page Size"]
    #[inline(always)]
    pub fn page_size(&self) -> PAGE_SIZE_R {
        PAGE_SIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - SCK High Clocks"]
    #[inline(always)]
    pub fn sck_hi_clk(&self) -> SCK_HI_CLK_R {
        SCK_HI_CLK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SCK Low Clocks"]
    #[inline(always)]
    pub fn sck_lo_clk(&self) -> SCK_LO_CLK_R {
        SCK_LO_CLK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - SS Active Timing"]
    #[inline(always)]
    pub fn act_delay(&self) -> ACT_DELAY_R {
        ACT_DELAY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - SS Inactive Timing"]
    #[inline(always)]
    pub fn inact_delay(&self) -> INACT_DELAY_R {
        INACT_DELAY_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - Alt SCK High Clocks"]
    #[inline(always)]
    pub fn alt_sck_hi_clk(&self) -> ALT_SCK_HI_CLK_R {
        ALT_SCK_HI_CLK_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alt SCK Low Clocks"]
    #[inline(always)]
    pub fn alt_sck_lo_clk(&self) -> ALT_SCK_LO_CLK_R {
        ALT_SCK_LO_CLK_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI Slave Select"]
    #[inline(always)]
    pub fn slave_sel(&mut self) -> SLAVE_SEL_W {
        SLAVE_SEL_W::new(self)
    }
    #[doc = "Bit 3 - 3-Wire Mode"]
    #[inline(always)]
    pub fn three_wire_mode(&mut self) -> THREE_WIRE_MODE_W {
        THREE_WIRE_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - SPI Mode"]
    #[inline(always)]
    pub fn spi_mode(&mut self) -> SPI_MODE_W {
        SPI_MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Page Size"]
    #[inline(always)]
    pub fn page_size(&mut self) -> PAGE_SIZE_W {
        PAGE_SIZE_W::new(self)
    }
    #[doc = "Bits 8:11 - SCK High Clocks"]
    #[inline(always)]
    pub fn sck_hi_clk(&mut self) -> SCK_HI_CLK_W {
        SCK_HI_CLK_W::new(self)
    }
    #[doc = "Bits 12:15 - SCK Low Clocks"]
    #[inline(always)]
    pub fn sck_lo_clk(&mut self) -> SCK_LO_CLK_W {
        SCK_LO_CLK_W::new(self)
    }
    #[doc = "Bits 16:17 - SS Active Timing"]
    #[inline(always)]
    pub fn act_delay(&mut self) -> ACT_DELAY_W {
        ACT_DELAY_W::new(self)
    }
    #[doc = "Bits 18:19 - SS Inactive Timing"]
    #[inline(always)]
    pub fn inact_delay(&mut self) -> INACT_DELAY_W {
        INACT_DELAY_W::new(self)
    }
    #[doc = "Bits 20:23 - Alt SCK High Clocks"]
    #[inline(always)]
    pub fn alt_sck_hi_clk(&mut self) -> ALT_SCK_HI_CLK_W {
        ALT_SCK_HI_CLK_W::new(self)
    }
    #[doc = "Bits 24:27 - Alt SCK Low Clocks"]
    #[inline(always)]
    pub fn alt_sck_lo_clk(&mut self) -> ALT_SCK_LO_CLK_W {
        ALT_SCK_LO_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstr_cfg](index.html) module"]
pub struct MSTR_CFG_SPEC;
impl crate::RegisterSpec for MSTR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstr_cfg::R](R) reader structure"]
impl crate::Readable for MSTR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstr_cfg::W](W) writer structure"]
impl crate::Writable for MSTR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSTR_CFG to value 0"]
impl crate::Resettable for MSTR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
