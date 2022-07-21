#[doc = "Register `TRIM_CALC` reader"]
pub struct R(crate::R<TRIM_CALC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_CALC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_CALC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_CALC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_CALC` writer"]
pub struct W(crate::W<TRIM_CALC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_CALC_SPEC>;
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
impl From<crate::W<TRIM_CALC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_CALC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `trim_clk_sel` reader - Trim Clock Select"]
pub type TRIM_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `trim_clk_sel` writer - Trim Clock Select"]
pub type TRIM_CLK_SEL_W<'a> = crate::BitWriter<'a, u32, TRIM_CALC_SPEC, bool, 0>;
#[doc = "Field `trim_calc_start` reader - Start Trim Calculation"]
pub type TRIM_CALC_START_R = crate::BitReader<bool>;
#[doc = "Field `trim_calc_start` writer - Start Trim Calculation"]
pub type TRIM_CALC_START_W<'a> = crate::BitWriter<'a, u32, TRIM_CALC_SPEC, bool, 1>;
#[doc = "Field `trim_calc_completed` reader - Trim Calculation Completed"]
pub type TRIM_CALC_COMPLETED_R = crate::BitReader<bool>;
#[doc = "Field `trim_enable` reader - Trim Logic Enable"]
pub type TRIM_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `trim_enable` writer - Trim Logic Enable"]
pub type TRIM_ENABLE_W<'a> = crate::BitWriter<'a, u32, TRIM_CALC_SPEC, bool, 3>;
#[doc = "Field `trim_calc_results` reader - Trim Calculation Results"]
pub type TRIM_CALC_RESULTS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Trim Clock Select"]
    #[inline(always)]
    pub fn trim_clk_sel(&self) -> TRIM_CLK_SEL_R {
        TRIM_CLK_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Trim Calculation"]
    #[inline(always)]
    pub fn trim_calc_start(&self) -> TRIM_CALC_START_R {
        TRIM_CALC_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trim Calculation Completed"]
    #[inline(always)]
    pub fn trim_calc_completed(&self) -> TRIM_CALC_COMPLETED_R {
        TRIM_CALC_COMPLETED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trim Logic Enable"]
    #[inline(always)]
    pub fn trim_enable(&self) -> TRIM_ENABLE_R {
        TRIM_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Trim Calculation Results"]
    #[inline(always)]
    pub fn trim_calc_results(&self) -> TRIM_CALC_RESULTS_R {
        TRIM_CALC_RESULTS_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Trim Clock Select"]
    #[inline(always)]
    pub fn trim_clk_sel(&mut self) -> TRIM_CLK_SEL_W {
        TRIM_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 1 - Start Trim Calculation"]
    #[inline(always)]
    pub fn trim_calc_start(&mut self) -> TRIM_CALC_START_W {
        TRIM_CALC_START_W::new(self)
    }
    #[doc = "Bit 3 - Trim Logic Enable"]
    #[inline(always)]
    pub fn trim_enable(&mut self) -> TRIM_ENABLE_W {
        TRIM_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim Calculation Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_calc](index.html) module"]
pub struct TRIM_CALC_SPEC;
impl crate::RegisterSpec for TRIM_CALC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_calc::R](R) reader structure"]
impl crate::Readable for TRIM_CALC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_calc::W](W) writer structure"]
impl crate::Writable for TRIM_CALC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_CALC to value 0"]
impl crate::Resettable for TRIM_CALC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
