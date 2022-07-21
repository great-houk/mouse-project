#[doc = "Register `SETUP0` reader"]
pub struct R(crate::R<SETUP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP0` writer"]
pub struct W(crate::W<SETUP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP0_SPEC>;
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
impl From<crate::W<SETUP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `byte0` reader - SETUP Packet Byte 0"]
pub type BYTE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `byte1` reader - SETUP Packet Byte 1"]
pub type BYTE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `byte2` reader - SETUP Packet Byte 2"]
pub type BYTE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `byte3` reader - SETUP Packet Byte 3"]
pub type BYTE3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SETUP Packet Byte 0"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SETUP Packet Byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SETUP Packet Byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SETUP Packet Byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
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
#[doc = "USB SETUP Packet Bytes 0 to 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup0](index.html) module"]
pub struct SETUP0_SPEC;
impl crate::RegisterSpec for SETUP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup0::R](R) reader structure"]
impl crate::Readable for SETUP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup0::W](W) writer structure"]
impl crate::Writable for SETUP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SETUP0 to value 0"]
impl crate::Resettable for SETUP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
