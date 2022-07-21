#[doc = "Register `ERASE_ALL` writer"]
pub struct W(crate::W<ERASE_ALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASE_ALL_SPEC>;
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
impl From<crate::W<ERASE_ALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASE_ALL_SPEC>) -> Self {
        W(writer)
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
#[doc = "A write to this location triggers an erase of all AES memory locations.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erase_all](index.html) module"]
pub struct ERASE_ALL_SPEC;
impl crate::RegisterSpec for ERASE_ALL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [erase_all::W](W) writer structure"]
impl crate::Writable for ERASE_ALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERASE_ALL to value 0"]
impl crate::Resettable for ERASE_ALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
