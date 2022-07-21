#[doc = "Register `MAWS` reader"]
pub struct R(crate::R<MAWS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAWS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAWS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAWS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAWS` writer"]
pub struct W(crate::W<MAWS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAWS_SPEC>;
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
impl From<crate::W<MAWS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAWS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `modlen` reader - MAA Word Size"]
pub type MODLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `modlen` writer - MAA Word Size"]
pub type MODLEN_W<'a> = crate::FieldWriter<'a, u32, MAWS_SPEC, u16, u16, 10, 0>;
#[doc = "Field `byteswap` reader - Big Endian Byte Mode"]
pub type BYTESWAP_R = crate::BitReader<bool>;
#[doc = "Field `byteswap` writer - Big Endian Byte Mode"]
pub type BYTESWAP_W<'a> = crate::BitWriter<'a, u32, MAWS_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:9 - MAA Word Size"]
    #[inline(always)]
    pub fn modlen(&self) -> MODLEN_R {
        MODLEN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Big Endian Byte Mode"]
    #[inline(always)]
    pub fn byteswap(&self) -> BYTESWAP_R {
        BYTESWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - MAA Word Size"]
    #[inline(always)]
    pub fn modlen(&mut self) -> MODLEN_W {
        MODLEN_W::new(self)
    }
    #[doc = "Bit 15 - Big Endian Byte Mode"]
    #[inline(always)]
    pub fn byteswap(&mut self) -> BYTESWAP_W {
        BYTESWAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAA Word (Operand) Size, Big/Little Endian Mode Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maws](index.html) module"]
pub struct MAWS_SPEC;
impl crate::RegisterSpec for MAWS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maws::R](R) reader structure"]
impl crate::Readable for MAWS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maws::W](W) writer structure"]
impl crate::Writable for MAWS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAWS to value 0"]
impl crate::Resettable for MAWS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
