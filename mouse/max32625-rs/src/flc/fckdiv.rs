#[doc = "Register `FCKDIV` reader"]
pub struct R(crate::R<FCKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCKDIV` writer"]
pub struct W(crate::W<FCKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCKDIV_SPEC>;
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
impl From<crate::W<FCKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fckdiv` reader - Flash Clock Pulse Divisor"]
pub type FCKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fckdiv` writer - Flash Clock Pulse Divisor"]
pub type FCKDIV_W<'a> = crate::FieldWriter<'a, u32, FCKDIV_SPEC, u8, u8, 7, 0>;
#[doc = "Field `auto_fckdiv_result` reader - Auto FCKDIV Calculation Result"]
pub type AUTO_FCKDIV_RESULT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:6 - Flash Clock Pulse Divisor"]
    #[inline(always)]
    pub fn fckdiv(&self) -> FCKDIV_R {
        FCKDIV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:31 - Auto FCKDIV Calculation Result"]
    #[inline(always)]
    pub fn auto_fckdiv_result(&self) -> AUTO_FCKDIV_RESULT_R {
        AUTO_FCKDIV_RESULT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Flash Clock Pulse Divisor"]
    #[inline(always)]
    pub fn fckdiv(&mut self) -> FCKDIV_W {
        FCKDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Clock Pulse Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fckdiv](index.html) module"]
pub struct FCKDIV_SPEC;
impl crate::RegisterSpec for FCKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fckdiv::R](R) reader structure"]
impl crate::Readable for FCKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fckdiv::W](W) writer structure"]
impl crate::Writable for FCKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCKDIV to value 0"]
impl crate::Resettable for FCKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
