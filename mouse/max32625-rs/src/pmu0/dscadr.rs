#[doc = "Register `DSCADR` reader"]
pub struct R(crate::R<DSCADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSCADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSCADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSCADR` writer"]
pub struct W(crate::W<DSCADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCADR_SPEC>;
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
impl From<crate::W<DSCADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCADR_SPEC>) -> Self {
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
#[doc = "Starting Descriptor Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscadr](index.html) module"]
pub struct DSCADR_SPEC;
impl crate::RegisterSpec for DSCADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dscadr::R](R) reader structure"]
impl crate::Readable for DSCADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dscadr::W](W) writer structure"]
impl crate::Writable for DSCADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSCADR to value 0"]
impl crate::Resettable for DSCADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
