#[doc = "Register `MODE_DATA` reader"]
pub struct R(crate::R<MODE_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE_DATA` writer"]
pub struct W(crate::W<MODE_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_DATA_SPEC>;
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
impl From<crate::W<MODE_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mode_data_bits` reader - Mode Data"]
pub type MODE_DATA_BITS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `mode_data_bits` writer - Mode Data"]
pub type MODE_DATA_BITS_W<'a> = crate::FieldWriter<'a, u32, MODE_DATA_SPEC, u16, u16, 16, 0>;
#[doc = "Field `mode_data_oe` reader - Mode Output Enable"]
pub type MODE_DATA_OE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `mode_data_oe` writer - Mode Output Enable"]
pub type MODE_DATA_OE_W<'a> = crate::FieldWriter<'a, u32, MODE_DATA_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15 - Mode Data"]
    #[inline(always)]
    pub fn mode_data_bits(&self) -> MODE_DATA_BITS_R {
        MODE_DATA_BITS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Mode Output Enable"]
    #[inline(always)]
    pub fn mode_data_oe(&self) -> MODE_DATA_OE_R {
        MODE_DATA_OE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mode Data"]
    #[inline(always)]
    pub fn mode_data_bits(&mut self) -> MODE_DATA_BITS_W {
        MODE_DATA_BITS_W::new(self)
    }
    #[doc = "Bits 16:31 - Mode Output Enable"]
    #[inline(always)]
    pub fn mode_data_oe(&mut self) -> MODE_DATA_OE_W {
        MODE_DATA_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIX Mode Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_data](index.html) module"]
pub struct MODE_DATA_SPEC;
impl crate::RegisterSpec for MODE_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode_data::R](R) reader structure"]
impl crate::Readable for MODE_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode_data::W](W) writer structure"]
impl crate::Writable for MODE_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE_DATA to value 0"]
impl crate::Resettable for MODE_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
