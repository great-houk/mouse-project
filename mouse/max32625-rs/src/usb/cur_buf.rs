#[doc = "Register `CUR_BUF` reader"]
pub struct R(crate::R<CUR_BUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CUR_BUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CUR_BUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CUR_BUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CUR_BUF` writer"]
pub struct W(crate::W<CUR_BUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CUR_BUF_SPEC>;
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
impl From<crate::W<CUR_BUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CUR_BUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `out_buf` reader - OUT Transfer Current Buffers"]
pub type OUT_BUF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `in_buf` reader - IN Transfer Current Buffers"]
pub type IN_BUF_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - OUT Transfer Current Buffers"]
    #[inline(always)]
    pub fn out_buf(&self) -> OUT_BUF_R {
        OUT_BUF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IN Transfer Current Buffers"]
    #[inline(always)]
    pub fn in_buf(&self) -> IN_BUF_R {
        IN_BUF_R::new(((self.bits >> 16) & 0xff) as u8)
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
#[doc = "USB Current Endpoint Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cur_buf](index.html) module"]
pub struct CUR_BUF_SPEC;
impl crate::RegisterSpec for CUR_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cur_buf::R](R) reader structure"]
impl crate::Readable for CUR_BUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cur_buf::W](W) writer structure"]
impl crate::Writable for CUR_BUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CUR_BUF to value 0"]
impl crate::Resettable for CUR_BUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
