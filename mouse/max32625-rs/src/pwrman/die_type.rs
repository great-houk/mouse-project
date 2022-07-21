#[doc = "Register `DIE_TYPE` reader"]
pub struct R(crate::R<DIE_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIE_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIE_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIE_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIE_TYPE` writer"]
pub struct W(crate::W<DIE_TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIE_TYPE_SPEC>;
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
impl From<crate::W<DIE_TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIE_TYPE_SPEC>) -> Self {
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
#[doc = "Die Type ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [die_type](index.html) module"]
pub struct DIE_TYPE_SPEC;
impl crate::RegisterSpec for DIE_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [die_type::R](R) reader structure"]
impl crate::Readable for DIE_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [die_type::W](W) writer structure"]
impl crate::Writable for DIE_TYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIE_TYPE to value 0"]
impl crate::Resettable for DIE_TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
