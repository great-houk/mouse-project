#[doc = "Register `AFE_CTRL` reader"]
pub struct R(crate::R<AFE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFE_CTRL` writer"]
pub struct W(crate::W<AFE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFE_CTRL_SPEC>;
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
impl From<crate::W<AFE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmon_intbias_en` reader - Enable internal temperature measurement bias generator"]
pub type TMON_INTBIAS_EN_R = crate::BitReader<bool>;
#[doc = "Field `tmon_intbias_en` writer - Enable internal temperature measurement bias generator"]
pub type TMON_INTBIAS_EN_W<'a> = crate::BitWriter<'a, u32, AFE_CTRL_SPEC, bool, 8>;
#[doc = "Field `tmon_extbias_en` reader - Enable external temperature measurement bias generator"]
pub type TMON_EXTBIAS_EN_R = crate::BitReader<bool>;
#[doc = "Field `tmon_extbias_en` writer - Enable external temperature measurement bias generator"]
pub type TMON_EXTBIAS_EN_W<'a> = crate::BitWriter<'a, u32, AFE_CTRL_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 8 - Enable internal temperature measurement bias generator"]
    #[inline(always)]
    pub fn tmon_intbias_en(&self) -> TMON_INTBIAS_EN_R {
        TMON_INTBIAS_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable external temperature measurement bias generator"]
    #[inline(always)]
    pub fn tmon_extbias_en(&self) -> TMON_EXTBIAS_EN_R {
        TMON_EXTBIAS_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable internal temperature measurement bias generator"]
    #[inline(always)]
    pub fn tmon_intbias_en(&mut self) -> TMON_INTBIAS_EN_W {
        TMON_INTBIAS_EN_W::new(self)
    }
    #[doc = "Bit 9 - Enable external temperature measurement bias generator"]
    #[inline(always)]
    pub fn tmon_extbias_en(&mut self) -> TMON_EXTBIAS_EN_W {
        TMON_EXTBIAS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFE Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afe_ctrl](index.html) module"]
pub struct AFE_CTRL_SPEC;
impl crate::RegisterSpec for AFE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afe_ctrl::R](R) reader structure"]
impl crate::Readable for AFE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afe_ctrl::W](W) writer structure"]
impl crate::Writable for AFE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AFE_CTRL to value 0"]
impl crate::Resettable for AFE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
