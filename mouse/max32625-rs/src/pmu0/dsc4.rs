#[doc = "Register `DSC4` reader"]
pub struct R(crate::R<DSC4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSC4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSC4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSC4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSC4` writer"]
pub struct W(crate::W<DSC4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSC4_SPEC>;
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
impl From<crate::W<DSC4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSC4_SPEC>) -> Self {
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
#[doc = "Current Descriptor DWORD 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsc4](index.html) module"]
pub struct DSC4_SPEC;
impl crate::RegisterSpec for DSC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsc4::R](R) reader structure"]
impl crate::Readable for DSC4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsc4::W](W) writer structure"]
impl crate::Writable for DSC4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSC4 to value 0"]
impl crate::Resettable for DSC4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
