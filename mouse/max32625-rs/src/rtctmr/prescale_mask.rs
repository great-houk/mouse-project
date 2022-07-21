#[doc = "Register `PRESCALE_MASK` reader"]
pub struct R(crate::R<PRESCALE_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESCALE_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESCALE_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESCALE_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESCALE_MASK` writer"]
pub struct W(crate::W<PRESCALE_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESCALE_MASK_SPEC>;
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
impl From<crate::W<PRESCALE_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESCALE_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `comp_mask` reader - RTC Timer Prescale Compare Mask"]
pub type COMP_MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `comp_mask` writer - RTC Timer Prescale Compare Mask"]
pub type COMP_MASK_W<'a> = crate::FieldWriter<'a, u32, PRESCALE_MASK_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - RTC Timer Prescale Compare Mask"]
    #[inline(always)]
    pub fn comp_mask(&self) -> COMP_MASK_R {
        COMP_MASK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC Timer Prescale Compare Mask"]
    #[inline(always)]
    pub fn comp_mask(&mut self) -> COMP_MASK_W {
        COMP_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Timer Prescale Compare Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescale_mask](index.html) module"]
pub struct PRESCALE_MASK_SPEC;
impl crate::RegisterSpec for PRESCALE_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prescale_mask::R](R) reader structure"]
impl crate::Readable for PRESCALE_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prescale_mask::W](W) writer structure"]
impl crate::Writable for PRESCALE_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESCALE_MASK to value 0"]
impl crate::Resettable for PRESCALE_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
