#[doc = "Register `BAUD` reader"]
pub struct R(crate::R<BAUD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD` writer"]
pub struct W(crate::W<BAUD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD_SPEC>;
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
impl From<crate::W<BAUD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `baud_divisor` reader - Baud Divisor"]
pub type BAUD_DIVISOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `baud_divisor` writer - Baud Divisor"]
pub type BAUD_DIVISOR_W<'a> = crate::FieldWriter<'a, u32, BAUD_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Baud Divisor"]
    #[inline(always)]
    pub fn baud_divisor(&self) -> BAUD_DIVISOR_R {
        BAUD_DIVISOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud Divisor"]
    #[inline(always)]
    pub fn baud_divisor(&mut self) -> BAUD_DIVISOR_W {
        BAUD_DIVISOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Baud Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud](index.html) module"]
pub struct BAUD_SPEC;
impl crate::RegisterSpec for BAUD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baud::R](R) reader structure"]
impl crate::Readable for BAUD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud::W](W) writer structure"]
impl crate::Writable for BAUD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUD to value 0"]
impl crate::Resettable for BAUD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
