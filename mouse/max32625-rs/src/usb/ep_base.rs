#[doc = "Register `EP_BASE` reader"]
pub struct R(crate::R<EP_BASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP_BASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP_BASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP_BASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP_BASE` writer"]
pub struct W(crate::W<EP_BASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP_BASE_SPEC>;
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
impl From<crate::W<EP_BASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP_BASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ep_base` reader - USB Endpoint Descriptor Table Base Address"]
pub type EP_BASE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ep_base` writer - USB Endpoint Descriptor Table Base Address"]
pub type EP_BASE_W<'a> = crate::FieldWriter<'a, u32, EP_BASE_SPEC, u32, u32, 23, 9>;
impl R {
    #[doc = "Bits 9:31 - USB Endpoint Descriptor Table Base Address"]
    #[inline(always)]
    pub fn ep_base(&self) -> EP_BASE_R {
        EP_BASE_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 9:31 - USB Endpoint Descriptor Table Base Address"]
    #[inline(always)]
    pub fn ep_base(&mut self) -> EP_BASE_W {
        EP_BASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint Descriptor Table Base Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep_base](index.html) module"]
pub struct EP_BASE_SPEC;
impl crate::RegisterSpec for EP_BASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep_base::R](R) reader structure"]
impl crate::Readable for EP_BASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep_base::W](W) writer structure"]
impl crate::Writable for EP_BASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP_BASE to value 0"]
impl crate::Resettable for EP_BASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
