#[doc = "Register `ALI_REQ1` reader"]
pub struct R(crate::R<ALI_REQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALI_REQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALI_REQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALI_REQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALI_REQ1` writer"]
pub struct W(crate::W<ALI_REQ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALI_REQ1_SPEC>;
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
impl From<crate::W<ALI_REQ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALI_REQ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ali_req_p4` reader - Analog Input Mode Request: P4\\[7:0\\]"]
pub type ALI_REQ_P4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ali_req_p4` writer - Analog Input Mode Request: P4\\[7:0\\]"]
pub type ALI_REQ_P4_W<'a> = crate::FieldWriter<'a, u32, ALI_REQ1_SPEC, u8, u8, 8, 0>;
#[doc = "Field `ali_req_p5` reader - Analog Input Mode Request: P5\\[7:0\\]"]
pub type ALI_REQ_P5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ali_req_p5` writer - Analog Input Mode Request: P5\\[7:0\\]"]
pub type ALI_REQ_P5_W<'a> = crate::FieldWriter<'a, u32, ALI_REQ1_SPEC, u8, u8, 8, 8>;
#[doc = "Field `ali_req_p6` reader - Analog Input Mode Request: P6\\[0\\]"]
pub type ALI_REQ_P6_R = crate::BitReader<bool>;
#[doc = "Field `ali_req_p6` writer - Analog Input Mode Request: P6\\[0\\]"]
pub type ALI_REQ_P6_W<'a> = crate::BitWriter<'a, u32, ALI_REQ1_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:7 - Analog Input Mode Request: P4\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p4(&self) -> ALI_REQ_P4_R {
        ALI_REQ_P4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Analog Input Mode Request: P5\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p5(&self) -> ALI_REQ_P5_R {
        ALI_REQ_P5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Analog Input Mode Request: P6\\[0\\]"]
    #[inline(always)]
    pub fn ali_req_p6(&self) -> ALI_REQ_P6_R {
        ALI_REQ_P6_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Analog Input Mode Request: P4\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p4(&mut self) -> ALI_REQ_P4_W {
        ALI_REQ_P4_W::new(self)
    }
    #[doc = "Bits 8:15 - Analog Input Mode Request: P5\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p5(&mut self) -> ALI_REQ_P5_W {
        ALI_REQ_P5_W::new(self)
    }
    #[doc = "Bit 16 - Analog Input Mode Request: P6\\[0\\]"]
    #[inline(always)]
    pub fn ali_req_p6(&mut self) -> ALI_REQ_P6_W {
        ALI_REQ_P6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Input Request Register 1 (P4/P5/P6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ali_req1](index.html) module"]
pub struct ALI_REQ1_SPEC;
impl crate::RegisterSpec for ALI_REQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ali_req1::R](R) reader structure"]
impl crate::Readable for ALI_REQ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ali_req1::W](W) writer structure"]
impl crate::Writable for ALI_REQ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALI_REQ1 to value 0"]
impl crate::Resettable for ALI_REQ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
