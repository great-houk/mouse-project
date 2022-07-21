#[doc = "Register `I2C_TIMER_CTRL` reader"]
pub struct R(crate::R<I2C_TIMER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TIMER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TIMER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TIMER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TIMER_CTRL` writer"]
pub struct W(crate::W<I2C_TIMER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TIMER_CTRL_SPEC>;
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
impl From<crate::W<I2C_TIMER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TIMER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `i2c_1ms_timer_en` reader - I2C 1ms Timer Enable"]
pub type I2C_1MS_TIMER_EN_R = crate::BitReader<bool>;
#[doc = "Field `i2c_1ms_timer_en` writer - I2C 1ms Timer Enable"]
pub type I2C_1MS_TIMER_EN_W<'a> = crate::BitWriter<'a, u32, I2C_TIMER_CTRL_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - I2C 1ms Timer Enable"]
    #[inline(always)]
    pub fn i2c_1ms_timer_en(&self) -> I2C_1MS_TIMER_EN_R {
        I2C_1MS_TIMER_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C 1ms Timer Enable"]
    #[inline(always)]
    pub fn i2c_1ms_timer_en(&mut self) -> I2C_1MS_TIMER_EN_W {
        I2C_1MS_TIMER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_timer_ctrl](index.html) module"]
pub struct I2C_TIMER_CTRL_SPEC;
impl crate::RegisterSpec for I2C_TIMER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_timer_ctrl::R](R) reader structure"]
impl crate::Readable for I2C_TIMER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_timer_ctrl::W](W) writer structure"]
impl crate::Writable for I2C_TIMER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2C_TIMER_CTRL to value 0"]
impl crate::Resettable for I2C_TIMER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
