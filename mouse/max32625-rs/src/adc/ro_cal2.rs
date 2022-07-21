#[doc = "Register `RO_CAL2` reader"]
pub struct R(crate::R<RO_CAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RO_CAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RO_CAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RO_CAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RO_CAL2` writer"]
pub struct W(crate::W<RO_CAL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RO_CAL2_SPEC>;
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
impl From<crate::W<RO_CAL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RO_CAL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `auto_cal_done_cnt` reader - Auto Cal Time Delay for Atomic Calibration (in milliseconds)"]
pub type AUTO_CAL_DONE_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `auto_cal_done_cnt` writer - Auto Cal Time Delay for Atomic Calibration (in milliseconds)"]
pub type AUTO_CAL_DONE_CNT_W<'a> = crate::FieldWriter<'a, u32, RO_CAL2_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Auto Cal Time Delay for Atomic Calibration (in milliseconds)"]
    #[inline(always)]
    pub fn auto_cal_done_cnt(&self) -> AUTO_CAL_DONE_CNT_R {
        AUTO_CAL_DONE_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Auto Cal Time Delay for Atomic Calibration (in milliseconds)"]
    #[inline(always)]
    pub fn auto_cal_done_cnt(&mut self) -> AUTO_CAL_DONE_CNT_W {
        AUTO_CAL_DONE_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RO Trim Calibration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ro_cal2](index.html) module"]
pub struct RO_CAL2_SPEC;
impl crate::RegisterSpec for RO_CAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ro_cal2::R](R) reader structure"]
impl crate::Readable for RO_CAL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ro_cal2::W](W) writer structure"]
impl crate::Writable for RO_CAL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RO_CAL2 to value 0"]
impl crate::Resettable for RO_CAL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
