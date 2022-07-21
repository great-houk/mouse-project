#[doc = "Register `CLK_CTRL` reader"]
pub struct R(crate::R<CLK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL` writer"]
pub struct W(crate::W<CLK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL_SPEC>;
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
impl From<crate::W<CLK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `system_source_select` reader - System Clock Source Select"]
pub type SYSTEM_SOURCE_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `system_source_select` writer - System Clock Source Select"]
pub type SYSTEM_SOURCE_SELECT_W<'a> = crate::FieldWriter<'a, u32, CLK_CTRL_SPEC, u8, u8, 2, 0>;
#[doc = "Field `usb_clock_enable` reader - USB Clock Enable"]
pub type USB_CLOCK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `usb_clock_enable` writer - USB Clock Enable"]
pub type USB_CLOCK_ENABLE_W<'a> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, 4>;
#[doc = "Field `usb_clock_select` reader - USB Clock Select"]
pub type USB_CLOCK_SELECT_R = crate::BitReader<bool>;
#[doc = "Field `usb_clock_select` writer - USB Clock Select"]
pub type USB_CLOCK_SELECT_W<'a> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, 5>;
#[doc = "Field `crypto_clock_enable` reader - Crypto Clock Enable"]
pub type CRYPTO_CLOCK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `crypto_clock_enable` writer - Crypto Clock Enable"]
pub type CRYPTO_CLOCK_ENABLE_W<'a> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, 8>;
#[doc = "Field `rtos_mode` reader - Enable RTOS Mode for SysTick Timers"]
pub type RTOS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `rtos_mode` writer - Enable RTOS Mode for SysTick Timers"]
pub type RTOS_MODE_W<'a> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, 12>;
#[doc = "Field `cpu_dynamic_clock` reader - Enable CPU Dynamic Clock Gating"]
pub type CPU_DYNAMIC_CLOCK_R = crate::BitReader<bool>;
#[doc = "Field `cpu_dynamic_clock` writer - Enable CPU Dynamic Clock Gating"]
pub type CPU_DYNAMIC_CLOCK_W<'a> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, 13>;
#[doc = "Field `wdt0_clock_enable` reader - Watchdog 0 Clock Enable"]
pub type WDT0_CLOCK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `wdt0_clock_enable` writer - Watchdog 0 Clock Enable"]
pub type WDT0_CLOCK_ENABLE_W<'a> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, 16>;
#[doc = "Field `wdt0_clock_select` reader - Watchdog 0 Clock Source Select"]
pub type WDT0_CLOCK_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wdt0_clock_select` writer - Watchdog 0 Clock Source Select"]
pub type WDT0_CLOCK_SELECT_W<'a> = crate::FieldWriter<'a, u32, CLK_CTRL_SPEC, u8, u8, 2, 17>;
#[doc = "Field `wdt1_clock_enable` reader - Watchdog 1 Clock Enable"]
pub type WDT1_CLOCK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `wdt1_clock_enable` writer - Watchdog 1 Clock Enable"]
pub type WDT1_CLOCK_ENABLE_W<'a> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, 20>;
#[doc = "Field `wdt1_clock_select` reader - Watchdog 1 Clock Source Select"]
pub type WDT1_CLOCK_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `wdt1_clock_select` writer - Watchdog 1 Clock Source Select"]
pub type WDT1_CLOCK_SELECT_W<'a> = crate::FieldWriter<'a, u32, CLK_CTRL_SPEC, u8, u8, 2, 21>;
#[doc = "Field `adc_clock_enable` reader - ADC Clock Enable"]
pub type ADC_CLOCK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `adc_clock_enable` writer - ADC Clock Enable"]
pub type ADC_CLOCK_ENABLE_W<'a> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, 24>;
impl R {
    #[doc = "Bits 0:1 - System Clock Source Select"]
    #[inline(always)]
    pub fn system_source_select(&self) -> SYSTEM_SOURCE_SELECT_R {
        SYSTEM_SOURCE_SELECT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - USB Clock Enable"]
    #[inline(always)]
    pub fn usb_clock_enable(&self) -> USB_CLOCK_ENABLE_R {
        USB_CLOCK_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB Clock Select"]
    #[inline(always)]
    pub fn usb_clock_select(&self) -> USB_CLOCK_SELECT_R {
        USB_CLOCK_SELECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Crypto Clock Enable"]
    #[inline(always)]
    pub fn crypto_clock_enable(&self) -> CRYPTO_CLOCK_ENABLE_R {
        CRYPTO_CLOCK_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable RTOS Mode for SysTick Timers"]
    #[inline(always)]
    pub fn rtos_mode(&self) -> RTOS_MODE_R {
        RTOS_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable CPU Dynamic Clock Gating"]
    #[inline(always)]
    pub fn cpu_dynamic_clock(&self) -> CPU_DYNAMIC_CLOCK_R {
        CPU_DYNAMIC_CLOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Watchdog 0 Clock Enable"]
    #[inline(always)]
    pub fn wdt0_clock_enable(&self) -> WDT0_CLOCK_ENABLE_R {
        WDT0_CLOCK_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Watchdog 0 Clock Source Select"]
    #[inline(always)]
    pub fn wdt0_clock_select(&self) -> WDT0_CLOCK_SELECT_R {
        WDT0_CLOCK_SELECT_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20 - Watchdog 1 Clock Enable"]
    #[inline(always)]
    pub fn wdt1_clock_enable(&self) -> WDT1_CLOCK_ENABLE_R {
        WDT1_CLOCK_ENABLE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Watchdog 1 Clock Source Select"]
    #[inline(always)]
    pub fn wdt1_clock_select(&self) -> WDT1_CLOCK_SELECT_R {
        WDT1_CLOCK_SELECT_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - ADC Clock Enable"]
    #[inline(always)]
    pub fn adc_clock_enable(&self) -> ADC_CLOCK_ENABLE_R {
        ADC_CLOCK_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System Clock Source Select"]
    #[inline(always)]
    pub fn system_source_select(&mut self) -> SYSTEM_SOURCE_SELECT_W {
        SYSTEM_SOURCE_SELECT_W::new(self)
    }
    #[doc = "Bit 4 - USB Clock Enable"]
    #[inline(always)]
    pub fn usb_clock_enable(&mut self) -> USB_CLOCK_ENABLE_W {
        USB_CLOCK_ENABLE_W::new(self)
    }
    #[doc = "Bit 5 - USB Clock Select"]
    #[inline(always)]
    pub fn usb_clock_select(&mut self) -> USB_CLOCK_SELECT_W {
        USB_CLOCK_SELECT_W::new(self)
    }
    #[doc = "Bit 8 - Crypto Clock Enable"]
    #[inline(always)]
    pub fn crypto_clock_enable(&mut self) -> CRYPTO_CLOCK_ENABLE_W {
        CRYPTO_CLOCK_ENABLE_W::new(self)
    }
    #[doc = "Bit 12 - Enable RTOS Mode for SysTick Timers"]
    #[inline(always)]
    pub fn rtos_mode(&mut self) -> RTOS_MODE_W {
        RTOS_MODE_W::new(self)
    }
    #[doc = "Bit 13 - Enable CPU Dynamic Clock Gating"]
    #[inline(always)]
    pub fn cpu_dynamic_clock(&mut self) -> CPU_DYNAMIC_CLOCK_W {
        CPU_DYNAMIC_CLOCK_W::new(self)
    }
    #[doc = "Bit 16 - Watchdog 0 Clock Enable"]
    #[inline(always)]
    pub fn wdt0_clock_enable(&mut self) -> WDT0_CLOCK_ENABLE_W {
        WDT0_CLOCK_ENABLE_W::new(self)
    }
    #[doc = "Bits 17:18 - Watchdog 0 Clock Source Select"]
    #[inline(always)]
    pub fn wdt0_clock_select(&mut self) -> WDT0_CLOCK_SELECT_W {
        WDT0_CLOCK_SELECT_W::new(self)
    }
    #[doc = "Bit 20 - Watchdog 1 Clock Enable"]
    #[inline(always)]
    pub fn wdt1_clock_enable(&mut self) -> WDT1_CLOCK_ENABLE_W {
        WDT1_CLOCK_ENABLE_W::new(self)
    }
    #[doc = "Bits 21:22 - Watchdog 1 Clock Source Select"]
    #[inline(always)]
    pub fn wdt1_clock_select(&mut self) -> WDT1_CLOCK_SELECT_W {
        WDT1_CLOCK_SELECT_W::new(self)
    }
    #[doc = "Bit 24 - ADC Clock Enable"]
    #[inline(always)]
    pub fn adc_clock_enable(&mut self) -> ADC_CLOCK_ENABLE_W {
        ADC_CLOCK_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl](index.html) module"]
pub struct CLK_CTRL_SPEC;
impl crate::RegisterSpec for CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl::R](R) reader structure"]
impl crate::Readable for CLK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl::W](W) writer structure"]
impl crate::Writable for CLK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CTRL to value 0"]
impl crate::Resettable for CLK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
