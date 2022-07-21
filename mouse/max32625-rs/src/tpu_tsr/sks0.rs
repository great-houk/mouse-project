#[doc = "Register `SKS0` reader"]
pub struct R(crate::R<SKS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SKS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SKS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SKS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SKS0` writer"]
pub struct W(crate::W<SKS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SKS0_SPEC>;
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
impl From<crate::W<SKS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SKS0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TPU Secure Key Storage Register 0 (Cleared on Tamper Detect)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sks0](index.html) module"]
pub struct SKS0_SPEC;
impl crate::RegisterSpec for SKS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sks0::R](R) reader structure"]
impl crate::Readable for SKS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sks0::W](W) writer structure"]
impl crate::Writable for SKS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SKS0 to value 0"]
impl crate::Resettable for SKS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
