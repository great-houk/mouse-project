#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_fifo_en` reader - Master Transaction FIFO Enable"]
pub type TX_FIFO_EN_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_en` writer - Master Transaction FIFO Enable"]
pub type TX_FIFO_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `rx_fifo_en` reader - Master Results FIFO Enable"]
pub type RX_FIFO_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_en` writer - Master Results FIFO Enable"]
pub type RX_FIFO_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Field `mstr_reset_en` reader - Master Reset"]
pub type MSTR_RESET_EN_R = crate::BitReader<bool>;
#[doc = "Field `mstr_reset_en` writer - Master Reset"]
pub type MSTR_RESET_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 2 - Master Transaction FIFO Enable"]
    #[inline(always)]
    pub fn tx_fifo_en(&self) -> TX_FIFO_EN_R {
        TX_FIFO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Results FIFO Enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RX_FIFO_EN_R {
        RX_FIFO_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Master Reset"]
    #[inline(always)]
    pub fn mstr_reset_en(&self) -> MSTR_RESET_EN_R {
        MSTR_RESET_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Master Transaction FIFO Enable"]
    #[inline(always)]
    pub fn tx_fifo_en(&mut self) -> TX_FIFO_EN_W {
        TX_FIFO_EN_W::new(self)
    }
    #[doc = "Bit 3 - Master Results FIFO Enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&mut self) -> RX_FIFO_EN_W {
        RX_FIFO_EN_W::new(self)
    }
    #[doc = "Bit 7 - Master Reset"]
    #[inline(always)]
    pub fn mstr_reset_en(&mut self) -> MSTR_RESET_EN_W {
        MSTR_RESET_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
