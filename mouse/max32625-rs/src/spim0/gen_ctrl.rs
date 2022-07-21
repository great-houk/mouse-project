#[doc = "Register `GEN_CTRL` reader"]
pub struct R(crate::R<GEN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN_CTRL` writer"]
pub struct W(crate::W<GEN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN_CTRL_SPEC>;
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
impl From<crate::W<GEN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_mstr_en` reader - Enable/Disable SPI Master"]
pub type SPI_MSTR_EN_R = crate::BitReader<bool>;
#[doc = "Field `spi_mstr_en` writer - Enable/Disable SPI Master"]
pub type SPI_MSTR_EN_W<'a> = crate::BitWriter<'a, u32, GEN_CTRL_SPEC, bool, 0>;
#[doc = "Field `tx_fifo_en` reader - Transaction FIFO Enable"]
pub type TX_FIFO_EN_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_en` writer - Transaction FIFO Enable"]
pub type TX_FIFO_EN_W<'a> = crate::BitWriter<'a, u32, GEN_CTRL_SPEC, bool, 1>;
#[doc = "Field `rx_fifo_en` reader - Results FIFO Enable"]
pub type RX_FIFO_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_en` writer - Results FIFO Enable"]
pub type RX_FIFO_EN_W<'a> = crate::BitWriter<'a, u32, GEN_CTRL_SPEC, bool, 2>;
#[doc = "Field `bit_bang_mode` reader - Bit Bang Mode Enable"]
pub type BIT_BANG_MODE_R = crate::BitReader<bool>;
#[doc = "Field `bit_bang_mode` writer - Bit Bang Mode Enable"]
pub type BIT_BANG_MODE_W<'a> = crate::BitWriter<'a, u32, GEN_CTRL_SPEC, bool, 3>;
#[doc = "Field `bb_ss_in_out` reader - Bit Bang SS Input/Output"]
pub type BB_SS_IN_OUT_R = crate::BitReader<bool>;
#[doc = "Field `bb_ss_in_out` writer - Bit Bang SS Input/Output"]
pub type BB_SS_IN_OUT_W<'a> = crate::BitWriter<'a, u32, GEN_CTRL_SPEC, bool, 4>;
#[doc = "Field `bb_sr_in` reader - Bit Bang SR Input"]
pub type BB_SR_IN_R = crate::BitReader<bool>;
#[doc = "Field `bb_sck_in_out` reader - Bit Bang SCK Input/Output"]
pub type BB_SCK_IN_OUT_R = crate::BitReader<bool>;
#[doc = "Field `bb_sck_in_out` writer - Bit Bang SCK Input/Output"]
pub type BB_SCK_IN_OUT_W<'a> = crate::BitWriter<'a, u32, GEN_CTRL_SPEC, bool, 6>;
#[doc = "Field `bb_sdio_in` reader - Bit Bang SDIO Input"]
pub type BB_SDIO_IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bb_sdio_out` reader - Bit Bang SDIO Output"]
pub type BB_SDIO_OUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bb_sdio_out` writer - Bit Bang SDIO Output"]
pub type BB_SDIO_OUT_W<'a> = crate::FieldWriter<'a, u32, GEN_CTRL_SPEC, u8, u8, 4, 12>;
#[doc = "Field `bb_sdio_dr_en` reader - Bit Bang SDIO Drive Enable"]
pub type BB_SDIO_DR_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bb_sdio_dr_en` writer - Bit Bang SDIO Drive Enable"]
pub type BB_SDIO_DR_EN_W<'a> = crate::FieldWriter<'a, u32, GEN_CTRL_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bit 0 - Enable/Disable SPI Master"]
    #[inline(always)]
    pub fn spi_mstr_en(&self) -> SPI_MSTR_EN_R {
        SPI_MSTR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transaction FIFO Enable"]
    #[inline(always)]
    pub fn tx_fifo_en(&self) -> TX_FIFO_EN_R {
        TX_FIFO_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Results FIFO Enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RX_FIFO_EN_R {
        RX_FIFO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit Bang Mode Enable"]
    #[inline(always)]
    pub fn bit_bang_mode(&self) -> BIT_BANG_MODE_R {
        BIT_BANG_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit Bang SS Input/Output"]
    #[inline(always)]
    pub fn bb_ss_in_out(&self) -> BB_SS_IN_OUT_R {
        BB_SS_IN_OUT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit Bang SR Input"]
    #[inline(always)]
    pub fn bb_sr_in(&self) -> BB_SR_IN_R {
        BB_SR_IN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit Bang SCK Input/Output"]
    #[inline(always)]
    pub fn bb_sck_in_out(&self) -> BB_SCK_IN_OUT_R {
        BB_SCK_IN_OUT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Bit Bang SDIO Input"]
    #[inline(always)]
    pub fn bb_sdio_in(&self) -> BB_SDIO_IN_R {
        BB_SDIO_IN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Bit Bang SDIO Output"]
    #[inline(always)]
    pub fn bb_sdio_out(&self) -> BB_SDIO_OUT_R {
        BB_SDIO_OUT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Bit Bang SDIO Drive Enable"]
    #[inline(always)]
    pub fn bb_sdio_dr_en(&self) -> BB_SDIO_DR_EN_R {
        BB_SDIO_DR_EN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable/Disable SPI Master"]
    #[inline(always)]
    pub fn spi_mstr_en(&mut self) -> SPI_MSTR_EN_W {
        SPI_MSTR_EN_W::new(self)
    }
    #[doc = "Bit 1 - Transaction FIFO Enable"]
    #[inline(always)]
    pub fn tx_fifo_en(&mut self) -> TX_FIFO_EN_W {
        TX_FIFO_EN_W::new(self)
    }
    #[doc = "Bit 2 - Results FIFO Enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&mut self) -> RX_FIFO_EN_W {
        RX_FIFO_EN_W::new(self)
    }
    #[doc = "Bit 3 - Bit Bang Mode Enable"]
    #[inline(always)]
    pub fn bit_bang_mode(&mut self) -> BIT_BANG_MODE_W {
        BIT_BANG_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Bit Bang SS Input/Output"]
    #[inline(always)]
    pub fn bb_ss_in_out(&mut self) -> BB_SS_IN_OUT_W {
        BB_SS_IN_OUT_W::new(self)
    }
    #[doc = "Bit 6 - Bit Bang SCK Input/Output"]
    #[inline(always)]
    pub fn bb_sck_in_out(&mut self) -> BB_SCK_IN_OUT_W {
        BB_SCK_IN_OUT_W::new(self)
    }
    #[doc = "Bits 12:15 - Bit Bang SDIO Output"]
    #[inline(always)]
    pub fn bb_sdio_out(&mut self) -> BB_SDIO_OUT_W {
        BB_SDIO_OUT_W::new(self)
    }
    #[doc = "Bits 16:19 - Bit Bang SDIO Drive Enable"]
    #[inline(always)]
    pub fn bb_sdio_dr_en(&mut self) -> BB_SDIO_DR_EN_W {
        BB_SDIO_DR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen_ctrl](index.html) module"]
pub struct GEN_CTRL_SPEC;
impl crate::RegisterSpec for GEN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen_ctrl::R](R) reader structure"]
impl crate::Readable for GEN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen_ctrl::W](W) writer structure"]
impl crate::Writable for GEN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN_CTRL to value 0"]
impl crate::Resettable for GEN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
