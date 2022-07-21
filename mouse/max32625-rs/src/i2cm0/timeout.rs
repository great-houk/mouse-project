#[doc = "Register `TIMEOUT` reader"]
pub struct R(crate::R<TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMEOUT` writer"]
pub struct W(crate::W<TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEOUT_SPEC>;
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
impl From<crate::W<TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_timeout` reader - Transaction Timeout Limit"]
pub type TX_TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_timeout` writer - Transaction Timeout Limit"]
pub type TX_TIMEOUT_W<'a> = crate::FieldWriter<'a, u32, TIMEOUT_SPEC, u8, u8, 8, 16>;
#[doc = "Field `auto_stop_en` reader - Auto-Stop Enable"]
pub type AUTO_STOP_EN_R = crate::BitReader<bool>;
#[doc = "Field `auto_stop_en` writer - Auto-Stop Enable"]
pub type AUTO_STOP_EN_W<'a> = crate::BitWriter<'a, u32, TIMEOUT_SPEC, bool, 24>;
impl R {
    #[doc = "Bits 16:23 - Transaction Timeout Limit"]
    #[inline(always)]
    pub fn tx_timeout(&self) -> TX_TIMEOUT_R {
        TX_TIMEOUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auto-Stop Enable"]
    #[inline(always)]
    pub fn auto_stop_en(&self) -> AUTO_STOP_EN_R {
        AUTO_STOP_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - Transaction Timeout Limit"]
    #[inline(always)]
    pub fn tx_timeout(&mut self) -> TX_TIMEOUT_W {
        TX_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 24 - Auto-Stop Enable"]
    #[inline(always)]
    pub fn auto_stop_en(&mut self) -> AUTO_STOP_EN_W {
        AUTO_STOP_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout and Auto-Stop Settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout](index.html) module"]
pub struct TIMEOUT_SPEC;
impl crate::RegisterSpec for TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timeout::R](R) reader structure"]
impl crate::Readable for TIMEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timeout::W](W) writer structure"]
impl crate::Writable for TIMEOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMEOUT to value 0"]
impl crate::Resettable for TIMEOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
