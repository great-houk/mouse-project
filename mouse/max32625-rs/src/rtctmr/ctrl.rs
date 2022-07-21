#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `enable` reader - RTC Timer Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `enable` writer - RTC Timer Enable"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `clear` writer - RTC Timer Clear Bit"]
pub type CLEAR_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `pending` reader - RTC Transaction Pending"]
pub type PENDING_R = crate::BitReader<bool>;
#[doc = "Field `use_async_flags` reader - Use Async RTC Flags"]
pub type USE_ASYNC_FLAGS_R = crate::BitReader<bool>;
#[doc = "Field `use_async_flags` writer - Use Async RTC Flags"]
pub type USE_ASYNC_FLAGS_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Field `aggressive_rst` reader - Use Aggressive Reset Mode"]
pub type AGGRESSIVE_RST_R = crate::BitReader<bool>;
#[doc = "Field `aggressive_rst` writer - Use Aggressive Reset Mode"]
pub type AGGRESSIVE_RST_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 4>;
#[doc = "Field `en_active` reader - Enable RTC in Active Modes"]
pub type EN_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `osc_goto_low_active` reader - osc_goto_low_r transaction is pending"]
pub type OSC_GOTO_LOW_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `osc_frce_sm_en_active` reader - osc_force_mode transaction is pending"]
pub type OSC_FRCE_SM_EN_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `osc_frce_st_active` reader - osc_force_state transaction is pending"]
pub type OSC_FRCE_ST_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `set_active` reader - timer_set_active"]
pub type SET_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `clr_active` reader - RTC clear is pending"]
pub type CLR_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `rollover_clr_active` reader - rollover clr is pending"]
pub type ROLLOVER_CLR_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `prescale_cmpr0_active` reader - prescale cmpr0 is pending"]
pub type PRESCALE_CMPR0_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `prescale_update_active` reader - prescale update transaction is pending"]
pub type PRESCALE_UPDATE_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `cmpr1_clr_active` reader - cmpr1 clear transaction is pending"]
pub type CMPR1_CLR_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `cmpr0_clr_active` reader - cmpr0 clear transaction is pending"]
pub type CMPR0_CLR_ACTIVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Transaction Pending"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Use Async RTC Flags"]
    #[inline(always)]
    pub fn use_async_flags(&self) -> USE_ASYNC_FLAGS_R {
        USE_ASYNC_FLAGS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Use Aggressive Reset Mode"]
    #[inline(always)]
    pub fn aggressive_rst(&self) -> AGGRESSIVE_RST_R {
        AGGRESSIVE_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable RTC in Active Modes"]
    #[inline(always)]
    pub fn en_active(&self) -> EN_ACTIVE_R {
        EN_ACTIVE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - osc_goto_low_r transaction is pending"]
    #[inline(always)]
    pub fn osc_goto_low_active(&self) -> OSC_GOTO_LOW_ACTIVE_R {
        OSC_GOTO_LOW_ACTIVE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - osc_force_mode transaction is pending"]
    #[inline(always)]
    pub fn osc_frce_sm_en_active(&self) -> OSC_FRCE_SM_EN_ACTIVE_R {
        OSC_FRCE_SM_EN_ACTIVE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - osc_force_state transaction is pending"]
    #[inline(always)]
    pub fn osc_frce_st_active(&self) -> OSC_FRCE_ST_ACTIVE_R {
        OSC_FRCE_ST_ACTIVE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - timer_set_active"]
    #[inline(always)]
    pub fn set_active(&self) -> SET_ACTIVE_R {
        SET_ACTIVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RTC clear is pending"]
    #[inline(always)]
    pub fn clr_active(&self) -> CLR_ACTIVE_R {
        CLR_ACTIVE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - rollover clr is pending"]
    #[inline(always)]
    pub fn rollover_clr_active(&self) -> ROLLOVER_CLR_ACTIVE_R {
        ROLLOVER_CLR_ACTIVE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - prescale cmpr0 is pending"]
    #[inline(always)]
    pub fn prescale_cmpr0_active(&self) -> PRESCALE_CMPR0_ACTIVE_R {
        PRESCALE_CMPR0_ACTIVE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - prescale update transaction is pending"]
    #[inline(always)]
    pub fn prescale_update_active(&self) -> PRESCALE_UPDATE_ACTIVE_R {
        PRESCALE_UPDATE_ACTIVE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - cmpr1 clear transaction is pending"]
    #[inline(always)]
    pub fn cmpr1_clr_active(&self) -> CMPR1_CLR_ACTIVE_R {
        CMPR1_CLR_ACTIVE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - cmpr0 clear transaction is pending"]
    #[inline(always)]
    pub fn cmpr0_clr_active(&self) -> CMPR0_CLR_ACTIVE_R {
        CMPR0_CLR_ACTIVE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - RTC Timer Clear Bit"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 3 - Use Async RTC Flags"]
    #[inline(always)]
    pub fn use_async_flags(&mut self) -> USE_ASYNC_FLAGS_W {
        USE_ASYNC_FLAGS_W::new(self)
    }
    #[doc = "Bit 4 - Use Aggressive Reset Mode"]
    #[inline(always)]
    pub fn aggressive_rst(&mut self) -> AGGRESSIVE_RST_W {
        AGGRESSIVE_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
