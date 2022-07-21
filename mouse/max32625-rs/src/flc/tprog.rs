#[doc = "Register `TPROG` reader"]
pub struct R(crate::R<TPROG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPROG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPROG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPROG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPROG` writer"]
pub struct W(crate::W<TPROG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPROG_SPEC>;
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
impl From<crate::W<TPROG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPROG_SPEC>) -> Self {
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
#[doc = "Flash Write Cycle Config\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tprog](index.html) module"]
pub struct TPROG_SPEC;
impl crate::RegisterSpec for TPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tprog::R](R) reader structure"]
impl crate::Readable for TPROG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tprog::W](W) writer structure"]
impl crate::Writable for TPROG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPROG to value 0"]
impl crate::Resettable for TPROG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
