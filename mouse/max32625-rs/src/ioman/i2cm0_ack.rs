#[doc = "Register `I2CM0_ACK` reader"]
pub struct R(crate::R<I2CM0_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CM0_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CM0_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CM0_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CM0_ACK` writer"]
pub struct W(crate::W<I2CM0_ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CM0_ACK_SPEC>;
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
impl From<crate::W<I2CM0_ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CM0_ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mapping_ack` reader - I2C Master 0 I/O Acknowledge"]
pub type MAPPING_ACK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 4 - I2C Master 0 I/O Acknowledge"]
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
#[doc = "I2C Master 0 I/O Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cm0_ack](index.html) module"]
pub struct I2CM0_ACK_SPEC;
impl crate::RegisterSpec for I2CM0_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cm0_ack::R](R) reader structure"]
impl crate::Readable for I2CM0_ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cm0_ack::W](W) writer structure"]
impl crate::Writable for I2CM0_ACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2CM0_ACK to value 0"]
impl crate::Resettable for I2CM0_ACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
