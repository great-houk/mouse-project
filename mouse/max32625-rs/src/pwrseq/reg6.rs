#[doc = "Register `REG6` reader"]
pub struct R(crate::R<REG6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG6` writer"]
pub struct W(crate::W<REG6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG6_SPEC>;
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
impl From<crate::W<REG6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_trim_usb_bias` reader - USB Bias Current trim setting"]
pub type PWR_TRIM_USB_BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_trim_usb_bias` writer - USB Bias Current trim setting"]
pub type PWR_TRIM_USB_BIAS_W<'a> = crate::FieldWriter<'a, u32, REG6_SPEC, u8, u8, 3, 0>;
#[doc = "Field `pwr_trim_usb_pm_res` reader - USB Data Plus Slew Rate trim setting"]
pub type PWR_TRIM_USB_PM_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_trim_usb_pm_res` writer - USB Data Plus Slew Rate trim setting"]
pub type PWR_TRIM_USB_PM_RES_W<'a> = crate::FieldWriter<'a, u32, REG6_SPEC, u8, u8, 4, 3>;
#[doc = "Field `pwr_trim_usb_dm_res` reader - USB Data Minus Slew Rate trim setting"]
pub type PWR_TRIM_USB_DM_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pwr_trim_usb_dm_res` writer - USB Data Minus Slew Rate trim setting"]
pub type PWR_TRIM_USB_DM_RES_W<'a> = crate::FieldWriter<'a, u32, REG6_SPEC, u8, u8, 4, 7>;
#[doc = "Field `pwr_trim_osc_vref` reader - Relaxation Oscillator trim setting"]
pub type PWR_TRIM_OSC_VREF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwr_trim_osc_vref` writer - Relaxation Oscillator trim setting"]
pub type PWR_TRIM_OSC_VREF_W<'a> = crate::FieldWriter<'a, u32, REG6_SPEC, u16, u16, 9, 11>;
#[doc = "Field `pwr_trim_crypto_osc` reader - Crypto Oscillator trim setting"]
pub type PWR_TRIM_CRYPTO_OSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwr_trim_crypto_osc` writer - Crypto Oscillator trim setting"]
pub type PWR_TRIM_CRYPTO_OSC_W<'a> = crate::FieldWriter<'a, u32, REG6_SPEC, u16, u16, 9, 20>;
impl R {
    #[doc = "Bits 0:2 - USB Bias Current trim setting"]
    #[inline(always)]
    pub fn pwr_trim_usb_bias(&self) -> PWR_TRIM_USB_BIAS_R {
        PWR_TRIM_USB_BIAS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - USB Data Plus Slew Rate trim setting"]
    #[inline(always)]
    pub fn pwr_trim_usb_pm_res(&self) -> PWR_TRIM_USB_PM_RES_R {
        PWR_TRIM_USB_PM_RES_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - USB Data Minus Slew Rate trim setting"]
    #[inline(always)]
    pub fn pwr_trim_usb_dm_res(&self) -> PWR_TRIM_USB_DM_RES_R {
        PWR_TRIM_USB_DM_RES_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 11:19 - Relaxation Oscillator trim setting"]
    #[inline(always)]
    pub fn pwr_trim_osc_vref(&self) -> PWR_TRIM_OSC_VREF_R {
        PWR_TRIM_OSC_VREF_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:28 - Crypto Oscillator trim setting"]
    #[inline(always)]
    pub fn pwr_trim_crypto_osc(&self) -> PWR_TRIM_CRYPTO_OSC_R {
        PWR_TRIM_CRYPTO_OSC_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Bias Current trim setting"]
    #[inline(always)]
    pub fn pwr_trim_usb_bias(&mut self) -> PWR_TRIM_USB_BIAS_W {
        PWR_TRIM_USB_BIAS_W::new(self)
    }
    #[doc = "Bits 3:6 - USB Data Plus Slew Rate trim setting"]
    #[inline(always)]
    pub fn pwr_trim_usb_pm_res(&mut self) -> PWR_TRIM_USB_PM_RES_W {
        PWR_TRIM_USB_PM_RES_W::new(self)
    }
    #[doc = "Bits 7:10 - USB Data Minus Slew Rate trim setting"]
    #[inline(always)]
    pub fn pwr_trim_usb_dm_res(&mut self) -> PWR_TRIM_USB_DM_RES_W {
        PWR_TRIM_USB_DM_RES_W::new(self)
    }
    #[doc = "Bits 11:19 - Relaxation Oscillator trim setting"]
    #[inline(always)]
    pub fn pwr_trim_osc_vref(&mut self) -> PWR_TRIM_OSC_VREF_W {
        PWR_TRIM_OSC_VREF_W::new(self)
    }
    #[doc = "Bits 20:28 - Crypto Oscillator trim setting"]
    #[inline(always)]
    pub fn pwr_trim_crypto_osc(&mut self) -> PWR_TRIM_CRYPTO_OSC_W {
        PWR_TRIM_CRYPTO_OSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Control Register 6 (Trim 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg6](index.html) module"]
pub struct REG6_SPEC;
impl crate::RegisterSpec for REG6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg6::R](R) reader structure"]
impl crate::Readable for REG6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg6::W](W) writer structure"]
impl crate::Writable for REG6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG6 to value 0"]
impl crate::Resettable for REG6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
