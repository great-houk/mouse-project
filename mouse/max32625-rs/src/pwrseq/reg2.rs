#[doc = "Register `REG2` reader"]
pub struct R(crate::R<REG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG2` writer"]
pub struct W(crate::W<REG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG2_SPEC>;
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
impl From<crate::W<REG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_vdd12_hyst` reader - VDD12_SW Comparator Hysteresis Setting"]
pub type PWR_VDD12_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_vdd12_hyst` writer - VDD12_SW Comparator Hysteresis Setting"]
pub type PWR_VDD12_HYST_W<'a> = crate::FieldWriter<'a, u32, REG2_SPEC, u8, u8, 2, 0>;
#[doc = "Field `pwr_vdd18_hyst` reader - VDD18_SW Comparator Hysteresis Setting"]
pub type PWR_VDD18_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_vdd18_hyst` writer - VDD18_SW Comparator Hysteresis Setting"]
pub type PWR_VDD18_HYST_W<'a> = crate::FieldWriter<'a, u32, REG2_SPEC, u8, u8, 2, 2>;
#[doc = "Field `pwr_vrtc_hyst` reader - VRTC Comparator Hysteresis Setting"]
pub type PWR_VRTC_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_vrtc_hyst` writer - VRTC Comparator Hysteresis Setting"]
pub type PWR_VRTC_HYST_W<'a> = crate::FieldWriter<'a, u32, REG2_SPEC, u8, u8, 2, 4>;
#[doc = "Field `pwr_vddb_hyst` reader - VDDB Comparator Hysteresis Setting"]
pub type PWR_VDDB_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_vddb_hyst` writer - VDDB Comparator Hysteresis Setting"]
pub type PWR_VDDB_HYST_W<'a> = crate::FieldWriter<'a, u32, REG2_SPEC, u8, u8, 2, 6>;
#[doc = "Field `pwr_tvdd12_hyst` reader - TVDD12 Comparator Hysteresis Setting"]
pub type PWR_TVDD12_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_tvdd12_hyst` writer - TVDD12 Comparator Hysteresis Setting"]
pub type PWR_TVDD12_HYST_W<'a> = crate::FieldWriter<'a, u32, REG2_SPEC, u8, u8, 2, 8>;
#[doc = "Field `pwr_vddio_hyst` reader - VDDIO Comparator Hysteresis Setting"]
pub type PWR_VDDIO_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_vddio_hyst` writer - VDDIO Comparator Hysteresis Setting"]
pub type PWR_VDDIO_HYST_W<'a> = crate::FieldWriter<'a, u32, REG2_SPEC, u8, u8, 2, 10>;
#[doc = "Field `pwr_vddioh_hyst` reader - VDDIO Comparator Hysteresis Setting"]
pub type PWR_VDDIOH_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_vddioh_hyst` writer - VDDIO Comparator Hysteresis Setting"]
pub type PWR_VDDIOH_HYST_W<'a> = crate::FieldWriter<'a, u32, REG2_SPEC, u8, u8, 2, 12>;
impl R {
    #[doc = "Bits 0:1 - VDD12_SW Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vdd12_hyst(&self) -> PWR_VDD12_HYST_R {
        PWR_VDD12_HYST_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - VDD18_SW Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vdd18_hyst(&self) -> PWR_VDD18_HYST_R {
        PWR_VDD18_HYST_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - VRTC Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vrtc_hyst(&self) -> PWR_VRTC_HYST_R {
        PWR_VRTC_HYST_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - VDDB Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vddb_hyst(&self) -> PWR_VDDB_HYST_R {
        PWR_VDDB_HYST_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TVDD12 Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_tvdd12_hyst(&self) -> PWR_TVDD12_HYST_R {
        PWR_TVDD12_HYST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - VDDIO Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vddio_hyst(&self) -> PWR_VDDIO_HYST_R {
        PWR_VDDIO_HYST_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - VDDIO Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vddioh_hyst(&self) -> PWR_VDDIOH_HYST_R {
        PWR_VDDIOH_HYST_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - VDD12_SW Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vdd12_hyst(&mut self) -> PWR_VDD12_HYST_W {
        PWR_VDD12_HYST_W::new(self)
    }
    #[doc = "Bits 2:3 - VDD18_SW Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vdd18_hyst(&mut self) -> PWR_VDD18_HYST_W {
        PWR_VDD18_HYST_W::new(self)
    }
    #[doc = "Bits 4:5 - VRTC Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vrtc_hyst(&mut self) -> PWR_VRTC_HYST_W {
        PWR_VRTC_HYST_W::new(self)
    }
    #[doc = "Bits 6:7 - VDDB Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vddb_hyst(&mut self) -> PWR_VDDB_HYST_W {
        PWR_VDDB_HYST_W::new(self)
    }
    #[doc = "Bits 8:9 - TVDD12 Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_tvdd12_hyst(&mut self) -> PWR_TVDD12_HYST_W {
        PWR_TVDD12_HYST_W::new(self)
    }
    #[doc = "Bits 10:11 - VDDIO Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vddio_hyst(&mut self) -> PWR_VDDIO_HYST_W {
        PWR_VDDIO_HYST_W::new(self)
    }
    #[doc = "Bits 12:13 - VDDIO Comparator Hysteresis Setting"]
    #[inline(always)]
    pub fn pwr_vddioh_hyst(&mut self) -> PWR_VDDIOH_HYST_W {
        PWR_VDDIOH_HYST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg2](index.html) module"]
pub struct REG2_SPEC;
impl crate::RegisterSpec for REG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg2::R](R) reader structure"]
impl crate::Readable for REG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg2::W](W) writer structure"]
impl crate::Writable for REG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG2 to value 0"]
impl crate::Resettable for REG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
