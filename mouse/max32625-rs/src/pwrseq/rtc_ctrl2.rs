#[doc = "Register `RTC_CTRL2` reader"]
pub struct R(crate::R<RTC_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CTRL2` writer"]
pub struct W(crate::W<RTC_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CTRL2_SPEC>;
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
impl From<crate::W<RTC_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer_async_rd` reader - "]
pub type TIMER_ASYNC_RD_R = crate::BitReader<bool>;
#[doc = "Field `timer_async_rd` writer - "]
pub type TIMER_ASYNC_RD_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL2_SPEC, bool, 0>;
#[doc = "Field `timer_async_wr` reader - "]
pub type TIMER_ASYNC_WR_R = crate::BitReader<bool>;
#[doc = "Field `timer_async_wr` writer - "]
pub type TIMER_ASYNC_WR_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL2_SPEC, bool, 1>;
#[doc = "Field `timer_auto_update` reader - "]
pub type TIMER_AUTO_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `timer_auto_update` writer - "]
pub type TIMER_AUTO_UPDATE_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL2_SPEC, bool, 2>;
#[doc = "Field `ssb_performance` reader - "]
pub type SSB_PERFORMANCE_R = crate::BitReader<bool>;
#[doc = "Field `ssb_performance` writer - "]
pub type SSB_PERFORMANCE_W<'a> = crate::BitWriter<'a, u32, RTC_CTRL2_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer_async_rd(&self) -> TIMER_ASYNC_RD_R {
        TIMER_ASYNC_RD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer_async_wr(&self) -> TIMER_ASYNC_WR_R {
        TIMER_ASYNC_WR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer_auto_update(&self) -> TIMER_AUTO_UPDATE_R {
        TIMER_AUTO_UPDATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ssb_performance(&self) -> SSB_PERFORMANCE_R {
        SSB_PERFORMANCE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer_async_rd(&mut self) -> TIMER_ASYNC_RD_W {
        TIMER_ASYNC_RD_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer_async_wr(&mut self) -> TIMER_ASYNC_WR_W {
        TIMER_ASYNC_WR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer_auto_update(&mut self) -> TIMER_AUTO_UPDATE_W {
        TIMER_AUTO_UPDATE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ssb_performance(&mut self) -> SSB_PERFORMANCE_W {
        SSB_PERFORMANCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Misc Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ctrl2](index.html) module"]
pub struct RTC_CTRL2_SPEC;
impl crate::RegisterSpec for RTC_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_ctrl2::R](R) reader structure"]
impl crate::Readable for RTC_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_ctrl2::W](W) writer structure"]
impl crate::Writable for RTC_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CTRL2 to value 0"]
impl crate::Resettable for RTC_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
