#[doc = "Register `TRIM_VALUE` reader"]
pub struct R(crate::R<TRIM_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_VALUE` writer"]
pub struct W(crate::W<TRIM_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_VALUE_SPEC>;
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
impl From<crate::W<TRIM_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `trim_value` reader - Trim PPM Value"]
pub type TRIM_VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `trim_value` writer - Trim PPM Value"]
pub type TRIM_VALUE_W<'a> = crate::FieldWriter<'a, u32, TRIM_VALUE_SPEC, u32, u32, 18, 0>;
#[doc = "Field `trim_control` reader - Trim Direction"]
pub type TRIM_CONTROL_R = crate::BitReader<bool>;
#[doc = "Field `trim_control` writer - Trim Direction"]
pub type TRIM_CONTROL_W<'a> = crate::BitWriter<'a, u32, TRIM_VALUE_SPEC, bool, 18>;
impl R {
    #[doc = "Bits 0:17 - Trim PPM Value"]
    #[inline(always)]
    pub fn trim_value(&self) -> TRIM_VALUE_R {
        TRIM_VALUE_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 18 - Trim Direction"]
    #[inline(always)]
    pub fn trim_control(&self) -> TRIM_CONTROL_R {
        TRIM_CONTROL_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Trim PPM Value"]
    #[inline(always)]
    pub fn trim_value(&mut self) -> TRIM_VALUE_W {
        TRIM_VALUE_W::new(self)
    }
    #[doc = "Bit 18 - Trim Direction"]
    #[inline(always)]
    pub fn trim_control(&mut self) -> TRIM_CONTROL_W {
        TRIM_CONTROL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Timer Trim Adjustment Interval\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_value](index.html) module"]
pub struct TRIM_VALUE_SPEC;
impl crate::RegisterSpec for TRIM_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_value::R](R) reader structure"]
impl crate::Readable for TRIM_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_value::W](W) writer structure"]
impl crate::Writable for TRIM_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_VALUE to value 0"]
impl crate::Resettable for TRIM_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
