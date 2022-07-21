#[doc = "Register `DSC1` reader"]
pub struct R(crate::R<DSC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSC1` writer"]
pub struct W(crate::W<DSC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSC1_SPEC>;
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
impl From<crate::W<DSC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSC1_SPEC>) -> Self {
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
#[doc = "Current Descriptor DWORD 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsc1](index.html) module"]
pub struct DSC1_SPEC;
impl crate::RegisterSpec for DSC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsc1::R](R) reader structure"]
impl crate::Readable for DSC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsc1::W](W) writer structure"]
impl crate::Writable for DSC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSC1 to value 0"]
impl crate::Resettable for DSC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
