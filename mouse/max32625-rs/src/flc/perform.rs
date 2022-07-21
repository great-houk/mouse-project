#[doc = "Register `PERFORM` reader"]
pub struct R(crate::R<PERFORM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFORM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFORM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFORM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERFORM` writer"]
pub struct W(crate::W<PERFORM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERFORM_SPEC>;
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
impl From<crate::W<PERFORM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERFORM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `delay_se_en` reader - Delay SE Enable (Deprecated)"]
pub type DELAY_SE_EN_R = crate::BitReader<bool>;
#[doc = "Field `delay_se_en` writer - Delay SE Enable (Deprecated)"]
pub type DELAY_SE_EN_W<'a> = crate::BitWriter<'a, u32, PERFORM_SPEC, bool, 0>;
#[doc = "Field `fast_read_mode_en` reader - Fast Read Mode Enable (Deprecated)"]
pub type FAST_READ_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `fast_read_mode_en` writer - Fast Read Mode Enable (Deprecated)"]
pub type FAST_READ_MODE_EN_W<'a> = crate::BitWriter<'a, u32, PERFORM_SPEC, bool, 8>;
#[doc = "Field `en_prevent_fail` reader - Prevent Fail Flag Set on FLC Busy"]
pub type EN_PREVENT_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `en_prevent_fail` writer - Prevent Fail Flag Set on FLC Busy"]
pub type EN_PREVENT_FAIL_W<'a> = crate::BitWriter<'a, u32, PERFORM_SPEC, bool, 12>;
#[doc = "Field `en_back2back_rds` reader - Enable Back To Back Reads"]
pub type EN_BACK2BACK_RDS_R = crate::BitReader<bool>;
#[doc = "Field `en_back2back_rds` writer - Enable Back To Back Reads"]
pub type EN_BACK2BACK_RDS_W<'a> = crate::BitWriter<'a, u32, PERFORM_SPEC, bool, 16>;
#[doc = "Field `en_back2back_wrs` reader - Enable Back To Back Writes"]
pub type EN_BACK2BACK_WRS_R = crate::BitReader<bool>;
#[doc = "Field `en_back2back_wrs` writer - Enable Back To Back Writes"]
pub type EN_BACK2BACK_WRS_W<'a> = crate::BitWriter<'a, u32, PERFORM_SPEC, bool, 20>;
#[doc = "Field `en_merge_grab_gnt` reader - Enable Merge Grab GNT"]
pub type EN_MERGE_GRAB_GNT_R = crate::BitReader<bool>;
#[doc = "Field `en_merge_grab_gnt` writer - Enable Merge Grab GNT"]
pub type EN_MERGE_GRAB_GNT_W<'a> = crate::BitWriter<'a, u32, PERFORM_SPEC, bool, 24>;
#[doc = "Field `auto_tacc` reader - Auto TACC"]
pub type AUTO_TACC_R = crate::BitReader<bool>;
#[doc = "Field `auto_tacc` writer - Auto TACC"]
pub type AUTO_TACC_W<'a> = crate::BitWriter<'a, u32, PERFORM_SPEC, bool, 28>;
#[doc = "Field `auto_clkdiv` reader - Auto CLKDIV"]
pub type AUTO_CLKDIV_R = crate::BitReader<bool>;
#[doc = "Field `auto_clkdiv` writer - Auto CLKDIV"]
pub type AUTO_CLKDIV_W<'a> = crate::BitWriter<'a, u32, PERFORM_SPEC, bool, 29>;
impl R {
    #[doc = "Bit 0 - Delay SE Enable (Deprecated)"]
    #[inline(always)]
    pub fn delay_se_en(&self) -> DELAY_SE_EN_R {
        DELAY_SE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Fast Read Mode Enable (Deprecated)"]
    #[inline(always)]
    pub fn fast_read_mode_en(&self) -> FAST_READ_MODE_EN_R {
        FAST_READ_MODE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Prevent Fail Flag Set on FLC Busy"]
    #[inline(always)]
    pub fn en_prevent_fail(&self) -> EN_PREVENT_FAIL_R {
        EN_PREVENT_FAIL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Back To Back Reads"]
    #[inline(always)]
    pub fn en_back2back_rds(&self) -> EN_BACK2BACK_RDS_R {
        EN_BACK2BACK_RDS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Back To Back Writes"]
    #[inline(always)]
    pub fn en_back2back_wrs(&self) -> EN_BACK2BACK_WRS_R {
        EN_BACK2BACK_WRS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Merge Grab GNT"]
    #[inline(always)]
    pub fn en_merge_grab_gnt(&self) -> EN_MERGE_GRAB_GNT_R {
        EN_MERGE_GRAB_GNT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Auto TACC"]
    #[inline(always)]
    pub fn auto_tacc(&self) -> AUTO_TACC_R {
        AUTO_TACC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Auto CLKDIV"]
    #[inline(always)]
    pub fn auto_clkdiv(&self) -> AUTO_CLKDIV_R {
        AUTO_CLKDIV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Delay SE Enable (Deprecated)"]
    #[inline(always)]
    pub fn delay_se_en(&mut self) -> DELAY_SE_EN_W {
        DELAY_SE_EN_W::new(self)
    }
    #[doc = "Bit 8 - Fast Read Mode Enable (Deprecated)"]
    #[inline(always)]
    pub fn fast_read_mode_en(&mut self) -> FAST_READ_MODE_EN_W {
        FAST_READ_MODE_EN_W::new(self)
    }
    #[doc = "Bit 12 - Prevent Fail Flag Set on FLC Busy"]
    #[inline(always)]
    pub fn en_prevent_fail(&mut self) -> EN_PREVENT_FAIL_W {
        EN_PREVENT_FAIL_W::new(self)
    }
    #[doc = "Bit 16 - Enable Back To Back Reads"]
    #[inline(always)]
    pub fn en_back2back_rds(&mut self) -> EN_BACK2BACK_RDS_W {
        EN_BACK2BACK_RDS_W::new(self)
    }
    #[doc = "Bit 20 - Enable Back To Back Writes"]
    #[inline(always)]
    pub fn en_back2back_wrs(&mut self) -> EN_BACK2BACK_WRS_W {
        EN_BACK2BACK_WRS_W::new(self)
    }
    #[doc = "Bit 24 - Enable Merge Grab GNT"]
    #[inline(always)]
    pub fn en_merge_grab_gnt(&mut self) -> EN_MERGE_GRAB_GNT_W {
        EN_MERGE_GRAB_GNT_W::new(self)
    }
    #[doc = "Bit 28 - Auto TACC"]
    #[inline(always)]
    pub fn auto_tacc(&mut self) -> AUTO_TACC_W {
        AUTO_TACC_W::new(self)
    }
    #[doc = "Bit 29 - Auto CLKDIV"]
    #[inline(always)]
    pub fn auto_clkdiv(&mut self) -> AUTO_CLKDIV_W {
        AUTO_CLKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Performance Settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perform](index.html) module"]
pub struct PERFORM_SPEC;
impl crate::RegisterSpec for PERFORM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perform::R](R) reader structure"]
impl crate::Readable for PERFORM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perform::W](W) writer structure"]
impl crate::Writable for PERFORM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERFORM to value 0"]
impl crate::Resettable for PERFORM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
