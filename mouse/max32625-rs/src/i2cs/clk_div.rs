#[doc = "Register `CLK_DIV` reader"]
pub struct R(crate::R<CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_DIV` writer"]
pub struct W(crate::W<CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_DIV_SPEC>;
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
impl From<crate::W<CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fs_filter_clock_div` reader - FS Filter Clock Divisor"]
pub type FS_FILTER_CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fs_filter_clock_div` writer - FS Filter Clock Divisor"]
pub type FS_FILTER_CLOCK_DIV_W<'a> = crate::FieldWriter<'a, u32, CLK_DIV_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - FS Filter Clock Divisor"]
    #[inline(always)]
    pub fn fs_filter_clock_div(&self) -> FS_FILTER_CLOCK_DIV_R {
        FS_FILTER_CLOCK_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FS Filter Clock Divisor"]
    #[inline(always)]
    pub fn fs_filter_clock_div(&mut self) -> FS_FILTER_CLOCK_DIV_W {
        FS_FILTER_CLOCK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Clock Divisor Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_div](index.html) module"]
pub struct CLK_DIV_SPEC;
impl crate::RegisterSpec for CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_div::R](R) reader structure"]
impl crate::Readable for CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_div::W](W) writer structure"]
impl crate::Writable for CLK_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_DIV to value 0"]
impl crate::Resettable for CLK_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
