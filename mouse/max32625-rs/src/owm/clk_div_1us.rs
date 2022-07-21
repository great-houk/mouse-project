#[doc = "Register `CLK_DIV_1US` reader"]
pub struct R(crate::R<CLK_DIV_1US_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_DIV_1US_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_DIV_1US_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_DIV_1US_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_DIV_1US` writer"]
pub struct W(crate::W<CLK_DIV_1US_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_DIV_1US_SPEC>;
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
impl From<crate::W<CLK_DIV_1US_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_DIV_1US_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `divisor` reader - Clock Divisor for 1MHz"]
pub type DIVISOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `divisor` writer - Clock Divisor for 1MHz"]
pub type DIVISOR_W<'a> = crate::FieldWriter<'a, u32, CLK_DIV_1US_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - Clock Divisor for 1MHz"]
    #[inline(always)]
    pub fn divisor(&self) -> DIVISOR_R {
        DIVISOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Divisor for 1MHz"]
    #[inline(always)]
    pub fn divisor(&mut self) -> DIVISOR_W {
        DIVISOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1-Wire Master Clock Divisor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_div_1us](index.html) module"]
pub struct CLK_DIV_1US_SPEC;
impl crate::RegisterSpec for CLK_DIV_1US_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_div_1us::R](R) reader structure"]
impl crate::Readable for CLK_DIV_1US_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_div_1us::W](W) writer structure"]
impl crate::Writable for CLK_DIV_1US_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_DIV_1US to value 0"]
impl crate::Resettable for CLK_DIV_1US_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
