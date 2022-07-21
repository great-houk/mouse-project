#[doc = "Register `PRESCALE` reader"]
pub struct R(crate::R<PRESCALE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESCALE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESCALE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESCALE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESCALE` writer"]
pub struct W(crate::W<PRESCALE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESCALE_SPEC>;
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
impl From<crate::W<PRESCALE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESCALE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `width_selection` reader - RTC Timer Prescale Setting"]
pub type WIDTH_SELECTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `width_selection` writer - RTC Timer Prescale Setting"]
pub type WIDTH_SELECTION_W<'a> = crate::FieldWriter<'a, u32, PRESCALE_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - RTC Timer Prescale Setting"]
    #[inline(always)]
    pub fn width_selection(&self) -> WIDTH_SELECTION_R {
        WIDTH_SELECTION_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC Timer Prescale Setting"]
    #[inline(always)]
    pub fn width_selection(&mut self) -> WIDTH_SELECTION_W {
        WIDTH_SELECTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Timer Prescale Setting\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescale](index.html) module"]
pub struct PRESCALE_SPEC;
impl crate::RegisterSpec for PRESCALE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prescale::R](R) reader structure"]
impl crate::Readable for PRESCALE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prescale::W](W) writer structure"]
impl crate::Writable for PRESCALE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESCALE to value 0"]
impl crate::Resettable for PRESCALE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
