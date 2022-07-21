#[doc = "Register `CRYPT_CLK_CTRL_0_AES` reader"]
pub struct R(crate::R<CRYPT_CLK_CTRL_0_AES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPT_CLK_CTRL_0_AES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPT_CLK_CTRL_0_AES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPT_CLK_CTRL_0_AES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPT_CLK_CTRL_0_AES` writer"]
pub struct W(crate::W<CRYPT_CLK_CTRL_0_AES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPT_CLK_CTRL_0_AES_SPEC>;
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
impl From<crate::W<CRYPT_CLK_CTRL_0_AES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPT_CLK_CTRL_0_AES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `aes_clk_scale` reader - Control Settings for Crypto Clock 0 - AES"]
pub type AES_CLK_SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `aes_clk_scale` writer - Control Settings for Crypto Clock 0 - AES"]
pub type AES_CLK_SCALE_W<'a> = crate::FieldWriter<'a, u32, CRYPT_CLK_CTRL_0_AES_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - Control Settings for Crypto Clock 0 - AES"]
    #[inline(always)]
    pub fn aes_clk_scale(&self) -> AES_CLK_SCALE_R {
        AES_CLK_SCALE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Control Settings for Crypto Clock 0 - AES"]
    #[inline(always)]
    pub fn aes_clk_scale(&mut self) -> AES_CLK_SCALE_W {
        AES_CLK_SCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Settings for Crypto Clock 0 - AES\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypt_clk_ctrl_0_aes](index.html) module"]
pub struct CRYPT_CLK_CTRL_0_AES_SPEC;
impl crate::RegisterSpec for CRYPT_CLK_CTRL_0_AES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypt_clk_ctrl_0_aes::R](R) reader structure"]
impl crate::Readable for CRYPT_CLK_CTRL_0_AES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypt_clk_ctrl_0_aes::W](W) writer structure"]
impl crate::Writable for CRYPT_CLK_CTRL_0_AES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPT_CLK_CTRL_0_AES to value 0"]
impl crate::Resettable for CRYPT_CLK_CTRL_0_AES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
