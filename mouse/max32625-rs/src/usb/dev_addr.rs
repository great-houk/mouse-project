#[doc = "Register `DEV_ADDR` reader"]
pub struct R(crate::R<DEV_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEV_ADDR` writer"]
pub struct W(crate::W<DEV_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEV_ADDR_SPEC>;
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
impl From<crate::W<DEV_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEV_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dev_addr` reader - USB Device Address"]
pub type DEV_ADDR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - USB Device Address"]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new((self.bits & 0x7f) as u8)
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
#[doc = "USB Device Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_addr](index.html) module"]
pub struct DEV_ADDR_SPEC;
impl crate::RegisterSpec for DEV_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_addr::R](R) reader structure"]
impl crate::Readable for DEV_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dev_addr::W](W) writer structure"]
impl crate::Writable for DEV_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEV_ADDR to value 0"]
impl crate::Resettable for DEV_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
