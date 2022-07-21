#[doc = "Register `SETUP1` reader"]
pub struct R(crate::R<SETUP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP1` writer"]
pub struct W(crate::W<SETUP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP1_SPEC>;
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
impl From<crate::W<SETUP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `byte4` reader - SETUP Packet Byte 4"]
pub type BYTE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `byte5` reader - SETUP Packet Byte 5"]
pub type BYTE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `byte6` reader - SETUP Packet Byte 6"]
pub type BYTE6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `byte7` reader - SETUP Packet Byte 7"]
pub type BYTE7_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SETUP Packet Byte 4"]
    #[inline(always)]
    pub fn byte4(&self) -> BYTE4_R {
        BYTE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SETUP Packet Byte 5"]
    #[inline(always)]
    pub fn byte5(&self) -> BYTE5_R {
        BYTE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SETUP Packet Byte 6"]
    #[inline(always)]
    pub fn byte6(&self) -> BYTE6_R {
        BYTE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SETUP Packet Byte 7"]
    #[inline(always)]
    pub fn byte7(&self) -> BYTE7_R {
        BYTE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB SETUP Packet Bytes 4 to 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup1](index.html) module"]
pub struct SETUP1_SPEC;
impl crate::RegisterSpec for SETUP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup1::R](R) reader structure"]
impl crate::Readable for SETUP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup1::W](W) writer structure"]
impl crate::Writable for SETUP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SETUP1 to value 0"]
impl crate::Resettable for SETUP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
