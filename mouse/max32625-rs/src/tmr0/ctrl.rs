#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mode` reader - Operating Modes for 32-bit/16-bit Timers"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mode` writer - Operating Modes for 32-bit/16-bit Timers"]
pub type MODE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, 0>;
#[doc = "Field `tmr2x16` reader - Dual 16-bit Timer Mode"]
pub type TMR2X16_R = crate::BitReader<bool>;
#[doc = "Field `tmr2x16` writer - Dual 16-bit Timer Mode"]
pub type TMR2X16_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Field `prescale` reader - Timer Clock Prescale Setting"]
pub type PRESCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `prescale` writer - Timer Clock Prescale Setting"]
pub type PRESCALE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 4>;
#[doc = "Field `polarity` reader - Timer I/O Polarity"]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `polarity` writer - Timer I/O Polarity"]
pub type POLARITY_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 8>;
#[doc = "Field `enable0` reader - Enable 32-bit timer / 16-bit timer 0"]
pub type ENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `enable0` writer - Enable 32-bit timer / 16-bit timer 0"]
pub type ENABLE0_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 12>;
#[doc = "Field `enable1` reader - Enable 16-bit timer 1"]
pub type ENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `enable1` writer - Enable 16-bit timer 1"]
pub type ENABLE1_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 13>;
impl R {
    #[doc = "Bits 0:2 - Operating Modes for 32-bit/16-bit Timers"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Dual 16-bit Timer Mode"]
    #[inline(always)]
    pub fn tmr2x16(&self) -> TMR2X16_R {
        TMR2X16_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Timer Clock Prescale Setting"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Timer I/O Polarity"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable 32-bit timer / 16-bit timer 0"]
    #[inline(always)]
    pub fn enable0(&self) -> ENABLE0_R {
        ENABLE0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable 16-bit timer 1"]
    #[inline(always)]
    pub fn enable1(&self) -> ENABLE1_R {
        ENABLE1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Operating Modes for 32-bit/16-bit Timers"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W::new(self)
    }
    #[doc = "Bit 3 - Dual 16-bit Timer Mode"]
    #[inline(always)]
    pub fn tmr2x16(&mut self) -> TMR2X16_W {
        TMR2X16_W::new(self)
    }
    #[doc = "Bits 4:7 - Timer Clock Prescale Setting"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W::new(self)
    }
    #[doc = "Bit 8 - Timer I/O Polarity"]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W {
        POLARITY_W::new(self)
    }
    #[doc = "Bit 12 - Enable 32-bit timer / 16-bit timer 0"]
    #[inline(always)]
    pub fn enable0(&mut self) -> ENABLE0_W {
        ENABLE0_W::new(self)
    }
    #[doc = "Bit 13 - Enable 16-bit timer 1"]
    #[inline(always)]
    pub fn enable1(&mut self) -> ENABLE1_W {
        ENABLE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
