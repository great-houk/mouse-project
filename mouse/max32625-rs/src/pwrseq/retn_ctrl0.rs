#[doc = "Register `RETN_CTRL0` reader"]
pub struct R(crate::R<RETN_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETN_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETN_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETN_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETN_CTRL0` writer"]
pub struct W(crate::W<RETN_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETN_CTRL0_SPEC>;
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
impl From<crate::W<RETN_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETN_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `retn_ctrl_en` reader - Retention Controller Enable"]
pub type RETN_CTRL_EN_R = crate::BitReader<bool>;
#[doc = "Field `retn_ctrl_en` writer - Retention Controller Enable"]
pub type RETN_CTRL_EN_W<'a> = crate::BitWriter<'a, u32, RETN_CTRL0_SPEC, bool, 0>;
#[doc = "Field `rc_rel_ccg_early` reader - Early Core Clock Gate Release"]
pub type RC_REL_CCG_EARLY_R = crate::BitReader<bool>;
#[doc = "Field `rc_rel_ccg_early` writer - Early Core Clock Gate Release"]
pub type RC_REL_CCG_EARLY_W<'a> = crate::BitWriter<'a, u32, RETN_CTRL0_SPEC, bool, 1>;
#[doc = "Field `rc_use_flc_twk` reader - Enable Flash Controller TWake Timer"]
pub type RC_USE_FLC_TWK_R = crate::BitReader<bool>;
#[doc = "Field `rc_use_flc_twk` writer - Enable Flash Controller TWake Timer"]
pub type RC_USE_FLC_TWK_W<'a> = crate::BitWriter<'a, u32, RETN_CTRL0_SPEC, bool, 2>;
#[doc = "Field `rc_poll_flash` reader - Enable Automatic Flash Polling for Wakeup"]
pub type RC_POLL_FLASH_R = crate::BitReader<bool>;
#[doc = "Field `rc_poll_flash` writer - Enable Automatic Flash Polling for Wakeup"]
pub type RC_POLL_FLASH_W<'a> = crate::BitWriter<'a, u32, RETN_CTRL0_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Retention Controller Enable"]
    #[inline(always)]
    pub fn retn_ctrl_en(&self) -> RETN_CTRL_EN_R {
        RETN_CTRL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Early Core Clock Gate Release"]
    #[inline(always)]
    pub fn rc_rel_ccg_early(&self) -> RC_REL_CCG_EARLY_R {
        RC_REL_CCG_EARLY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Flash Controller TWake Timer"]
    #[inline(always)]
    pub fn rc_use_flc_twk(&self) -> RC_USE_FLC_TWK_R {
        RC_USE_FLC_TWK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Automatic Flash Polling for Wakeup"]
    #[inline(always)]
    pub fn rc_poll_flash(&self) -> RC_POLL_FLASH_R {
        RC_POLL_FLASH_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Retention Controller Enable"]
    #[inline(always)]
    pub fn retn_ctrl_en(&mut self) -> RETN_CTRL_EN_W {
        RETN_CTRL_EN_W::new(self)
    }
    #[doc = "Bit 1 - Early Core Clock Gate Release"]
    #[inline(always)]
    pub fn rc_rel_ccg_early(&mut self) -> RC_REL_CCG_EARLY_W {
        RC_REL_CCG_EARLY_W::new(self)
    }
    #[doc = "Bit 2 - Enable Flash Controller TWake Timer"]
    #[inline(always)]
    pub fn rc_use_flc_twk(&mut self) -> RC_USE_FLC_TWK_W {
        RC_USE_FLC_TWK_W::new(self)
    }
    #[doc = "Bit 3 - Enable Automatic Flash Polling for Wakeup"]
    #[inline(always)]
    pub fn rc_poll_flash(&mut self) -> RC_POLL_FLASH_W {
        RC_POLL_FLASH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Retention Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retn_ctrl0](index.html) module"]
pub struct RETN_CTRL0_SPEC;
impl crate::RegisterSpec for RETN_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retn_ctrl0::R](R) reader structure"]
impl crate::Readable for RETN_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retn_ctrl0::W](W) writer structure"]
impl crate::Writable for RETN_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETN_CTRL0 to value 0"]
impl crate::Resettable for RETN_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
