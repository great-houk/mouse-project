#[doc = "Register `LIMIT3` reader"]
pub struct R(crate::R<LIMIT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMIT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMIT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMIT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMIT3` writer"]
pub struct W(crate::W<LIMIT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMIT3_SPEC>;
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
impl From<crate::W<LIMIT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMIT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch_lo_limit` reader - Low Limit Threshold"]
pub type CH_LO_LIMIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ch_lo_limit` writer - Low Limit Threshold"]
pub type CH_LO_LIMIT_W<'a> = crate::FieldWriter<'a, u32, LIMIT3_SPEC, u16, u16, 10, 0>;
#[doc = "Field `ch_hi_limit` reader - High Limit Threshold"]
pub type CH_HI_LIMIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ch_hi_limit` writer - High Limit Threshold"]
pub type CH_HI_LIMIT_W<'a> = crate::FieldWriter<'a, u32, LIMIT3_SPEC, u16, u16, 10, 12>;
#[doc = "Field `ch_sel` reader - ADC Channel Select"]
pub type CH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ch_sel` writer - ADC Channel Select"]
pub type CH_SEL_W<'a> = crate::FieldWriter<'a, u32, LIMIT3_SPEC, u8, u8, 4, 24>;
#[doc = "Field `ch_lo_limit_en` reader - Low Limit Monitoring Enable"]
pub type CH_LO_LIMIT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ch_lo_limit_en` writer - Low Limit Monitoring Enable"]
pub type CH_LO_LIMIT_EN_W<'a> = crate::BitWriter<'a, u32, LIMIT3_SPEC, bool, 28>;
#[doc = "Field `ch_hi_limit_en` reader - High Limit Monitoring Enable"]
pub type CH_HI_LIMIT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ch_hi_limit_en` writer - High Limit Monitoring Enable"]
pub type CH_HI_LIMIT_EN_W<'a> = crate::BitWriter<'a, u32, LIMIT3_SPEC, bool, 29>;
impl R {
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline(always)]
    pub fn ch_lo_limit(&self) -> CH_LO_LIMIT_R {
        CH_LO_LIMIT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline(always)]
    pub fn ch_hi_limit(&self) -> CH_HI_LIMIT_R {
        CH_HI_LIMIT_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:27 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&self) -> CH_SEL_R {
        CH_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Low Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_lo_limit_en(&self) -> CH_LO_LIMIT_EN_R {
        CH_LO_LIMIT_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - High Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_hi_limit_en(&self) -> CH_HI_LIMIT_EN_R {
        CH_HI_LIMIT_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline(always)]
    pub fn ch_lo_limit(&mut self) -> CH_LO_LIMIT_W {
        CH_LO_LIMIT_W::new(self)
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline(always)]
    pub fn ch_hi_limit(&mut self) -> CH_HI_LIMIT_W {
        CH_HI_LIMIT_W::new(self)
    }
    #[doc = "Bits 24:27 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&mut self) -> CH_SEL_W {
        CH_SEL_W::new(self)
    }
    #[doc = "Bit 28 - Low Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_lo_limit_en(&mut self) -> CH_LO_LIMIT_EN_W {
        CH_LO_LIMIT_EN_W::new(self)
    }
    #[doc = "Bit 29 - High Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_hi_limit_en(&mut self) -> CH_HI_LIMIT_EN_W {
        CH_HI_LIMIT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Limit 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limit3](index.html) module"]
pub struct LIMIT3_SPEC;
impl crate::RegisterSpec for LIMIT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [limit3::R](R) reader structure"]
impl crate::Readable for LIMIT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limit3::W](W) writer structure"]
impl crate::Writable for LIMIT3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIMIT3 to value 0"]
impl crate::Resettable for LIMIT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
