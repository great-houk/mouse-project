#[doc = "Register `DISABLE_XR7` reader"]
pub struct R(crate::R<DISABLE_XR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISABLE_XR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISABLE_XR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISABLE_XR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISABLE_XR7` writer"]
pub struct W(crate::W<DISABLE_XR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISABLE_XR7_SPEC>;
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
impl From<crate::W<DISABLE_XR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISABLE_XR7_SPEC>) -> Self {
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
#[doc = "Disable Flash Page Exec/Read Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disable_xr7](index.html) module"]
pub struct DISABLE_XR7_SPEC;
impl crate::RegisterSpec for DISABLE_XR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [disable_xr7::R](R) reader structure"]
impl crate::Readable for DISABLE_XR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [disable_xr7::W](W) writer structure"]
impl crate::Writable for DISABLE_XR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DISABLE_XR7 to value 0"]
impl crate::Resettable for DISABLE_XR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
