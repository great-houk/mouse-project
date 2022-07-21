#[doc = "Register `USER_OPTION` reader"]
pub struct R(crate::R<USER_OPTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER_OPTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER_OPTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER_OPTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER_OPTION` writer"]
pub struct W(crate::W<USER_OPTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER_OPTION_SPEC>;
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
impl From<crate::W<USER_OPTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER_OPTION_SPEC>) -> Self {
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
#[doc = "Used to set DSB Access code and Auto-Lock in info block\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user_option](index.html) module"]
pub struct USER_OPTION_SPEC;
impl crate::RegisterSpec for USER_OPTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user_option::R](R) reader structure"]
impl crate::Readable for USER_OPTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user_option::W](W) writer structure"]
impl crate::Writable for USER_OPTION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USER_OPTION to value 0"]
impl crate::Resettable for USER_OPTION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
