#[doc = "Register `SVM_EVENTS` reader"]
pub struct R(crate::R<SVM_EVENTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVM_EVENTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVM_EVENTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVM_EVENTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVM_EVENTS` writer"]
pub struct W(crate::W<SVM_EVENTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVM_EVENTS_SPEC>;
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
impl From<crate::W<SVM_EVENTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVM_EVENTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `v1_2_warning` reader - 1.2V Warning Monitor Event Input"]
pub type V1_2_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `v1_8_warning` reader - 1.8V Warning Monitor Event Input"]
pub type V1_8_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `rtc_warning` reader - RTC Warning Monitor Event Input"]
pub type RTC_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `vdda_warning` reader - VDDA Warning Monitor Event Input"]
pub type VDDA_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `vddb_warning` reader - VDDB Warning Monitor Event Input"]
pub type VDDB_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `vddio_warning` reader - VDDIO Warning Monitor Event Input"]
pub type VDDIO_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `vddioh_warning` reader - VDDIOH Warning Monitor Event Input"]
pub type VDDIOH_WARNING_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 1.2V Warning Monitor Event Input"]
    #[inline(always)]
    pub fn v1_2_warning(&self) -> V1_2_WARNING_R {
        V1_2_WARNING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1.8V Warning Monitor Event Input"]
    #[inline(always)]
    pub fn v1_8_warning(&self) -> V1_8_WARNING_R {
        V1_8_WARNING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Warning Monitor Event Input"]
    #[inline(always)]
    pub fn rtc_warning(&self) -> RTC_WARNING_R {
        RTC_WARNING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VDDA Warning Monitor Event Input"]
    #[inline(always)]
    pub fn vdda_warning(&self) -> VDDA_WARNING_R {
        VDDA_WARNING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VDDB Warning Monitor Event Input"]
    #[inline(always)]
    pub fn vddb_warning(&self) -> VDDB_WARNING_R {
        VDDB_WARNING_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VDDIO Warning Monitor Event Input"]
    #[inline(always)]
    pub fn vddio_warning(&self) -> VDDIO_WARNING_R {
        VDDIO_WARNING_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VDDIOH Warning Monitor Event Input"]
    #[inline(always)]
    pub fn vddioh_warning(&self) -> VDDIOH_WARNING_R {
        VDDIOH_WARNING_R::new(((self.bits >> 6) & 1) != 0)
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
#[doc = "SVM Event Status Flags (read-only)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svm_events](index.html) module"]
pub struct SVM_EVENTS_SPEC;
impl crate::RegisterSpec for SVM_EVENTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [svm_events::R](R) reader structure"]
impl crate::Readable for SVM_EVENTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svm_events::W](W) writer structure"]
impl crate::Writable for SVM_EVENTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SVM_EVENTS to value 0"]
impl crate::Resettable for SVM_EVENTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
