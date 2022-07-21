#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `comp0` reader - RTC Time of Day Alarm (Compare 0) Interrupt Enable"]
pub type COMP0_R = crate::BitReader<bool>;
#[doc = "Field `comp0` writer - RTC Time of Day Alarm (Compare 0) Interrupt Enable"]
pub type COMP0_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 0>;
#[doc = "Field `comp1` reader - RTC Time of Day Alarm (Compare 1) Interrupt Enable"]
pub type COMP1_R = crate::BitReader<bool>;
#[doc = "Field `comp1` writer - RTC Time of Day Alarm (Compare 1) Interrupt Enable"]
pub type COMP1_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 1>;
#[doc = "Field `prescale_comp` reader - RTC Prescale Compare Int Enable"]
pub type PRESCALE_COMP_R = crate::BitReader<bool>;
#[doc = "Field `prescale_comp` writer - RTC Prescale Compare Int Enable"]
pub type PRESCALE_COMP_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 2>;
#[doc = "Field `overflow` reader - RTC Overflow Interrupt Enable"]
pub type OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `overflow` writer - RTC Overflow Interrupt Enable"]
pub type OVERFLOW_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 3>;
#[doc = "Field `trim` reader - RTC Trim Adjust Event Interrupt Enable"]
pub type TRIM_R = crate::BitReader<bool>;
#[doc = "Field `trim` writer - RTC Trim Adjust Event Interrupt Enable"]
pub type TRIM_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - RTC Time of Day Alarm (Compare 0) Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC Time of Day Alarm (Compare 1) Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Prescale Compare Int Enable"]
    #[inline(always)]
    pub fn prescale_comp(&self) -> PRESCALE_COMP_R {
        PRESCALE_COMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC Trim Adjust Event Interrupt Enable"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Time of Day Alarm (Compare 0) Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&mut self) -> COMP0_W {
        COMP0_W::new(self)
    }
    #[doc = "Bit 1 - RTC Time of Day Alarm (Compare 1) Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&mut self) -> COMP1_W {
        COMP1_W::new(self)
    }
    #[doc = "Bit 2 - RTC Prescale Compare Int Enable"]
    #[inline(always)]
    pub fn prescale_comp(&mut self) -> PRESCALE_COMP_W {
        PRESCALE_COMP_W::new(self)
    }
    #[doc = "Bit 3 - RTC Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W::new(self)
    }
    #[doc = "Bit 4 - RTC Trim Adjust Event Interrupt Enable"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
