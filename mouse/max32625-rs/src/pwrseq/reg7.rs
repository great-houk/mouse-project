#[doc = "Register `REG7` reader"]
pub struct R(crate::R<REG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG7` writer"]
pub struct W(crate::W<REG7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG7_SPEC>;
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
impl From<crate::W<REG7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_flash_pd_lookahead` reader - Flash Powerdown Lookahead Flag"]
pub type PWR_FLASH_PD_LOOKAHEAD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Flash Powerdown Lookahead Flag"]
    #[inline(always)]
    pub fn pwr_flash_pd_lookahead(&self) -> PWR_FLASH_PD_LOOKAHEAD_R {
        PWR_FLASH_PD_LOOKAHEAD_R::new((self.bits & 1) != 0)
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
#[doc = "Power Sequencer Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg7](index.html) module"]
pub struct REG7_SPEC;
impl crate::RegisterSpec for REG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg7::R](R) reader structure"]
impl crate::Readable for REG7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg7::W](W) writer structure"]
impl crate::Writable for REG7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG7 to value 0"]
impl crate::Resettable for REG7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
