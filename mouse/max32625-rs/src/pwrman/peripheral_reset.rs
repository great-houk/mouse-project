#[doc = "Register `PERIPHERAL_RESET` reader"]
pub struct R(crate::R<PERIPHERAL_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPHERAL_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPHERAL_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPHERAL_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIPHERAL_RESET` writer"]
pub struct W(crate::W<PERIPHERAL_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIPHERAL_RESET_SPEC>;
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
impl From<crate::W<PERIPHERAL_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIPHERAL_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ssb` reader - Reset SSB"]
pub type SSB_R = crate::BitReader<bool>;
#[doc = "Field `ssb` writer - Reset SSB"]
pub type SSB_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 0>;
#[doc = "Field `spix` reader - Reset SPI XIP"]
pub type SPIX_R = crate::BitReader<bool>;
#[doc = "Field `spix` writer - Reset SPI XIP"]
pub type SPIX_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 1>;
#[doc = "Field `pmu` reader - Reset PMU"]
pub type PMU_R = crate::BitReader<bool>;
#[doc = "Field `pmu` writer - Reset PMU"]
pub type PMU_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 2>;
#[doc = "Field `usb` reader - Reset USB"]
pub type USB_R = crate::BitReader<bool>;
#[doc = "Field `usb` writer - Reset USB"]
pub type USB_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 3>;
#[doc = "Field `crc` reader - Reset CRC"]
pub type CRC_R = crate::BitReader<bool>;
#[doc = "Field `crc` writer - Reset CRC"]
pub type CRC_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 4>;
#[doc = "Field `tpu` reader - Reset TPU"]
pub type TPU_R = crate::BitReader<bool>;
#[doc = "Field `tpu` writer - Reset TPU"]
pub type TPU_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 5>;
#[doc = "Field `watchdog0` reader - Reset Watchdog Timer 0"]
pub type WATCHDOG0_R = crate::BitReader<bool>;
#[doc = "Field `watchdog0` writer - Reset Watchdog Timer 0"]
pub type WATCHDOG0_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 6>;
#[doc = "Field `gpio` reader - Reset GPIO"]
pub type GPIO_R = crate::BitReader<bool>;
#[doc = "Field `gpio` writer - Reset GPIO"]
pub type GPIO_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 7>;
#[doc = "Field `timer0` reader - Reset Timer/Counter Module 0"]
pub type TIMER0_R = crate::BitReader<bool>;
#[doc = "Field `timer0` writer - Reset Timer/Counter Module 0"]
pub type TIMER0_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 8>;
#[doc = "Field `timer1` reader - Reset Timer/Counter Module 1"]
pub type TIMER1_R = crate::BitReader<bool>;
#[doc = "Field `timer1` writer - Reset Timer/Counter Module 1"]
pub type TIMER1_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 9>;
#[doc = "Field `timer2` reader - Reset Timer/Counter Module 2"]
pub type TIMER2_R = crate::BitReader<bool>;
#[doc = "Field `timer2` writer - Reset Timer/Counter Module 2"]
pub type TIMER2_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 10>;
#[doc = "Field `timer3` reader - Reset Timer/Counter Module 3"]
pub type TIMER3_R = crate::BitReader<bool>;
#[doc = "Field `timer3` writer - Reset Timer/Counter Module 3"]
pub type TIMER3_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 11>;
#[doc = "Field `timer4` reader - Reset Timer/Counter Module 4"]
pub type TIMER4_R = crate::BitReader<bool>;
#[doc = "Field `timer4` writer - Reset Timer/Counter Module 4"]
pub type TIMER4_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 12>;
#[doc = "Field `timer5` reader - Reset Timer/Counter Module 5"]
pub type TIMER5_R = crate::BitReader<bool>;
#[doc = "Field `timer5` writer - Reset Timer/Counter Module 5"]
pub type TIMER5_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 13>;
#[doc = "Field `pulse_train` reader - Reset All Pulse Trains"]
pub type PULSE_TRAIN_R = crate::BitReader<bool>;
#[doc = "Field `pulse_train` writer - Reset All Pulse Trains"]
pub type PULSE_TRAIN_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 14>;
#[doc = "Field `uart0` reader - Reset UART 0"]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `uart0` writer - Reset UART 0"]
pub type UART0_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 15>;
#[doc = "Field `uart1` reader - Reset UART 1"]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `uart1` writer - Reset UART 1"]
pub type UART1_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 16>;
#[doc = "Field `uart2` reader - Reset UART 2"]
pub type UART2_R = crate::BitReader<bool>;
#[doc = "Field `uart2` writer - Reset UART 2"]
pub type UART2_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 17>;
#[doc = "Field `uart3` reader - Reset UART 3"]
pub type UART3_R = crate::BitReader<bool>;
#[doc = "Field `uart3` writer - Reset UART 3"]
pub type UART3_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 18>;
#[doc = "Field `i2cm0` reader - Reset I2C Master 0"]
pub type I2CM0_R = crate::BitReader<bool>;
#[doc = "Field `i2cm0` writer - Reset I2C Master 0"]
pub type I2CM0_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 19>;
#[doc = "Field `i2cm1` reader - Reset I2C Master 1"]
pub type I2CM1_R = crate::BitReader<bool>;
#[doc = "Field `i2cm1` writer - Reset I2C Master 1"]
pub type I2CM1_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 20>;
#[doc = "Field `i2cm2` reader - Reset I2C Master 2"]
pub type I2CM2_R = crate::BitReader<bool>;
#[doc = "Field `i2cm2` writer - Reset I2C Master 2"]
pub type I2CM2_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 21>;
#[doc = "Field `i2cs` reader - Reset I2C Slave"]
pub type I2CS_R = crate::BitReader<bool>;
#[doc = "Field `i2cs` writer - Reset I2C Slave"]
pub type I2CS_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 22>;
#[doc = "Field `spim0` reader - Reset SPI Master 0"]
pub type SPIM0_R = crate::BitReader<bool>;
#[doc = "Field `spim0` writer - Reset SPI Master 0"]
pub type SPIM0_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 23>;
#[doc = "Field `spim1` reader - Reset SPI Master 1"]
pub type SPIM1_R = crate::BitReader<bool>;
#[doc = "Field `spim1` writer - Reset SPI Master 1"]
pub type SPIM1_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 24>;
#[doc = "Field `spim2` reader - Reset SPI Master 2"]
pub type SPIM2_R = crate::BitReader<bool>;
#[doc = "Field `spim2` writer - Reset SPI Master 2"]
pub type SPIM2_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 25>;
#[doc = "Field `spib` reader - Reset SPI Bridge"]
pub type SPIB_R = crate::BitReader<bool>;
#[doc = "Field `spib` writer - Reset SPI Bridge"]
pub type SPIB_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 26>;
#[doc = "Field `owm` reader - Reset 1-Wire Master"]
pub type OWM_R = crate::BitReader<bool>;
#[doc = "Field `owm` writer - Reset 1-Wire Master"]
pub type OWM_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 27>;
#[doc = "Field `adc` reader - Reset ADC"]
pub type ADC_R = crate::BitReader<bool>;
#[doc = "Field `adc` writer - Reset ADC"]
pub type ADC_W<'a> = crate::BitWriter<'a, u32, PERIPHERAL_RESET_SPEC, bool, 28>;
impl R {
    #[doc = "Bit 0 - Reset SSB"]
    #[inline(always)]
    pub fn ssb(&self) -> SSB_R {
        SSB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset SPI XIP"]
    #[inline(always)]
    pub fn spix(&self) -> SPIX_R {
        SPIX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset PMU"]
    #[inline(always)]
    pub fn pmu(&self) -> PMU_R {
        PMU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset USB"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset TPU"]
    #[inline(always)]
    pub fn tpu(&self) -> TPU_R {
        TPU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset Watchdog Timer 0"]
    #[inline(always)]
    pub fn watchdog0(&self) -> WATCHDOG0_R {
        WATCHDOG0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset GPIO"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset Timer/Counter Module 0"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset Timer/Counter Module 1"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset Timer/Counter Module 2"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset Timer/Counter Module 3"]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset Timer/Counter Module 4"]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset Timer/Counter Module 5"]
    #[inline(always)]
    pub fn timer5(&self) -> TIMER5_R {
        TIMER5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset All Pulse Trains"]
    #[inline(always)]
    pub fn pulse_train(&self) -> PULSE_TRAIN_R {
        PULSE_TRAIN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset UART 0"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset UART 1"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset UART 2"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset UART 3"]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset I2C Master 0"]
    #[inline(always)]
    pub fn i2cm0(&self) -> I2CM0_R {
        I2CM0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset I2C Master 1"]
    #[inline(always)]
    pub fn i2cm1(&self) -> I2CM1_R {
        I2CM1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset I2C Master 2"]
    #[inline(always)]
    pub fn i2cm2(&self) -> I2CM2_R {
        I2CM2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset I2C Slave"]
    #[inline(always)]
    pub fn i2cs(&self) -> I2CS_R {
        I2CS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset SPI Master 0"]
    #[inline(always)]
    pub fn spim0(&self) -> SPIM0_R {
        SPIM0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset SPI Master 1"]
    #[inline(always)]
    pub fn spim1(&self) -> SPIM1_R {
        SPIM1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset SPI Master 2"]
    #[inline(always)]
    pub fn spim2(&self) -> SPIM2_R {
        SPIM2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset SPI Bridge"]
    #[inline(always)]
    pub fn spib(&self) -> SPIB_R {
        SPIB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reset 1-Wire Master"]
    #[inline(always)]
    pub fn owm(&self) -> OWM_R {
        OWM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset SSB"]
    #[inline(always)]
    pub fn ssb(&mut self) -> SSB_W {
        SSB_W::new(self)
    }
    #[doc = "Bit 1 - Reset SPI XIP"]
    #[inline(always)]
    pub fn spix(&mut self) -> SPIX_W {
        SPIX_W::new(self)
    }
    #[doc = "Bit 2 - Reset PMU"]
    #[inline(always)]
    pub fn pmu(&mut self) -> PMU_W {
        PMU_W::new(self)
    }
    #[doc = "Bit 3 - Reset USB"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W::new(self)
    }
    #[doc = "Bit 4 - Reset CRC"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W::new(self)
    }
    #[doc = "Bit 5 - Reset TPU"]
    #[inline(always)]
    pub fn tpu(&mut self) -> TPU_W {
        TPU_W::new(self)
    }
    #[doc = "Bit 6 - Reset Watchdog Timer 0"]
    #[inline(always)]
    pub fn watchdog0(&mut self) -> WATCHDOG0_W {
        WATCHDOG0_W::new(self)
    }
    #[doc = "Bit 7 - Reset GPIO"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W::new(self)
    }
    #[doc = "Bit 8 - Reset Timer/Counter Module 0"]
    #[inline(always)]
    pub fn timer0(&mut self) -> TIMER0_W {
        TIMER0_W::new(self)
    }
    #[doc = "Bit 9 - Reset Timer/Counter Module 1"]
    #[inline(always)]
    pub fn timer1(&mut self) -> TIMER1_W {
        TIMER1_W::new(self)
    }
    #[doc = "Bit 10 - Reset Timer/Counter Module 2"]
    #[inline(always)]
    pub fn timer2(&mut self) -> TIMER2_W {
        TIMER2_W::new(self)
    }
    #[doc = "Bit 11 - Reset Timer/Counter Module 3"]
    #[inline(always)]
    pub fn timer3(&mut self) -> TIMER3_W {
        TIMER3_W::new(self)
    }
    #[doc = "Bit 12 - Reset Timer/Counter Module 4"]
    #[inline(always)]
    pub fn timer4(&mut self) -> TIMER4_W {
        TIMER4_W::new(self)
    }
    #[doc = "Bit 13 - Reset Timer/Counter Module 5"]
    #[inline(always)]
    pub fn timer5(&mut self) -> TIMER5_W {
        TIMER5_W::new(self)
    }
    #[doc = "Bit 14 - Reset All Pulse Trains"]
    #[inline(always)]
    pub fn pulse_train(&mut self) -> PULSE_TRAIN_W {
        PULSE_TRAIN_W::new(self)
    }
    #[doc = "Bit 15 - Reset UART 0"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W::new(self)
    }
    #[doc = "Bit 16 - Reset UART 1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W::new(self)
    }
    #[doc = "Bit 17 - Reset UART 2"]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W {
        UART2_W::new(self)
    }
    #[doc = "Bit 18 - Reset UART 3"]
    #[inline(always)]
    pub fn uart3(&mut self) -> UART3_W {
        UART3_W::new(self)
    }
    #[doc = "Bit 19 - Reset I2C Master 0"]
    #[inline(always)]
    pub fn i2cm0(&mut self) -> I2CM0_W {
        I2CM0_W::new(self)
    }
    #[doc = "Bit 20 - Reset I2C Master 1"]
    #[inline(always)]
    pub fn i2cm1(&mut self) -> I2CM1_W {
        I2CM1_W::new(self)
    }
    #[doc = "Bit 21 - Reset I2C Master 2"]
    #[inline(always)]
    pub fn i2cm2(&mut self) -> I2CM2_W {
        I2CM2_W::new(self)
    }
    #[doc = "Bit 22 - Reset I2C Slave"]
    #[inline(always)]
    pub fn i2cs(&mut self) -> I2CS_W {
        I2CS_W::new(self)
    }
    #[doc = "Bit 23 - Reset SPI Master 0"]
    #[inline(always)]
    pub fn spim0(&mut self) -> SPIM0_W {
        SPIM0_W::new(self)
    }
    #[doc = "Bit 24 - Reset SPI Master 1"]
    #[inline(always)]
    pub fn spim1(&mut self) -> SPIM1_W {
        SPIM1_W::new(self)
    }
    #[doc = "Bit 25 - Reset SPI Master 2"]
    #[inline(always)]
    pub fn spim2(&mut self) -> SPIM2_W {
        SPIM2_W::new(self)
    }
    #[doc = "Bit 26 - Reset SPI Bridge"]
    #[inline(always)]
    pub fn spib(&mut self) -> SPIB_W {
        SPIB_W::new(self)
    }
    #[doc = "Bit 27 - Reset 1-Wire Master"]
    #[inline(always)]
    pub fn owm(&mut self) -> OWM_W {
        OWM_W::new(self)
    }
    #[doc = "Bit 28 - Reset ADC"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peripheral_reset](index.html) module"]
pub struct PERIPHERAL_RESET_SPEC;
impl crate::RegisterSpec for PERIPHERAL_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peripheral_reset::R](R) reader structure"]
impl crate::Readable for PERIPHERAL_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peripheral_reset::W](W) writer structure"]
impl crate::Writable for PERIPHERAL_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIPHERAL_RESET to value 0"]
impl crate::Resettable for PERIPHERAL_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
