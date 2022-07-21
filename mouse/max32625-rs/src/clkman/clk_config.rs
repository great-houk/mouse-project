#[doc = "Register `CLK_CONFIG` reader"]
pub struct R(crate::R<CLK_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONFIG` writer"]
pub struct W(crate::W<CLK_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONFIG_SPEC>;
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
impl From<crate::W<CLK_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `crypto_enable` reader - Cryptographic (TPU) Relaxation Oscillator Enable"]
pub type CRYPTO_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `crypto_enable` writer - Cryptographic (TPU) Relaxation Oscillator Enable"]
pub type CRYPTO_ENABLE_W<'a> = crate::BitWriter<'a, u32, CLK_CONFIG_SPEC, bool, 0>;
#[doc = "Field `crypto_stability_count` reader - Crypto Oscillator Stability Select"]
pub type CRYPTO_STABILITY_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `crypto_stability_count` writer - Crypto Oscillator Stability Select"]
pub type CRYPTO_STABILITY_COUNT_W<'a> = crate::FieldWriter<'a, u32, CLK_CONFIG_SPEC, u8, u8, 4, 4>;
impl R {
    #[doc = "Bit 0 - Cryptographic (TPU) Relaxation Oscillator Enable"]
    #[inline(always)]
    pub fn crypto_enable(&self) -> CRYPTO_ENABLE_R {
        CRYPTO_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Crypto Oscillator Stability Select"]
    #[inline(always)]
    pub fn crypto_stability_count(&self) -> CRYPTO_STABILITY_COUNT_R {
        CRYPTO_STABILITY_COUNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cryptographic (TPU) Relaxation Oscillator Enable"]
    #[inline(always)]
    pub fn crypto_enable(&mut self) -> CRYPTO_ENABLE_W {
        CRYPTO_ENABLE_W::new(self)
    }
    #[doc = "Bits 4:7 - Crypto Oscillator Stability Select"]
    #[inline(always)]
    pub fn crypto_stability_count(&mut self) -> CRYPTO_STABILITY_COUNT_W {
        CRYPTO_STABILITY_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_config](index.html) module"]
pub struct CLK_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_config::R](R) reader structure"]
impl crate::Readable for CLK_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_config::W](W) writer structure"]
impl crate::Writable for CLK_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CONFIG to value 0"]
impl crate::Resettable for CLK_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
