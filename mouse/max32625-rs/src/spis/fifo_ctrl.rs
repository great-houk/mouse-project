#[doc = "Register `FIFO_CTRL` reader"]
pub struct R(crate::R<FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_CTRL` writer"]
pub struct W(crate::W<FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CTRL_SPEC>;
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
impl From<crate::W<FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_fifo_ae_lvl` reader - Transaction FIFO Almost Empty Flag Level"]
pub type TX_FIFO_AE_LVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_fifo_ae_lvl` writer - Transaction FIFO Almost Empty Flag Level"]
pub type TX_FIFO_AE_LVL_W<'a> = crate::FieldWriter<'a, u32, FIFO_CTRL_SPEC, u8, u8, 5, 0>;
#[doc = "Field `rx_fifo_af_lvl` reader - Receive FIFO Almost Full Flag Level"]
pub type RX_FIFO_AF_LVL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Transaction FIFO Almost Empty Flag Level"]
    #[inline(always)]
    pub fn tx_fifo_ae_lvl(&self) -> TX_FIFO_AE_LVL_R {
        TX_FIFO_AE_LVL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Receive FIFO Almost Full Flag Level"]
    #[inline(always)]
    pub fn rx_fifo_af_lvl(&self) -> RX_FIFO_AF_LVL_R {
        RX_FIFO_AF_LVL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transaction FIFO Almost Empty Flag Level"]
    #[inline(always)]
    pub fn tx_fifo_ae_lvl(&mut self) -> TX_FIFO_AE_LVL_W {
        TX_FIFO_AE_LVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_ctrl](index.html) module"]
pub struct FIFO_CTRL_SPEC;
impl crate::RegisterSpec for FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_ctrl::R](R) reader structure"]
impl crate::Readable for FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_ctrl::W](W) writer structure"]
impl crate::Writable for FIFO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_CTRL to value 0"]
impl crate::Resettable for FIFO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
