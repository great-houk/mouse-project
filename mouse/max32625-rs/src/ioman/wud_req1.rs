#[doc = "Register `WUD_REQ1` reader"]
pub struct R(crate::R<WUD_REQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUD_REQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUD_REQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUD_REQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUD_REQ1` writer"]
pub struct W(crate::W<WUD_REQ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUD_REQ1_SPEC>;
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
impl From<crate::W<WUD_REQ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUD_REQ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wud_req_p4` reader - Wakeup Detect Request Mode: P4\\[7:0\\]"]
pub type WUD_REQ_P4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wud_req_p4` writer - Wakeup Detect Request Mode: P4\\[7:0\\]"]
pub type WUD_REQ_P4_W<'a> = crate::FieldWriter<'a, u32, WUD_REQ1_SPEC, u8, u8, 8, 0>;
#[doc = "Field `wud_req_p5` reader - Wakeup Detect Request Mode: P5\\[7:0\\]"]
pub type WUD_REQ_P5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wud_req_p5` writer - Wakeup Detect Request Mode: P5\\[7:0\\]"]
pub type WUD_REQ_P5_W<'a> = crate::FieldWriter<'a, u32, WUD_REQ1_SPEC, u8, u8, 8, 8>;
#[doc = "Field `wud_req_p6` reader - Wakeup Detect Request Mode: P6\\[0\\]"]
pub type WUD_REQ_P6_R = crate::BitReader<bool>;
#[doc = "Field `wud_req_p6` writer - Wakeup Detect Request Mode: P6\\[0\\]"]
pub type WUD_REQ_P6_W<'a> = crate::BitWriter<'a, u32, WUD_REQ1_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:7 - Wakeup Detect Request Mode: P4\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_req_p4(&self) -> WUD_REQ_P4_R {
        WUD_REQ_P4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Wakeup Detect Request Mode: P5\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_req_p5(&self) -> WUD_REQ_P5_R {
        WUD_REQ_P5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Wakeup Detect Request Mode: P6\\[0\\]"]
    #[inline(always)]
    pub fn wud_req_p6(&self) -> WUD_REQ_P6_R {
        WUD_REQ_P6_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wakeup Detect Request Mode: P4\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_req_p4(&mut self) -> WUD_REQ_P4_W {
        WUD_REQ_P4_W::new(self)
    }
    #[doc = "Bits 8:15 - Wakeup Detect Request Mode: P5\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_req_p5(&mut self) -> WUD_REQ_P5_W {
        WUD_REQ_P5_W::new(self)
    }
    #[doc = "Bit 16 - Wakeup Detect Request Mode: P6\\[0\\]"]
    #[inline(always)]
    pub fn wud_req_p6(&mut self) -> WUD_REQ_P6_W {
        WUD_REQ_P6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup Detect Mode Request Register 1 (P4/P5/P6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wud_req1](index.html) module"]
pub struct WUD_REQ1_SPEC;
impl crate::RegisterSpec for WUD_REQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wud_req1::R](R) reader structure"]
impl crate::Readable for WUD_REQ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wud_req1::W](W) writer structure"]
impl crate::Writable for WUD_REQ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUD_REQ1 to value 0"]
impl crate::Resettable for WUD_REQ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
