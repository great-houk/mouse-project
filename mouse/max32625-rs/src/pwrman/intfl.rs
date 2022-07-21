#[doc = "Register `INTFL` reader"]
pub struct R(crate::R<INTFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFL` writer"]
pub struct W(crate::W<INTFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFL_SPEC>;
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
impl From<crate::W<INTFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `v1_2_warning` reader - 1.2V Warning Monitor Int Flag"]
pub type V1_2_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `v1_2_warning` writer - 1.2V Warning Monitor Int Flag"]
pub type V1_2_WARNING_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 0>;
#[doc = "Field `v1_8_warning` reader - 1.8V Warning Monitor Int Flag"]
pub type V1_8_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `v1_8_warning` writer - 1.8V Warning Monitor Int Flag"]
pub type V1_8_WARNING_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 1>;
#[doc = "Field `rtc_warning` reader - RTC Warning Monitor Int Flag"]
pub type RTC_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `rtc_warning` writer - RTC Warning Monitor Int Flag"]
pub type RTC_WARNING_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 2>;
#[doc = "Field `vdda_warning` reader - VDDA Warning Monitor Int Flag"]
pub type VDDA_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `vdda_warning` writer - VDDA Warning Monitor Int Flag"]
pub type VDDA_WARNING_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 3>;
#[doc = "Field `vddb_warning` reader - VDDB Warning Monitor Int Flag"]
pub type VDDB_WARNING_R = crate::BitReader<bool>;
#[doc = "Field `vddb_warning` writer - VDDB Warning Monitor Int Flag"]
pub type VDDB_WARNING_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 4>;
#[doc = "Field `vddio_reset` reader - VDDIO Supply Reset Int Flag"]
pub type VDDIO_RESET_R = crate::BitReader<bool>;
#[doc = "Field `vddio_reset` writer - VDDIO Supply Reset Int Flag"]
pub type VDDIO_RESET_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 5>;
#[doc = "Field `vddioh_reset` reader - VDDIOH Supply Reset Int Flag"]
pub type VDDIOH_RESET_R = crate::BitReader<bool>;
#[doc = "Field `vddioh_reset` writer - VDDIOH Supply Reset Int Flag"]
pub type VDDIOH_RESET_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - 1.2V Warning Monitor Int Flag"]
    #[inline(always)]
    pub fn v1_2_warning(&self) -> V1_2_WARNING_R {
        V1_2_WARNING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1.8V Warning Monitor Int Flag"]
    #[inline(always)]
    pub fn v1_8_warning(&self) -> V1_8_WARNING_R {
        V1_8_WARNING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Warning Monitor Int Flag"]
    #[inline(always)]
    pub fn rtc_warning(&self) -> RTC_WARNING_R {
        RTC_WARNING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VDDA Warning Monitor Int Flag"]
    #[inline(always)]
    pub fn vdda_warning(&self) -> VDDA_WARNING_R {
        VDDA_WARNING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VDDB Warning Monitor Int Flag"]
    #[inline(always)]
    pub fn vddb_warning(&self) -> VDDB_WARNING_R {
        VDDB_WARNING_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VDDIO Supply Reset Int Flag"]
    #[inline(always)]
    pub fn vddio_reset(&self) -> VDDIO_RESET_R {
        VDDIO_RESET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VDDIOH Supply Reset Int Flag"]
    #[inline(always)]
    pub fn vddioh_reset(&self) -> VDDIOH_RESET_R {
        VDDIOH_RESET_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1.2V Warning Monitor Int Flag"]
    #[inline(always)]
    pub fn v1_2_warning(&mut self) -> V1_2_WARNING_W {
        V1_2_WARNING_W::new(self)
    }
    #[doc = "Bit 1 - 1.8V Warning Monitor Int Flag"]
    #[inline(always)]
    pub fn v1_8_warning(&mut self) -> V1_8_WARNING_W {
        V1_8_WARNING_W::new(self)
    }
    #[doc = "Bit 2 - RTC Warning Monitor Int Flag"]
    #[inline(always)]
    pub fn rtc_warning(&mut self) -> RTC_WARNING_W {
        RTC_WARNING_W::new(self)
    }
    #[doc = "Bit 3 - VDDA Warning Monitor Int Flag"]
    #[inline(always)]
    pub fn vdda_warning(&mut self) -> VDDA_WARNING_W {
        VDDA_WARNING_W::new(self)
    }
    #[doc = "Bit 4 - VDDB Warning Monitor Int Flag"]
    #[inline(always)]
    pub fn vddb_warning(&mut self) -> VDDB_WARNING_W {
        VDDB_WARNING_W::new(self)
    }
    #[doc = "Bit 5 - VDDIO Supply Reset Int Flag"]
    #[inline(always)]
    pub fn vddio_reset(&mut self) -> VDDIO_RESET_W {
        VDDIO_RESET_W::new(self)
    }
    #[doc = "Bit 6 - VDDIOH Supply Reset Int Flag"]
    #[inline(always)]
    pub fn vddioh_reset(&mut self) -> VDDIOH_RESET_W {
        VDDIOH_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](index.html) module"]
pub struct INTFL_SPEC;
impl crate::RegisterSpec for INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intfl::R](R) reader structure"]
impl crate::Readable for INTFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intfl::W](W) writer structure"]
impl crate::Writable for INTFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for INTFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
