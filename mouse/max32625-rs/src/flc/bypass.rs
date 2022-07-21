#[doc = "Register `BYPASS` reader"]
pub struct R(crate::R<BYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BYPASS` writer"]
pub struct W(crate::W<BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BYPASS_SPEC>;
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
impl From<crate::W<BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `destruct_bypass_erase` reader - Destructive Security Bypass In Progress"]
pub type DESTRUCT_BYPASS_ERASE_R = crate::BitReader<bool>;
#[doc = "Field `superwipe_erase` reader - Superwipe Erase In Progress"]
pub type SUPERWIPE_ERASE_R = crate::BitReader<bool>;
#[doc = "Field `destruct_bypass_complete` reader - Destructive Security Bypass Erase Complete"]
pub type DESTRUCT_BYPASS_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `superwipe_complete` reader - Superwipe Erase Complete"]
pub type SUPERWIPE_COMPLETE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Destructive Security Bypass In Progress"]
    #[inline(always)]
    pub fn destruct_bypass_erase(&self) -> DESTRUCT_BYPASS_ERASE_R {
        DESTRUCT_BYPASS_ERASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Superwipe Erase In Progress"]
    #[inline(always)]
    pub fn superwipe_erase(&self) -> SUPERWIPE_ERASE_R {
        SUPERWIPE_ERASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Destructive Security Bypass Erase Complete"]
    #[inline(always)]
    pub fn destruct_bypass_complete(&self) -> DESTRUCT_BYPASS_COMPLETE_R {
        DESTRUCT_BYPASS_COMPLETE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Superwipe Erase Complete"]
    #[inline(always)]
    pub fn superwipe_complete(&self) -> SUPERWIPE_COMPLETE_R {
        SUPERWIPE_COMPLETE_R::new(((self.bits >> 3) & 1) != 0)
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
#[doc = "Status Flags for DSB Operations\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bypass](index.html) module"]
pub struct BYPASS_SPEC;
impl crate::RegisterSpec for BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bypass::R](R) reader structure"]
impl crate::Readable for BYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bypass::W](W) writer structure"]
impl crate::Writable for BYPASS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BYPASS to value 0"]
impl crate::Resettable for BYPASS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
