#[doc = "Register `TIMER` reader"]
pub struct R(crate::R<TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER` writer"]
pub struct W(crate::W<TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_SPEC>;
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
impl From<crate::W<TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_SPEC>) -> Self {
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
#[doc = "RTC Timer Count Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer](index.html) module"]
pub struct TIMER_SPEC;
impl crate::RegisterSpec for TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer::R](R) reader structure"]
impl crate::Readable for TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer::W](W) writer structure"]
impl crate::Writable for TIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER to value 0"]
impl crate::Resettable for TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
