#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `flash_lve` reader - Flash LVE Enable"]
pub type FLASH_LVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `flash_lve` writer - Flash LVE Enable"]
pub type FLASH_LVE_W<'a> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 8, 0>;
#[doc = "Field `bypass_ahb_fail` reader - AHB Fail Bypass"]
pub type BYPASS_AHB_FAIL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bypass_ahb_fail` writer - AHB Fail Bypass"]
pub type BYPASS_AHB_FAIL_W<'a> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 8, 8>;
impl R {
    #[doc = "Bits 0:7 - Flash LVE Enable"]
    #[inline(always)]
    pub fn flash_lve(&self) -> FLASH_LVE_R {
        FLASH_LVE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AHB Fail Bypass"]
    #[inline(always)]
    pub fn bypass_ahb_fail(&self) -> BYPASS_AHB_FAIL_R {
        BYPASS_AHB_FAIL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash LVE Enable"]
    #[inline(always)]
    pub fn flash_lve(&mut self) -> FLASH_LVE_W {
        FLASH_LVE_W::new(self)
    }
    #[doc = "Bits 8:15 - AHB Fail Bypass"]
    #[inline(always)]
    pub fn bypass_ahb_fail(&mut self) -> BYPASS_AHB_FAIL_W {
        BYPASS_AHB_FAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
