#[doc = "Register `PWR_MISC` reader"]
pub struct R(crate::R<PWR_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_MISC` writer"]
pub struct W(crate::W<PWR_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_MISC_SPEC>;
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
impl From<crate::W<PWR_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `invert_4_mask_bits` reader - Invert Sense of Power Mask Flags for RTC"]
pub type INVERT_4_MASK_BITS_R = crate::BitReader<bool>;
#[doc = "Field `invert_4_mask_bits` writer - Invert Sense of Power Mask Flags for RTC"]
pub type INVERT_4_MASK_BITS_W<'a> = crate::BitWriter<'a, u32, PWR_MISC_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Invert Sense of Power Mask Flags for RTC"]
    #[inline(always)]
    pub fn invert_4_mask_bits(&self) -> INVERT_4_MASK_BITS_R {
        INVERT_4_MASK_BITS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invert Sense of Power Mask Flags for RTC"]
    #[inline(always)]
    pub fn invert_4_mask_bits(&mut self) -> INVERT_4_MASK_BITS_W {
        INVERT_4_MASK_BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Misc Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_misc](index.html) module"]
pub struct PWR_MISC_SPEC;
impl crate::RegisterSpec for PWR_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_misc::R](R) reader structure"]
impl crate::Readable for PWR_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_misc::W](W) writer structure"]
impl crate::Writable for PWR_MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_MISC to value 0"]
impl crate::Resettable for PWR_MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
