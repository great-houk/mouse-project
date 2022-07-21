#[doc = "Register `LOCK_CTRL` reader"]
pub struct R(crate::R<LOCK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCK_CTRL` writer"]
pub struct W(crate::W<LOCK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_CTRL_SPEC>;
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
impl From<crate::W<LOCK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wdlock` reader - Lock for WDT CTRL Register"]
pub type WDLOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wdlock` writer - Lock for WDT CTRL Register"]
pub type WDLOCK_W<'a> = crate::FieldWriter<'a, u32, LOCK_CTRL_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Lock for WDT CTRL Register"]
    #[inline(always)]
    pub fn wdlock(&self) -> WDLOCK_R {
        WDLOCK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lock for WDT CTRL Register"]
    #[inline(always)]
    pub fn wdlock(&mut self) -> WDLOCK_W {
        WDLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT0 - Register Setting Lock for WDT0_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock_ctrl](index.html) module"]
pub struct LOCK_CTRL_SPEC;
impl crate::RegisterSpec for LOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock_ctrl::R](R) reader structure"]
impl crate::Readable for LOCK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lock_ctrl::W](W) writer structure"]
impl crate::Writable for LOCK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCK_CTRL to value 0"]
impl crate::Resettable for LOCK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
