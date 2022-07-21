#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_active` reader - ADC Conversion In Progress"]
pub type ADC_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ro_cal_atomic_active` reader - RO Frequency Calibration Active (If Atomic)"]
pub type RO_CAL_ATOMIC_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `afe_pwr_up_active` reader - AFE Power Up Delay Active"]
pub type AFE_PWR_UP_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `adc_overflow` reader - ADC Overflow"]
pub type ADC_OVERFLOW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - ADC Conversion In Progress"]
    #[inline(always)]
    pub fn adc_active(&self) -> ADC_ACTIVE_R {
        ADC_ACTIVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RO Frequency Calibration Active (If Atomic)"]
    #[inline(always)]
    pub fn ro_cal_atomic_active(&self) -> RO_CAL_ATOMIC_ACTIVE_R {
        RO_CAL_ATOMIC_ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AFE Power Up Delay Active"]
    #[inline(always)]
    pub fn afe_pwr_up_active(&self) -> AFE_PWR_UP_ACTIVE_R {
        AFE_PWR_UP_ACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Overflow"]
    #[inline(always)]
    pub fn adc_overflow(&self) -> ADC_OVERFLOW_R {
        ADC_OVERFLOW_R::new(((self.bits >> 3) & 1) != 0)
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
#[doc = "ADC Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
