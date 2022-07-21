#[doc = "Register `WR_PROTECT` reader"]
pub struct R(crate::R<WR_PROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WR_PROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WR_PROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WR_PROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WR_PROTECT` writer"]
pub struct W(crate::W<WR_PROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WR_PROTECT_SPEC>;
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
impl From<crate::W<WR_PROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WR_PROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bypass_seq` reader - Write Protect Bypass Sequence Write"]
pub type BYPASS_SEQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bypass_seq` writer - Write Protect Bypass Sequence Write"]
pub type BYPASS_SEQ_W<'a> = crate::FieldWriter<'a, u32, WR_PROTECT_SPEC, u8, u8, 8, 0>;
#[doc = "Field `rtc_seq` reader - Write Protect RTC Sequence Write"]
pub type RTC_SEQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rtc_seq` writer - Write Protect RTC Sequence Write"]
pub type RTC_SEQ_W<'a> = crate::FieldWriter<'a, u32, WR_PROTECT_SPEC, u8, u8, 8, 8>;
#[doc = "Field `rtc` reader - Write Protect from RTC"]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `rtc` writer - Write Protect from RTC"]
pub type RTC_W<'a> = crate::BitWriter<'a, u32, WR_PROTECT_SPEC, bool, 28>;
#[doc = "Field `info` reader - Write Protect from Info Block"]
pub type INFO_R = crate::BitReader<bool>;
#[doc = "Field `info` writer - Write Protect from Info Block"]
pub type INFO_W<'a> = crate::BitWriter<'a, u32, WR_PROTECT_SPEC, bool, 29>;
#[doc = "Field `bypass` reader - Write Protect Bypass"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `bypass` writer - Write Protect Bypass"]
pub type BYPASS_W<'a> = crate::BitWriter<'a, u32, WR_PROTECT_SPEC, bool, 30>;
#[doc = "Field `wp` reader - Critical Setting Write Protect"]
pub type WP_R = crate::BitReader<bool>;
#[doc = "Field `wp` writer - Critical Setting Write Protect"]
pub type WP_W<'a> = crate::BitWriter<'a, u32, WR_PROTECT_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:7 - Write Protect Bypass Sequence Write"]
    #[inline(always)]
    pub fn bypass_seq(&self) -> BYPASS_SEQ_R {
        BYPASS_SEQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Write Protect RTC Sequence Write"]
    #[inline(always)]
    pub fn rtc_seq(&self) -> RTC_SEQ_R {
        RTC_SEQ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Write Protect from RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write Protect from Info Block"]
    #[inline(always)]
    pub fn info(&self) -> INFO_R {
        INFO_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write Protect Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Critical Setting Write Protect"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Protect Bypass Sequence Write"]
    #[inline(always)]
    pub fn bypass_seq(&mut self) -> BYPASS_SEQ_W {
        BYPASS_SEQ_W::new(self)
    }
    #[doc = "Bits 8:15 - Write Protect RTC Sequence Write"]
    #[inline(always)]
    pub fn rtc_seq(&mut self) -> RTC_SEQ_W {
        RTC_SEQ_W::new(self)
    }
    #[doc = "Bit 28 - Write Protect from RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W::new(self)
    }
    #[doc = "Bit 29 - Write Protect from Info Block"]
    #[inline(always)]
    pub fn info(&mut self) -> INFO_W {
        INFO_W::new(self)
    }
    #[doc = "Bit 30 - Write Protect Bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 31 - Critical Setting Write Protect"]
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W {
        WP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Critical Setting Write Protect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wr_protect](index.html) module"]
pub struct WR_PROTECT_SPEC;
impl crate::RegisterSpec for WR_PROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wr_protect::R](R) reader structure"]
impl crate::Readable for WR_PROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wr_protect::W](W) writer structure"]
impl crate::Writable for WR_PROTECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WR_PROTECT to value 0"]
impl crate::Resettable for WR_PROTECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
