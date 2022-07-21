#[doc = "Register `RETN_CTRL1` reader"]
pub struct R(crate::R<RETN_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETN_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETN_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETN_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETN_CTRL1` writer"]
pub struct W(crate::W<RETN_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETN_CTRL1_SPEC>;
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
impl From<crate::W<RETN_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETN_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc_twk` reader - Retention Controller TWake Cycle Count"]
pub type RTC_TWK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rtc_twk` writer - Retention Controller TWake Cycle Count"]
pub type RTC_TWK_W<'a> = crate::FieldWriter<'a, u32, RETN_CTRL1_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - Retention Controller TWake Cycle Count"]
    #[inline(always)]
    pub fn rtc_twk(&self) -> RTC_TWK_R {
        RTC_TWK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Retention Controller TWake Cycle Count"]
    #[inline(always)]
    pub fn rtc_twk(&mut self) -> RTC_TWK_W {
        RTC_TWK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Retention Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retn_ctrl1](index.html) module"]
pub struct RETN_CTRL1_SPEC;
impl crate::RegisterSpec for RETN_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retn_ctrl1::R](R) reader structure"]
impl crate::Readable for RETN_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retn_ctrl1::W](W) writer structure"]
impl crate::Writable for RETN_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RETN_CTRL1 to value 0"]
impl crate::Resettable for RETN_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
