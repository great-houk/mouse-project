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
#[doc = "Field `nano_en` reader - Enable nanoring oscillator output"]
pub type NANO_EN_R = crate::BitReader<bool>;
#[doc = "Field `nano_en` writer - Enable nanoring oscillator output"]
pub type NANO_EN_W<'a> = crate::BitWriter<'a, u32, CLK_CTRL_SPEC, bool, 2>;
impl R {
    #[doc = "Bit 2 - Enable nanoring oscillator output"]
    #[inline(always)]
    pub fn nano_en(&self) -> NANO_EN_R {
        NANO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enable nanoring oscillator output"]
    #[inline(always)]
    pub fn nano_en(&mut self) -> NANO_EN_W {
        NANO_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Clock Control Settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl](index.html) module"]
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
