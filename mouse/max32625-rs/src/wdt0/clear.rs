#[doc = "Register `CLEAR` reader"]
pub struct R(crate::R<CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLEAR` writer"]
pub struct W(crate::W<CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEAR_SPEC>;
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
impl From<crate::W<CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEAR_SPEC>) -> Self {
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
#[doc = "WDT0 - Watchdog Clear Register (Feed Dog)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clear](index.html) module"]
pub struct CLEAR_SPEC;
impl crate::RegisterSpec for CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clear::R](R) reader structure"]
impl crate::Readable for CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clear::W](W) writer structure"]
impl crate::Writable for CLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
