#[doc = "Register `SNZ_VALUE` reader"]
pub struct R(crate::R<SNZ_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNZ_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNZ_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNZ_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNZ_VALUE` writer"]
pub struct W(crate::W<SNZ_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNZ_VALUE_SPEC>;
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
impl From<crate::W<SNZ_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNZ_VALUE_SPEC>) -> Self {
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
#[doc = "RTC Timer Alarm Snooze Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snz_value](index.html) module"]
pub struct SNZ_VALUE_SPEC;
impl crate::RegisterSpec for SNZ_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [snz_value::R](R) reader structure"]
impl crate::Readable for SNZ_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snz_value::W](W) writer structure"]
impl crate::Writable for SNZ_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SNZ_VALUE to value 0"]
impl crate::Resettable for SNZ_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
