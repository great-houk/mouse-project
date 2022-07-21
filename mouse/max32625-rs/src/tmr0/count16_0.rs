#[doc = "Register `COUNT16_0` reader"]
pub struct R(crate::R<COUNT16_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT16_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT16_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT16_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNT16_0` writer"]
pub struct W(crate::W<COUNT16_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT16_0_SPEC>;
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
impl From<crate::W<COUNT16_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT16_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `value` reader - Count Value"]
pub type VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `value` writer - Count Value"]
pub type VALUE_W<'a> = crate::FieldWriter<'a, u32, COUNT16_0_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Count Value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count Value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "16 bit\\]
Current Count Value, 16-bit Timer 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count16_0](index.html) module"]
pub struct COUNT16_0_SPEC;
impl crate::RegisterSpec for COUNT16_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [count16_0::R](R) reader structure"]
impl crate::Readable for COUNT16_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [count16_0::W](W) writer structure"]
impl crate::Writable for COUNT16_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNT16_0 to value 0"]
impl crate::Resettable for COUNT16_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
