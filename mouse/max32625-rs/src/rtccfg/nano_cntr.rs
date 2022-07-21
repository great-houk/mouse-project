#[doc = "Register `NANO_CNTR` reader"]
pub struct R(crate::R<NANO_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NANO_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NANO_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NANO_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NANO_CNTR` writer"]
pub struct W(crate::W<NANO_CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NANO_CNTR_SPEC>;
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
impl From<crate::W<NANO_CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NANO_CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `nanoring_counter` reader - Nano Oscillator Counter"]
pub type NANORING_COUNTER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Nano Oscillator Counter"]
    #[inline(always)]
    pub fn nanoring_counter(&self) -> NANORING_COUNTER_R {
        NANORING_COUNTER_R::new((self.bits & 0xffff) as u16)
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
#[doc = "Nano Oscillator Counter Read Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nano_cntr](index.html) module"]
pub struct NANO_CNTR_SPEC;
impl crate::RegisterSpec for NANO_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nano_cntr::R](R) reader structure"]
impl crate::Readable for NANO_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nano_cntr::W](W) writer structure"]
impl crate::Writable for NANO_CNTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NANO_CNTR to value 0"]
impl crate::Resettable for NANO_CNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
