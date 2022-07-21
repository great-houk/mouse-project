#[doc = "Register `BASE_PART_NUM` reader"]
pub struct R(crate::R<BASE_PART_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE_PART_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE_PART_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE_PART_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASE_PART_NUM` writer"]
pub struct W(crate::W<BASE_PART_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASE_PART_NUM_SPEC>;
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
impl From<crate::W<BASE_PART_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASE_PART_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `base_part_number` reader - Base Part Number"]
pub type BASE_PART_NUMBER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Base Part Number"]
    #[inline(always)]
    pub fn base_part_number(&self) -> BASE_PART_NUMBER_R {
        BASE_PART_NUMBER_R::new((self.bits & 0xffff) as u16)
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
#[doc = "Base Part Number\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base_part_num](index.html) module"]
pub struct BASE_PART_NUM_SPEC;
impl crate::RegisterSpec for BASE_PART_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base_part_num::R](R) reader structure"]
impl crate::Readable for BASE_PART_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [base_part_num::W](W) writer structure"]
impl crate::Writable for BASE_PART_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BASE_PART_NUM to value 0"]
impl crate::Resettable for BASE_PART_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
