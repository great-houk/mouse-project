#[doc = "Register `CTRL_STAT` reader"]
pub struct R(crate::R<CTRL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_STAT` writer"]
pub struct W(crate::W<CTRL_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_STAT_SPEC>;
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
impl From<crate::W<CTRL_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `enable` reader - Cache Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `enable` writer - Cache Enable"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, CTRL_STAT_SPEC, bool, 0>;
#[doc = "Field `ready` reader - Cache Ready Status"]
pub type READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Cache Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Cache Ready Status"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_stat](index.html) module"]
pub struct CTRL_STAT_SPEC;
impl crate::RegisterSpec for CTRL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_stat::R](R) reader structure"]
impl crate::Readable for CTRL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_stat::W](W) writer structure"]
impl crate::Writable for CTRL_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_STAT to value 0"]
impl crate::Resettable for CTRL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
