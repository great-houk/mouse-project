#[doc = "Register `RO_CAL0` reader"]
pub struct R(crate::R<RO_CAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RO_CAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RO_CAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RO_CAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RO_CAL0` writer"]
pub struct W(crate::W<RO_CAL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RO_CAL0_SPEC>;
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
impl From<crate::W<RO_CAL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RO_CAL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ro_cal_en` reader - RO Calibration Enable"]
pub type RO_CAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `ro_cal_en` writer - RO Calibration Enable"]
pub type RO_CAL_EN_W<'a> = crate::BitWriter<'a, u32, RO_CAL0_SPEC, bool, 0>;
#[doc = "Field `ro_cal_run` reader - RO Calibration Run"]
pub type RO_CAL_RUN_R = crate::BitReader<bool>;
#[doc = "Field `ro_cal_run` writer - RO Calibration Run"]
pub type RO_CAL_RUN_W<'a> = crate::BitWriter<'a, u32, RO_CAL0_SPEC, bool, 1>;
#[doc = "Field `ro_cal_load` reader - RO Calibration Load Initial Value"]
pub type RO_CAL_LOAD_R = crate::BitReader<bool>;
#[doc = "Field `ro_cal_load` writer - RO Calibration Load Initial Value"]
pub type RO_CAL_LOAD_W<'a> = crate::BitWriter<'a, u32, RO_CAL0_SPEC, bool, 2>;
#[doc = "Field `ro_cal_atomic` reader - RO Calibration Run Atomic"]
pub type RO_CAL_ATOMIC_R = crate::BitReader<bool>;
#[doc = "Field `ro_cal_atomic` writer - RO Calibration Run Atomic"]
pub type RO_CAL_ATOMIC_W<'a> = crate::BitWriter<'a, u32, RO_CAL0_SPEC, bool, 4>;
#[doc = "Field `dummy` reader - Dummy Write Field"]
pub type DUMMY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dummy` writer - Dummy Write Field"]
pub type DUMMY_W<'a> = crate::FieldWriter<'a, u32, RO_CAL0_SPEC, u8, u8, 3, 5>;
#[doc = "Field `trm_mu` reader - RO Trim Adaptation Gain"]
pub type TRM_MU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `trm_mu` writer - RO Trim Adaptation Gain"]
pub type TRM_MU_W<'a> = crate::FieldWriter<'a, u32, RO_CAL0_SPEC, u16, u16, 12, 8>;
#[doc = "Field `ro_trm` reader - RO Trim Calibration Result"]
pub type RO_TRM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ro_trm` writer - RO Trim Calibration Result"]
pub type RO_TRM_W<'a> = crate::FieldWriter<'a, u32, RO_CAL0_SPEC, u16, u16, 9, 23>;
impl R {
    #[doc = "Bit 0 - RO Calibration Enable"]
    #[inline(always)]
    pub fn ro_cal_en(&self) -> RO_CAL_EN_R {
        RO_CAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RO Calibration Run"]
    #[inline(always)]
    pub fn ro_cal_run(&self) -> RO_CAL_RUN_R {
        RO_CAL_RUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RO Calibration Load Initial Value"]
    #[inline(always)]
    pub fn ro_cal_load(&self) -> RO_CAL_LOAD_R {
        RO_CAL_LOAD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RO Calibration Run Atomic"]
    #[inline(always)]
    pub fn ro_cal_atomic(&self) -> RO_CAL_ATOMIC_R {
        RO_CAL_ATOMIC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Dummy Write Field"]
    #[inline(always)]
    pub fn dummy(&self) -> DUMMY_R {
        DUMMY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:19 - RO Trim Adaptation Gain"]
    #[inline(always)]
    pub fn trm_mu(&self) -> TRM_MU_R {
        TRM_MU_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 23:31 - RO Trim Calibration Result"]
    #[inline(always)]
    pub fn ro_trm(&self) -> RO_TRM_R {
        RO_TRM_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - RO Calibration Enable"]
    #[inline(always)]
    pub fn ro_cal_en(&mut self) -> RO_CAL_EN_W {
        RO_CAL_EN_W::new(self)
    }
    #[doc = "Bit 1 - RO Calibration Run"]
    #[inline(always)]
    pub fn ro_cal_run(&mut self) -> RO_CAL_RUN_W {
        RO_CAL_RUN_W::new(self)
    }
    #[doc = "Bit 2 - RO Calibration Load Initial Value"]
    #[inline(always)]
    pub fn ro_cal_load(&mut self) -> RO_CAL_LOAD_W {
        RO_CAL_LOAD_W::new(self)
    }
    #[doc = "Bit 4 - RO Calibration Run Atomic"]
    #[inline(always)]
    pub fn ro_cal_atomic(&mut self) -> RO_CAL_ATOMIC_W {
        RO_CAL_ATOMIC_W::new(self)
    }
    #[doc = "Bits 5:7 - Dummy Write Field"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DUMMY_W {
        DUMMY_W::new(self)
    }
    #[doc = "Bits 8:19 - RO Trim Adaptation Gain"]
    #[inline(always)]
    pub fn trm_mu(&mut self) -> TRM_MU_W {
        TRM_MU_W::new(self)
    }
    #[doc = "Bits 23:31 - RO Trim Calibration Result"]
    #[inline(always)]
    pub fn ro_trm(&mut self) -> RO_TRM_W {
        RO_TRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RO Trim Calibration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ro_cal0](index.html) module"]
pub struct RO_CAL0_SPEC;
impl crate::RegisterSpec for RO_CAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ro_cal0::R](R) reader structure"]
impl crate::Readable for RO_CAL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ro_cal0::W](W) writer structure"]
impl crate::Writable for RO_CAL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RO_CAL0 to value 0"]
impl crate::Resettable for RO_CAL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
