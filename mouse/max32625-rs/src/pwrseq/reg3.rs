#[doc = "Register `REG3` reader"]
pub struct R(crate::R<REG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG3` writer"]
pub struct W(crate::W<REG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG3_SPEC>;
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
impl From<crate::W<REG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_rosel` reader - Relaxation Oscillator Stable Timeout"]
pub type PWR_ROSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_rosel` writer - Relaxation Oscillator Stable Timeout"]
pub type PWR_ROSEL_W<'a> = crate::FieldWriter<'a, u32, REG3_SPEC, u8, u8, 3, 0>;
#[doc = "Field `pwr_fltrrosel` reader - Window of time power must be valid before entering Run mode."]
pub type PWR_FLTRROSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_fltrrosel` writer - Window of time power must be valid before entering Run mode."]
pub type PWR_FLTRROSEL_W<'a> = crate::FieldWriter<'a, u32, REG3_SPEC, u8, u8, 3, 3>;
#[doc = "Field `pwr_svm_clk_mux` reader - SVM Clock Mux"]
pub type PWR_SVM_CLK_MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_svm_clk_mux` writer - SVM Clock Mux"]
pub type PWR_SVM_CLK_MUX_W<'a> = crate::FieldWriter<'a, u32, REG3_SPEC, u8, u8, 2, 6>;
#[doc = "Field `pwr_ro_clk_mux` reader - Relaxation Clock Mux"]
pub type PWR_RO_CLK_MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_ro_clk_mux` writer - Relaxation Clock Mux"]
pub type PWR_RO_CLK_MUX_W<'a> = crate::FieldWriter<'a, u32, REG3_SPEC, u8, u8, 2, 8>;
#[doc = "Field `pwr_failsel` reader - Timeout before rebooting during PowerFail/BootFail events."]
pub type PWR_FAILSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_failsel` writer - Timeout before rebooting during PowerFail/BootFail events."]
pub type PWR_FAILSEL_W<'a> = crate::FieldWriter<'a, u32, REG3_SPEC, u8, u8, 3, 10>;
#[doc = "Field `pwr_ro_div` reader - Frequency Divider for 96MHz Relaxation Oscillator"]
pub type PWR_RO_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_ro_div` writer - Frequency Divider for 96MHz Relaxation Oscillator"]
pub type PWR_RO_DIV_W<'a> = crate::FieldWriter<'a, u32, REG3_SPEC, u8, u8, 3, 16>;
#[doc = "Field `pwr_rc_div` reader - Frequency Divider for 4MHz RC Oscillator"]
pub type PWR_RC_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_rc_div` writer - Frequency Divider for 4MHz RC Oscillator"]
pub type PWR_RC_DIV_W<'a> = crate::FieldWriter<'a, u32, REG3_SPEC, u8, u8, 2, 20>;
impl R {
    #[doc = "Bits 0:2 - Relaxation Oscillator Stable Timeout"]
    #[inline(always)]
    pub fn pwr_rosel(&self) -> PWR_ROSEL_R {
        PWR_ROSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Window of time power must be valid before entering Run mode."]
    #[inline(always)]
    pub fn pwr_fltrrosel(&self) -> PWR_FLTRROSEL_R {
        PWR_FLTRROSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - SVM Clock Mux"]
    #[inline(always)]
    pub fn pwr_svm_clk_mux(&self) -> PWR_SVM_CLK_MUX_R {
        PWR_SVM_CLK_MUX_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Relaxation Clock Mux"]
    #[inline(always)]
    pub fn pwr_ro_clk_mux(&self) -> PWR_RO_CLK_MUX_R {
        PWR_RO_CLK_MUX_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Timeout before rebooting during PowerFail/BootFail events."]
    #[inline(always)]
    pub fn pwr_failsel(&self) -> PWR_FAILSEL_R {
        PWR_FAILSEL_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Frequency Divider for 96MHz Relaxation Oscillator"]
    #[inline(always)]
    pub fn pwr_ro_div(&self) -> PWR_RO_DIV_R {
        PWR_RO_DIV_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Frequency Divider for 4MHz RC Oscillator"]
    #[inline(always)]
    pub fn pwr_rc_div(&self) -> PWR_RC_DIV_R {
        PWR_RC_DIV_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Relaxation Oscillator Stable Timeout"]
    #[inline(always)]
    pub fn pwr_rosel(&mut self) -> PWR_ROSEL_W {
        PWR_ROSEL_W::new(self)
    }
    #[doc = "Bits 3:5 - Window of time power must be valid before entering Run mode."]
    #[inline(always)]
    pub fn pwr_fltrrosel(&mut self) -> PWR_FLTRROSEL_W {
        PWR_FLTRROSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - SVM Clock Mux"]
    #[inline(always)]
    pub fn pwr_svm_clk_mux(&mut self) -> PWR_SVM_CLK_MUX_W {
        PWR_SVM_CLK_MUX_W::new(self)
    }
    #[doc = "Bits 8:9 - Relaxation Clock Mux"]
    #[inline(always)]
    pub fn pwr_ro_clk_mux(&mut self) -> PWR_RO_CLK_MUX_W {
        PWR_RO_CLK_MUX_W::new(self)
    }
    #[doc = "Bits 10:12 - Timeout before rebooting during PowerFail/BootFail events."]
    #[inline(always)]
    pub fn pwr_failsel(&mut self) -> PWR_FAILSEL_W {
        PWR_FAILSEL_W::new(self)
    }
    #[doc = "Bits 16:18 - Frequency Divider for 96MHz Relaxation Oscillator"]
    #[inline(always)]
    pub fn pwr_ro_div(&mut self) -> PWR_RO_DIV_W {
        PWR_RO_DIV_W::new(self)
    }
    #[doc = "Bits 20:21 - Frequency Divider for 4MHz RC Oscillator"]
    #[inline(always)]
    pub fn pwr_rc_div(&mut self) -> PWR_RC_DIV_W {
        PWR_RC_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg3](index.html) module"]
pub struct REG3_SPEC;
impl crate::RegisterSpec for REG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg3::R](R) reader structure"]
impl crate::Readable for REG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg3::W](W) writer structure"]
impl crate::Writable for REG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG3 to value 0"]
impl crate::Resettable for REG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
