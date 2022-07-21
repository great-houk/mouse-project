#[doc = "Register `CLK_GATE_CTRL1` reader"]
pub struct R(crate::R<CLK_GATE_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_GATE_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_GATE_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_GATE_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_GATE_CTRL1` writer"]
pub struct W(crate::W<CLK_GATE_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_GATE_CTRL1_SPEC>;
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
impl From<crate::W<CLK_GATE_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_GATE_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `watchdog1_clk_gater` reader - Clock Gating Control for Watchdog Timer 1"]
pub type WATCHDOG1_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `watchdog1_clk_gater` writer - Clock Gating Control for Watchdog Timer 1"]
pub type WATCHDOG1_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 0>;
#[doc = "Field `gpio_clk_gater` reader - Clock Gating Control for GPIO Ports"]
pub type GPIO_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gpio_clk_gater` writer - Clock Gating Control for GPIO Ports"]
pub type GPIO_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 2>;
#[doc = "Field `timer0_clk_gater` reader - Clock Gating Control for Timer/Counter Module 0"]
pub type TIMER0_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `timer0_clk_gater` writer - Clock Gating Control for Timer/Counter Module 0"]
pub type TIMER0_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 4>;
#[doc = "Field `timer1_clk_gater` reader - Clock Gating Control for Timer/Counter Module 1"]
pub type TIMER1_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `timer1_clk_gater` writer - Clock Gating Control for Timer/Counter Module 1"]
pub type TIMER1_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 6>;
#[doc = "Field `timer2_clk_gater` reader - Clock Gating Control for Timer/Counter Module 2"]
pub type TIMER2_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `timer2_clk_gater` writer - Clock Gating Control for Timer/Counter Module 2"]
pub type TIMER2_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 8>;
#[doc = "Field `timer3_clk_gater` reader - Clock Gating Control for Timer/Counter Module 3"]
pub type TIMER3_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `timer3_clk_gater` writer - Clock Gating Control for Timer/Counter Module 3"]
pub type TIMER3_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 10>;
#[doc = "Field `timer4_clk_gater` reader - Clock Gating Control for Timer/Counter Module 4"]
pub type TIMER4_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `timer4_clk_gater` writer - Clock Gating Control for Timer/Counter Module 4"]
pub type TIMER4_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 12>;
#[doc = "Field `timer5_clk_gater` reader - Clock Gating Control for Timer/Counter Module 5"]
pub type TIMER5_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `timer5_clk_gater` writer - Clock Gating Control for Timer/Counter Module 5"]
pub type TIMER5_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 14>;
#[doc = "Field `pulsetrain_clk_gater` reader - Clock Gating Control for Pulse Train Generators"]
pub type PULSETRAIN_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pulsetrain_clk_gater` writer - Clock Gating Control for Pulse Train Generators"]
pub type PULSETRAIN_CLK_GATER_W<'a> =
    crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 16>;
#[doc = "Field `uart0_clk_gater` reader - Clock Gating Control for UART 0"]
pub type UART0_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart0_clk_gater` writer - Clock Gating Control for UART 0"]
pub type UART0_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 18>;
#[doc = "Field `uart1_clk_gater` reader - Clock Gating Control for UART 1"]
pub type UART1_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart1_clk_gater` writer - Clock Gating Control for UART 1"]
pub type UART1_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 20>;
#[doc = "Field `uart2_clk_gater` reader - Clock Gating Control for UART 2"]
pub type UART2_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart2_clk_gater` writer - Clock Gating Control for UART 2"]
pub type UART2_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 22>;
#[doc = "Field `uart3_clk_gater` reader - Clock Gating Control for UART 3"]
pub type UART3_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uart3_clk_gater` writer - Clock Gating Control for UART 3"]
pub type UART3_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 24>;
#[doc = "Field `i2cm0_clk_gater` reader - Clock Gating Control for I2C Master 0"]
pub type I2CM0_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `i2cm0_clk_gater` writer - Clock Gating Control for I2C Master 0"]
pub type I2CM0_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 26>;
#[doc = "Field `i2cm1_clk_gater` reader - Clock Gating Control for I2C Master 1"]
pub type I2CM1_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `i2cm1_clk_gater` writer - Clock Gating Control for I2C Master 1"]
pub type I2CM1_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 28>;
#[doc = "Field `i2cm2_clk_gater` reader - Clock Gating Control for I2C Master 2"]
pub type I2CM2_CLK_GATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `i2cm2_clk_gater` writer - Clock Gating Control for I2C Master 2"]
pub type I2CM2_CLK_GATER_W<'a> = crate::FieldWriter<'a, u32, CLK_GATE_CTRL1_SPEC, u8, u8, 2, 30>;
impl R {
    #[doc = "Bits 0:1 - Clock Gating Control for Watchdog Timer 1"]
    #[inline(always)]
    pub fn watchdog1_clk_gater(&self) -> WATCHDOG1_CLK_GATER_R {
        WATCHDOG1_CLK_GATER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Clock Gating Control for GPIO Ports"]
    #[inline(always)]
    pub fn gpio_clk_gater(&self) -> GPIO_CLK_GATER_R {
        GPIO_CLK_GATER_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Clock Gating Control for Timer/Counter Module 0"]
    #[inline(always)]
    pub fn timer0_clk_gater(&self) -> TIMER0_CLK_GATER_R {
        TIMER0_CLK_GATER_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Clock Gating Control for Timer/Counter Module 1"]
    #[inline(always)]
    pub fn timer1_clk_gater(&self) -> TIMER1_CLK_GATER_R {
        TIMER1_CLK_GATER_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Clock Gating Control for Timer/Counter Module 2"]
    #[inline(always)]
    pub fn timer2_clk_gater(&self) -> TIMER2_CLK_GATER_R {
        TIMER2_CLK_GATER_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Clock Gating Control for Timer/Counter Module 3"]
    #[inline(always)]
    pub fn timer3_clk_gater(&self) -> TIMER3_CLK_GATER_R {
        TIMER3_CLK_GATER_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Clock Gating Control for Timer/Counter Module 4"]
    #[inline(always)]
    pub fn timer4_clk_gater(&self) -> TIMER4_CLK_GATER_R {
        TIMER4_CLK_GATER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Clock Gating Control for Timer/Counter Module 5"]
    #[inline(always)]
    pub fn timer5_clk_gater(&self) -> TIMER5_CLK_GATER_R {
        TIMER5_CLK_GATER_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Clock Gating Control for Pulse Train Generators"]
    #[inline(always)]
    pub fn pulsetrain_clk_gater(&self) -> PULSETRAIN_CLK_GATER_R {
        PULSETRAIN_CLK_GATER_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Clock Gating Control for UART 0"]
    #[inline(always)]
    pub fn uart0_clk_gater(&self) -> UART0_CLK_GATER_R {
        UART0_CLK_GATER_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Clock Gating Control for UART 1"]
    #[inline(always)]
    pub fn uart1_clk_gater(&self) -> UART1_CLK_GATER_R {
        UART1_CLK_GATER_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Clock Gating Control for UART 2"]
    #[inline(always)]
    pub fn uart2_clk_gater(&self) -> UART2_CLK_GATER_R {
        UART2_CLK_GATER_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Clock Gating Control for UART 3"]
    #[inline(always)]
    pub fn uart3_clk_gater(&self) -> UART3_CLK_GATER_R {
        UART3_CLK_GATER_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Clock Gating Control for I2C Master 0"]
    #[inline(always)]
    pub fn i2cm0_clk_gater(&self) -> I2CM0_CLK_GATER_R {
        I2CM0_CLK_GATER_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Clock Gating Control for I2C Master 1"]
    #[inline(always)]
    pub fn i2cm1_clk_gater(&self) -> I2CM1_CLK_GATER_R {
        I2CM1_CLK_GATER_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Clock Gating Control for I2C Master 2"]
    #[inline(always)]
    pub fn i2cm2_clk_gater(&self) -> I2CM2_CLK_GATER_R {
        I2CM2_CLK_GATER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Gating Control for Watchdog Timer 1"]
    #[inline(always)]
    pub fn watchdog1_clk_gater(&mut self) -> WATCHDOG1_CLK_GATER_W {
        WATCHDOG1_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 2:3 - Clock Gating Control for GPIO Ports"]
    #[inline(always)]
    pub fn gpio_clk_gater(&mut self) -> GPIO_CLK_GATER_W {
        GPIO_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 4:5 - Clock Gating Control for Timer/Counter Module 0"]
    #[inline(always)]
    pub fn timer0_clk_gater(&mut self) -> TIMER0_CLK_GATER_W {
        TIMER0_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 6:7 - Clock Gating Control for Timer/Counter Module 1"]
    #[inline(always)]
    pub fn timer1_clk_gater(&mut self) -> TIMER1_CLK_GATER_W {
        TIMER1_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 8:9 - Clock Gating Control for Timer/Counter Module 2"]
    #[inline(always)]
    pub fn timer2_clk_gater(&mut self) -> TIMER2_CLK_GATER_W {
        TIMER2_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 10:11 - Clock Gating Control for Timer/Counter Module 3"]
    #[inline(always)]
    pub fn timer3_clk_gater(&mut self) -> TIMER3_CLK_GATER_W {
        TIMER3_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 12:13 - Clock Gating Control for Timer/Counter Module 4"]
    #[inline(always)]
    pub fn timer4_clk_gater(&mut self) -> TIMER4_CLK_GATER_W {
        TIMER4_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 14:15 - Clock Gating Control for Timer/Counter Module 5"]
    #[inline(always)]
    pub fn timer5_clk_gater(&mut self) -> TIMER5_CLK_GATER_W {
        TIMER5_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 16:17 - Clock Gating Control for Pulse Train Generators"]
    #[inline(always)]
    pub fn pulsetrain_clk_gater(&mut self) -> PULSETRAIN_CLK_GATER_W {
        PULSETRAIN_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 18:19 - Clock Gating Control for UART 0"]
    #[inline(always)]
    pub fn uart0_clk_gater(&mut self) -> UART0_CLK_GATER_W {
        UART0_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 20:21 - Clock Gating Control for UART 1"]
    #[inline(always)]
    pub fn uart1_clk_gater(&mut self) -> UART1_CLK_GATER_W {
        UART1_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 22:23 - Clock Gating Control for UART 2"]
    #[inline(always)]
    pub fn uart2_clk_gater(&mut self) -> UART2_CLK_GATER_W {
        UART2_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 24:25 - Clock Gating Control for UART 3"]
    #[inline(always)]
    pub fn uart3_clk_gater(&mut self) -> UART3_CLK_GATER_W {
        UART3_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 26:27 - Clock Gating Control for I2C Master 0"]
    #[inline(always)]
    pub fn i2cm0_clk_gater(&mut self) -> I2CM0_CLK_GATER_W {
        I2CM0_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 28:29 - Clock Gating Control for I2C Master 1"]
    #[inline(always)]
    pub fn i2cm1_clk_gater(&mut self) -> I2CM1_CLK_GATER_W {
        I2CM1_CLK_GATER_W::new(self)
    }
    #[doc = "Bits 30:31 - Clock Gating Control for I2C Master 2"]
    #[inline(always)]
    pub fn i2cm2_clk_gater(&mut self) -> I2CM2_CLK_GATER_W {
        I2CM2_CLK_GATER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic Clock Gating Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gate_ctrl1](index.html) module"]
pub struct CLK_GATE_CTRL1_SPEC;
impl crate::RegisterSpec for CLK_GATE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_gate_ctrl1::R](R) reader structure"]
impl crate::Readable for CLK_GATE_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_gate_ctrl1::W](W) writer structure"]
impl crate::Writable for CLK_GATE_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_GATE_CTRL1 to value 0"]
impl crate::Resettable for CLK_GATE_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
