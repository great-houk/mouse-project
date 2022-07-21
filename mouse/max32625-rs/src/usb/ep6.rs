#[doc = "Register `EP6` reader"]
pub struct R(crate::R<EP6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP6` writer"]
pub struct W(crate::W<EP6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP6_SPEC>;
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
impl From<crate::W<EP6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ep_dir` reader - Endpoint Direction"]
pub type EP_DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ep_dir` writer - Endpoint Direction"]
pub type EP_DIR_W<'a> = crate::FieldWriter<'a, u32, EP6_SPEC, u8, u8, 2, 0>;
#[doc = "Field `ep_buf2` reader - Endpoint Double Buffered Enable"]
pub type EP_BUF2_R = crate::BitReader<bool>;
#[doc = "Field `ep_buf2` writer - Endpoint Double Buffered Enable"]
pub type EP_BUF2_W<'a> = crate::BitWriter<'a, u32, EP6_SPEC, bool, 3>;
#[doc = "Field `ep_int_en` reader - Endpoint Transfer Complete Interrupt Enable"]
pub type EP_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ep_int_en` writer - Endpoint Transfer Complete Interrupt Enable"]
pub type EP_INT_EN_W<'a> = crate::BitWriter<'a, u32, EP6_SPEC, bool, 4>;
#[doc = "Field `ep_nak_en` reader - Endpoint NAK Interrupt Enable"]
pub type EP_NAK_EN_R = crate::BitReader<bool>;
#[doc = "Field `ep_nak_en` writer - Endpoint NAK Interrupt Enable"]
pub type EP_NAK_EN_W<'a> = crate::BitWriter<'a, u32, EP6_SPEC, bool, 5>;
#[doc = "Field `ep_dt` reader - Endpoint Data Toggle Clear"]
pub type EP_DT_R = crate::BitReader<bool>;
#[doc = "Field `ep_dt` writer - Endpoint Data Toggle Clear"]
pub type EP_DT_W<'a> = crate::BitWriter<'a, u32, EP6_SPEC, bool, 6>;
#[doc = "Field `ep_stall` reader - Endpoint Stall"]
pub type EP_STALL_R = crate::BitReader<bool>;
#[doc = "Field `ep_stall` writer - Endpoint Stall"]
pub type EP_STALL_W<'a> = crate::BitWriter<'a, u32, EP6_SPEC, bool, 8>;
#[doc = "Field `ep_st_stall` reader - Endpoint Stall Status Stage of Control Transfer"]
pub type EP_ST_STALL_R = crate::BitReader<bool>;
#[doc = "Field `ep_st_stall` writer - Endpoint Stall Status Stage of Control Transfer"]
pub type EP_ST_STALL_W<'a> = crate::BitWriter<'a, u32, EP6_SPEC, bool, 9>;
#[doc = "Field `ep_st_ack` reader - Endpoint Acknowledge Status Stage of Control Transfer"]
pub type EP_ST_ACK_R = crate::BitReader<bool>;
#[doc = "Field `ep_st_ack` writer - Endpoint Acknowledge Status Stage of Control Transfer"]
pub type EP_ST_ACK_W<'a> = crate::BitWriter<'a, u32, EP6_SPEC, bool, 10>;
impl R {
    #[doc = "Bits 0:1 - Endpoint Direction"]
    #[inline(always)]
    pub fn ep_dir(&self) -> EP_DIR_R {
        EP_DIR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Endpoint Double Buffered Enable"]
    #[inline(always)]
    pub fn ep_buf2(&self) -> EP_BUF2_R {
        EP_BUF2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ep_int_en(&self) -> EP_INT_EN_R {
        EP_INT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep_nak_en(&self) -> EP_NAK_EN_R {
        EP_NAK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint Data Toggle Clear"]
    #[inline(always)]
    pub fn ep_dt(&self) -> EP_DT_R {
        EP_DT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint Stall"]
    #[inline(always)]
    pub fn ep_stall(&self) -> EP_STALL_R {
        EP_STALL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint Stall Status Stage of Control Transfer"]
    #[inline(always)]
    pub fn ep_st_stall(&self) -> EP_ST_STALL_R {
        EP_ST_STALL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint Acknowledge Status Stage of Control Transfer"]
    #[inline(always)]
    pub fn ep_st_ack(&self) -> EP_ST_ACK_R {
        EP_ST_ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Endpoint Direction"]
    #[inline(always)]
    pub fn ep_dir(&mut self) -> EP_DIR_W {
        EP_DIR_W::new(self)
    }
    #[doc = "Bit 3 - Endpoint Double Buffered Enable"]
    #[inline(always)]
    pub fn ep_buf2(&mut self) -> EP_BUF2_W {
        EP_BUF2_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ep_int_en(&mut self) -> EP_INT_EN_W {
        EP_INT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Endpoint NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep_nak_en(&mut self) -> EP_NAK_EN_W {
        EP_NAK_EN_W::new(self)
    }
    #[doc = "Bit 6 - Endpoint Data Toggle Clear"]
    #[inline(always)]
    pub fn ep_dt(&mut self) -> EP_DT_W {
        EP_DT_W::new(self)
    }
    #[doc = "Bit 8 - Endpoint Stall"]
    #[inline(always)]
    pub fn ep_stall(&mut self) -> EP_STALL_W {
        EP_STALL_W::new(self)
    }
    #[doc = "Bit 9 - Endpoint Stall Status Stage of Control Transfer"]
    #[inline(always)]
    pub fn ep_st_stall(&mut self) -> EP_ST_STALL_W {
        EP_ST_STALL_W::new(self)
    }
    #[doc = "Bit 10 - Endpoint Acknowledge Status Stage of Control Transfer"]
    #[inline(always)]
    pub fn ep_st_ack(&mut self) -> EP_ST_ACK_W {
        EP_ST_ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint 6 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6](index.html) module"]
pub struct EP6_SPEC;
impl crate::RegisterSpec for EP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep6::R](R) reader structure"]
impl crate::Readable for EP6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep6::W](W) writer structure"]
impl crate::Writable for EP6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP6 to value 0"]
impl crate::Resettable for EP6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
