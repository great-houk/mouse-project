#[doc = "Register `ALI_ACK1` reader"]
pub struct R(crate::R<ALI_ACK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALI_ACK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALI_ACK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALI_ACK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALI_ACK1` writer"]
pub struct W(crate::W<ALI_ACK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALI_ACK1_SPEC>;
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
impl From<crate::W<ALI_ACK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALI_ACK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ali_ack_p4` reader - Analog In Mode Acknowledge: P4\\[7:0\\]"]
pub type ALI_ACK_P4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ali_ack_p5` reader - Analog In Mode Acknowledge: P5\\[7:0\\]"]
pub type ALI_ACK_P5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ali_ack_p6` reader - Analog In Mode Acknowledge: P6\\[0\\]"]
pub type ALI_ACK_P6_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Analog In Mode Acknowledge: P4\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_ack_p4(&self) -> ALI_ACK_P4_R {
        ALI_ACK_P4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Analog In Mode Acknowledge: P5\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_ack_p5(&self) -> ALI_ACK_P5_R {
        ALI_ACK_P5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Analog In Mode Acknowledge: P6\\[0\\]"]
    #[inline(always)]
    pub fn ali_ack_p6(&self) -> ALI_ACK_P6_R {
        ALI_ACK_P6_R::new(((self.bits >> 16) & 1) != 0)
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
#[doc = "Analog Input Acknowledge Register 1 (P4/P5/P6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ali_ack1](index.html) module"]
pub struct ALI_ACK1_SPEC;
impl crate::RegisterSpec for ALI_ACK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ali_ack1::R](R) reader structure"]
impl crate::Readable for ALI_ACK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ali_ack1::W](W) writer structure"]
impl crate::Writable for ALI_ACK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALI_ACK1 to value 0"]
impl crate::Resettable for ALI_ACK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
