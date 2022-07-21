#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `long_line_mode` reader - Long Line Mode"]
pub type LONG_LINE_MODE_R = crate::BitReader<bool>;
#[doc = "Field `long_line_mode` writer - Long Line Mode"]
pub type LONG_LINE_MODE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 0>;
#[doc = "Field `force_pres_det` reader - Force Line During Presence Detect"]
pub type FORCE_PRES_DET_R = crate::BitReader<bool>;
#[doc = "Field `force_pres_det` writer - Force Line During Presence Detect"]
pub type FORCE_PRES_DET_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 1>;
#[doc = "Field `bit_bang_en` reader - Bit Bang Enable"]
pub type BIT_BANG_EN_R = crate::BitReader<bool>;
#[doc = "Field `bit_bang_en` writer - Bit Bang Enable"]
pub type BIT_BANG_EN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 2>;
#[doc = "Field `ext_pullup_mode` reader - Provide an extra output to control an external pullup."]
pub type EXT_PULLUP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ext_pullup_mode` writer - Provide an extra output to control an external pullup."]
pub type EXT_PULLUP_MODE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 3>;
#[doc = "Field `ext_pullup_enable` reader - Enable External Pullup"]
pub type EXT_PULLUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ext_pullup_enable` writer - Enable External Pullup"]
pub type EXT_PULLUP_ENABLE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 4>;
#[doc = "Field `single_bit_mode` reader - Enable Single Bit TX/RX Mode"]
pub type SINGLE_BIT_MODE_R = crate::BitReader<bool>;
#[doc = "Field `single_bit_mode` writer - Enable Single Bit TX/RX Mode"]
pub type SINGLE_BIT_MODE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 5>;
#[doc = "Field `overdrive` reader - Enables overdrive speed for 1-Wire operations."]
pub type OVERDRIVE_R = crate::BitReader<bool>;
#[doc = "Field `overdrive` writer - Enables overdrive speed for 1-Wire operations."]
pub type OVERDRIVE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 6>;
#[doc = "Field `int_pullup_enable` reader - Enable internal pullup."]
pub type INT_PULLUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `int_pullup_enable` writer - Enable internal pullup."]
pub type INT_PULLUP_ENABLE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Long Line Mode"]
    #[inline(always)]
    pub fn long_line_mode(&self) -> LONG_LINE_MODE_R {
        LONG_LINE_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Line During Presence Detect"]
    #[inline(always)]
    pub fn force_pres_det(&self) -> FORCE_PRES_DET_R {
        FORCE_PRES_DET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit Bang Enable"]
    #[inline(always)]
    pub fn bit_bang_en(&self) -> BIT_BANG_EN_R {
        BIT_BANG_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Provide an extra output to control an external pullup."]
    #[inline(always)]
    pub fn ext_pullup_mode(&self) -> EXT_PULLUP_MODE_R {
        EXT_PULLUP_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable External Pullup"]
    #[inline(always)]
    pub fn ext_pullup_enable(&self) -> EXT_PULLUP_ENABLE_R {
        EXT_PULLUP_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode"]
    #[inline(always)]
    pub fn single_bit_mode(&self) -> SINGLE_BIT_MODE_R {
        SINGLE_BIT_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline(always)]
    pub fn overdrive(&self) -> OVERDRIVE_R {
        OVERDRIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable internal pullup."]
    #[inline(always)]
    pub fn int_pullup_enable(&self) -> INT_PULLUP_ENABLE_R {
        INT_PULLUP_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Long Line Mode"]
    #[inline(always)]
    pub fn long_line_mode(&mut self) -> LONG_LINE_MODE_W {
        LONG_LINE_MODE_W::new(self)
    }
    #[doc = "Bit 1 - Force Line During Presence Detect"]
    #[inline(always)]
    pub fn force_pres_det(&mut self) -> FORCE_PRES_DET_W {
        FORCE_PRES_DET_W::new(self)
    }
    #[doc = "Bit 2 - Bit Bang Enable"]
    #[inline(always)]
    pub fn bit_bang_en(&mut self) -> BIT_BANG_EN_W {
        BIT_BANG_EN_W::new(self)
    }
    #[doc = "Bit 3 - Provide an extra output to control an external pullup."]
    #[inline(always)]
    pub fn ext_pullup_mode(&mut self) -> EXT_PULLUP_MODE_W {
        EXT_PULLUP_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Enable External Pullup"]
    #[inline(always)]
    pub fn ext_pullup_enable(&mut self) -> EXT_PULLUP_ENABLE_W {
        EXT_PULLUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode"]
    #[inline(always)]
    pub fn single_bit_mode(&mut self) -> SINGLE_BIT_MODE_W {
        SINGLE_BIT_MODE_W::new(self)
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline(always)]
    pub fn overdrive(&mut self) -> OVERDRIVE_W {
        OVERDRIVE_W::new(self)
    }
    #[doc = "Bit 7 - Enable internal pullup."]
    #[inline(always)]
    pub fn int_pullup_enable(&mut self) -> INT_PULLUP_ENABLE_W {
        INT_PULLUP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1-Wire Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
