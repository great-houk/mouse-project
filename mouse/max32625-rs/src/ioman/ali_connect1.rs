#[doc = "Register `ALI_CONNECT1` reader"]
pub struct R(crate::R<ALI_CONNECT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALI_CONNECT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALI_CONNECT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALI_CONNECT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALI_CONNECT1` writer"]
pub struct W(crate::W<ALI_CONNECT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALI_CONNECT1_SPEC>;
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
impl From<crate::W<ALI_CONNECT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALI_CONNECT1_SPEC>) -> Self {
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
#[doc = "Analog I/O Connection Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ali_connect1](index.html) module"]
pub struct ALI_CONNECT1_SPEC;
impl crate::RegisterSpec for ALI_CONNECT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ali_connect1::R](R) reader structure"]
impl crate::Readable for ALI_CONNECT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ali_connect1::W](W) writer structure"]
impl crate::Writable for ALI_CONNECT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALI_CONNECT1 to value 0"]
impl crate::Resettable for ALI_CONNECT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
