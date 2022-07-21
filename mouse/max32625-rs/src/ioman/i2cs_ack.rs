#[doc = "Register `I2CS_ACK` reader"]
pub struct R(crate::R<I2CS_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CS_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CS_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CS_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CS_ACK` writer"]
pub struct W(crate::W<I2CS_ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CS_ACK_SPEC>;
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
impl From<crate::W<I2CS_ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CS_ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `io_sel` reader - I2C Slave I/O Mapping Acknowledge"]
pub type IO_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mapping_ack` reader - I2C Slave I/O Acknowledge"]
pub type MAPPING_ACK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - I2C Slave I/O Mapping Acknowledge"]
    #[inline(always)]
    pub fn io_sel(&self) -> IO_SEL_R {
        IO_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - I2C Slave I/O Acknowledge"]
    #[inline(always)]
    pub fn mapping_ack(&self) -> MAPPING_ACK_R {
        MAPPING_ACK_R::new(((self.bits >> 4) & 1) != 0)
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
#[doc = "I2C Slave I/O Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cs_ack](index.html) module"]
pub struct I2CS_ACK_SPEC;
impl crate::RegisterSpec for I2CS_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cs_ack::R](R) reader structure"]
impl crate::Readable for I2CS_ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cs_ack::W](W) writer structure"]
impl crate::Writable for I2CS_ACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2CS_ACK to value 0"]
impl crate::Resettable for I2CS_ACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
