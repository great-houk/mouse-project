#[doc = "Register `TRAIN` reader"]
pub struct R(crate::R<TRAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRAIN` writer"]
pub struct W(crate::W<TRAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRAIN_SPEC>;
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
impl From<crate::W<TRAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRAIN_SPEC>) -> Self {
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
#[doc = "Pulse Train Output Pattern\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [train](index.html) module"]
pub struct TRAIN_SPEC;
impl crate::RegisterSpec for TRAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [train::R](R) reader structure"]
impl crate::Readable for TRAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [train::W](W) writer structure"]
impl crate::Writable for TRAIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRAIN to value 0"]
impl crate::Resettable for TRAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
