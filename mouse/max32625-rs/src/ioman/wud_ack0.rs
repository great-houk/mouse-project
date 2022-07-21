#[doc = "Register `WUD_ACK0` reader"]
pub struct R(crate::R<WUD_ACK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUD_ACK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUD_ACK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUD_ACK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUD_ACK0` writer"]
pub struct W(crate::W<WUD_ACK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUD_ACK0_SPEC>;
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
impl From<crate::W<WUD_ACK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUD_ACK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wud_ack_p0` reader - WUD Mode Acknowledge: P0\\[7:0\\]"]
pub type WUD_ACK_P0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wud_ack_p1` reader - WUD Mode Acknowledge: P1\\[7:0\\]"]
pub type WUD_ACK_P1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wud_ack_p2` reader - WUD Mode Acknowledge: P2\\[7:0\\]"]
pub type WUD_ACK_P2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wud_ack_p3` reader - WUD Mode Acknowledge: P3\\[7:0\\]"]
pub type WUD_ACK_P3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - WUD Mode Acknowledge: P0\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_ack_p0(&self) -> WUD_ACK_P0_R {
        WUD_ACK_P0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - WUD Mode Acknowledge: P1\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_ack_p1(&self) -> WUD_ACK_P1_R {
        WUD_ACK_P1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - WUD Mode Acknowledge: P2\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_ack_p2(&self) -> WUD_ACK_P2_R {
        WUD_ACK_P2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - WUD Mode Acknowledge: P3\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_ack_p3(&self) -> WUD_ACK_P3_R {
        WUD_ACK_P3_R::new(((self.bits >> 24) & 0xff) as u8)
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
#[doc = "Wakeup Detect Mode Acknowledge Register 0 (P0/P1/P2/P3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wud_ack0](index.html) module"]
pub struct WUD_ACK0_SPEC;
impl crate::RegisterSpec for WUD_ACK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wud_ack0::R](R) reader structure"]
impl crate::Readable for WUD_ACK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wud_ack0::W](W) writer structure"]
impl crate::Writable for WUD_ACK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUD_ACK0 to value 0"]
impl crate::Resettable for WUD_ACK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
