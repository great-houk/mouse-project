#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `crypto_stable` reader - Crypto Oscillator Stable Interrupt Enable"]
pub type CRYPTO_STABLE_R = crate::BitReader<bool>;
#[doc = "Field `crypto_stable` writer - Crypto Oscillator Stable Interrupt Enable"]
pub type CRYPTO_STABLE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 0>;
#[doc = "Field `sys_ro_stable` reader - System Oscillator Stable Interrupt Enable"]
pub type SYS_RO_STABLE_R = crate::BitReader<bool>;
#[doc = "Field `sys_ro_stable` writer - System Oscillator Stable Interrupt Enable"]
pub type SYS_RO_STABLE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Crypto Oscillator Stable Interrupt Enable"]
    #[inline(always)]
    pub fn crypto_stable(&self) -> CRYPTO_STABLE_R {
        CRYPTO_STABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System Oscillator Stable Interrupt Enable"]
    #[inline(always)]
    pub fn sys_ro_stable(&self) -> SYS_RO_STABLE_R {
        SYS_RO_STABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Crypto Oscillator Stable Interrupt Enable"]
    #[inline(always)]
    pub fn crypto_stable(&mut self) -> CRYPTO_STABLE_W {
        CRYPTO_STABLE_W::new(self)
    }
    #[doc = "Bit 1 - System Oscillator Stable Interrupt Enable"]
    #[inline(always)]
    pub fn sys_ro_stable(&mut self) -> SYS_RO_STABLE_W {
        SYS_RO_STABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable/Disable Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
