#[doc = "Register `ID` reader"]
pub struct R(crate::R<ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID` writer"]
pub struct W(crate::W<ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_SPEC>;
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
impl From<crate::W<ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtl_version` reader - RTL Release Version"]
pub type RTL_VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `part_num` reader - Part Number ID"]
pub type PART_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cache_id` reader - Cache ID"]
pub type CACHE_ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - RTL Release Version"]
    #[inline(always)]
    pub fn rtl_version(&self) -> RTL_VERSION_R {
        RTL_VERSION_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - Part Number ID"]
    #[inline(always)]
    pub fn part_num(&self) -> PART_NUM_R {
        PART_NUM_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:15 - Cache ID"]
    #[inline(always)]
    pub fn cache_id(&self) -> CACHE_ID_R {
        CACHE_ID_R::new(((self.bits >> 10) & 0x3f) as u8)
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
#[doc = "Device ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id](index.html) module"]
pub struct ID_SPEC;
impl crate::RegisterSpec for ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id::R](R) reader structure"]
impl crate::Readable for ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id::W](W) writer structure"]
impl crate::Writable for ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ID to value 0"]
impl crate::Resettable for ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
