#[doc = "Register `MASK_ID0` reader"]
pub struct R(crate::R<MASK_ID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_ID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_ID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_ID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK_ID0` writer"]
pub struct W(crate::W<MASK_ID0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_ID0_SPEC>;
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
impl From<crate::W<MASK_ID0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_ID0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `revision_id` reader - Revision ID"]
pub type REVISION_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mask_id` reader - Mask ID\\[27:0\\]"]
pub type MASK_ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:3 - Revision ID"]
    #[inline(always)]
    pub fn revision_id(&self) -> REVISION_ID_R {
        REVISION_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - Mask ID\\[27:0\\]"]
    #[inline(always)]
    pub fn mask_id(&self) -> MASK_ID_R {
        MASK_ID_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
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
#[doc = "Mask ID Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask_id0](index.html) module"]
pub struct MASK_ID0_SPEC;
impl crate::RegisterSpec for MASK_ID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask_id0::R](R) reader structure"]
impl crate::Readable for MASK_ID0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask_id0::W](W) writer structure"]
impl crate::Writable for MASK_ID0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK_ID0 to value 0"]
impl crate::Resettable for MASK_ID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
