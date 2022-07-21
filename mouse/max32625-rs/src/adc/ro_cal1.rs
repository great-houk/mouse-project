#[doc = "Register `RO_CAL1` reader"]
pub struct R(crate::R<RO_CAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RO_CAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RO_CAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RO_CAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RO_CAL1` writer"]
pub struct W(crate::W<RO_CAL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RO_CAL1_SPEC>;
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
impl From<crate::W<RO_CAL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RO_CAL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `trm_init` reader - RO Trim Initial Value"]
pub type TRM_INIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `trm_init` writer - RO Trim Initial Value"]
pub type TRM_INIT_W<'a> = crate::FieldWriter<'a, u32, RO_CAL1_SPEC, u16, u16, 9, 0>;
#[doc = "Field `trm_min` reader - RO Trim Maximum Adaptive Limit"]
pub type TRM_MIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `trm_min` writer - RO Trim Maximum Adaptive Limit"]
pub type TRM_MIN_W<'a> = crate::FieldWriter<'a, u32, RO_CAL1_SPEC, u16, u16, 9, 10>;
#[doc = "Field `trm_max` reader - RO Trim Minimum Adaptive Limit"]
pub type TRM_MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `trm_max` writer - RO Trim Minimum Adaptive Limit"]
pub type TRM_MAX_W<'a> = crate::FieldWriter<'a, u32, RO_CAL1_SPEC, u16, u16, 9, 20>;
impl R {
    #[doc = "Bits 0:8 - RO Trim Initial Value"]
    #[inline(always)]
    pub fn trm_init(&self) -> TRM_INIT_R {
        TRM_INIT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 10:18 - RO Trim Maximum Adaptive Limit"]
    #[inline(always)]
    pub fn trm_min(&self) -> TRM_MIN_R {
        TRM_MIN_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:28 - RO Trim Minimum Adaptive Limit"]
    #[inline(always)]
    pub fn trm_max(&self) -> TRM_MAX_R {
        TRM_MAX_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - RO Trim Initial Value"]
    #[inline(always)]
    pub fn trm_init(&mut self) -> TRM_INIT_W {
        TRM_INIT_W::new(self)
    }
    #[doc = "Bits 10:18 - RO Trim Maximum Adaptive Limit"]
    #[inline(always)]
    pub fn trm_min(&mut self) -> TRM_MIN_W {
        TRM_MIN_W::new(self)
    }
    #[doc = "Bits 20:28 - RO Trim Minimum Adaptive Limit"]
    #[inline(always)]
    pub fn trm_max(&mut self) -> TRM_MAX_W {
        TRM_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RO Trim Calibration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ro_cal1](index.html) module"]
pub struct RO_CAL1_SPEC;
impl crate::RegisterSpec for RO_CAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ro_cal1::R](R) reader structure"]
impl crate::Readable for RO_CAL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ro_cal1::W](W) writer structure"]
impl crate::Writable for RO_CAL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RO_CAL1 to value 0"]
impl crate::Resettable for RO_CAL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
