#[doc = "Register `SYS_CLK_CTRL_14_SPIB` reader"]
pub struct R(crate::R<SYS_CLK_CTRL_14_SPIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CLK_CTRL_14_SPIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CLK_CTRL_14_SPIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CLK_CTRL_14_SPIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CLK_CTRL_14_SPIB` writer"]
pub struct W(crate::W<SYS_CLK_CTRL_14_SPIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CLK_CTRL_14_SPIB_SPEC>;
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
impl From<crate::W<SYS_CLK_CTRL_14_SPIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CLK_CTRL_14_SPIB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spib_clk_scale` reader - Control Settings for CLK14 - SPI Bridge Clock"]
pub type SPIB_CLK_SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spib_clk_scale` writer - Control Settings for CLK14 - SPI Bridge Clock"]
pub type SPIB_CLK_SCALE_W<'a> =
    crate::FieldWriter<'a, u32, SYS_CLK_CTRL_14_SPIB_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - Control Settings for CLK14 - SPI Bridge Clock"]
    #[inline(always)]
    pub fn spib_clk_scale(&self) -> SPIB_CLK_SCALE_R {
        SPIB_CLK_SCALE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Control Settings for CLK14 - SPI Bridge Clock"]
    #[inline(always)]
    pub fn spib_clk_scale(&mut self) -> SPIB_CLK_SCALE_W {
        SPIB_CLK_SCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Settings for CLK14 - SPI Bridge Clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_clk_ctrl_14_spib](index.html) module"]
pub struct SYS_CLK_CTRL_14_SPIB_SPEC;
impl crate::RegisterSpec for SYS_CLK_CTRL_14_SPIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_clk_ctrl_14_spib::R](R) reader structure"]
impl crate::Readable for SYS_CLK_CTRL_14_SPIB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_clk_ctrl_14_spib::W](W) writer structure"]
impl crate::Writable for SYS_CLK_CTRL_14_SPIB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_CLK_CTRL_14_SPIB to value 0"]
impl crate::Resettable for SYS_CLK_CTRL_14_SPIB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
