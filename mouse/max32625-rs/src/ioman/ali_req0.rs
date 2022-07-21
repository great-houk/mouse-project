#[doc = "Register `ALI_REQ0` reader"]
pub struct R(crate::R<ALI_REQ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALI_REQ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALI_REQ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALI_REQ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALI_REQ0` writer"]
pub struct W(crate::W<ALI_REQ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALI_REQ0_SPEC>;
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
impl From<crate::W<ALI_REQ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALI_REQ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ali_req_p0` reader - Analog Input Mode Request: P0\\[7:0\\]"]
pub type ALI_REQ_P0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ali_req_p0` writer - Analog Input Mode Request: P0\\[7:0\\]"]
pub type ALI_REQ_P0_W<'a> = crate::FieldWriter<'a, u32, ALI_REQ0_SPEC, u8, u8, 8, 0>;
#[doc = "Field `ali_req_p1` reader - Analog Input Mode Request: P1\\[7:0\\]"]
pub type ALI_REQ_P1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ali_req_p1` writer - Analog Input Mode Request: P1\\[7:0\\]"]
pub type ALI_REQ_P1_W<'a> = crate::FieldWriter<'a, u32, ALI_REQ0_SPEC, u8, u8, 8, 8>;
#[doc = "Field `ali_req_p2` reader - Analog Input Mode Request: P2\\[7:0\\]"]
pub type ALI_REQ_P2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ali_req_p2` writer - Analog Input Mode Request: P2\\[7:0\\]"]
pub type ALI_REQ_P2_W<'a> = crate::FieldWriter<'a, u32, ALI_REQ0_SPEC, u8, u8, 8, 16>;
#[doc = "Field `ali_req_p3` reader - Analog Input Mode Request: P3\\[7:0\\]"]
pub type ALI_REQ_P3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ali_req_p3` writer - Analog Input Mode Request: P3\\[7:0\\]"]
pub type ALI_REQ_P3_W<'a> = crate::FieldWriter<'a, u32, ALI_REQ0_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:7 - Analog Input Mode Request: P0\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p0(&self) -> ALI_REQ_P0_R {
        ALI_REQ_P0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Analog Input Mode Request: P1\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p1(&self) -> ALI_REQ_P1_R {
        ALI_REQ_P1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Analog Input Mode Request: P2\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p2(&self) -> ALI_REQ_P2_R {
        ALI_REQ_P2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Analog Input Mode Request: P3\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p3(&self) -> ALI_REQ_P3_R {
        ALI_REQ_P3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Analog Input Mode Request: P0\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p0(&mut self) -> ALI_REQ_P0_W {
        ALI_REQ_P0_W::new(self)
    }
    #[doc = "Bits 8:15 - Analog Input Mode Request: P1\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p1(&mut self) -> ALI_REQ_P1_W {
        ALI_REQ_P1_W::new(self)
    }
    #[doc = "Bits 16:23 - Analog Input Mode Request: P2\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p2(&mut self) -> ALI_REQ_P2_W {
        ALI_REQ_P2_W::new(self)
    }
    #[doc = "Bits 24:31 - Analog Input Mode Request: P3\\[7:0\\]"]
    #[inline(always)]
    pub fn ali_req_p3(&mut self) -> ALI_REQ_P3_W {
        ALI_REQ_P3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Input Request Register 0 (P0/P1/P2/P3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ali_req0](index.html) module"]
pub struct ALI_REQ0_SPEC;
impl crate::RegisterSpec for ALI_REQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ali_req0::R](R) reader structure"]
impl crate::Readable for ALI_REQ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ali_req0::W](W) writer structure"]
impl crate::Writable for ALI_REQ0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALI_REQ0 to value 0"]
impl crate::Resettable for ALI_REQ0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
