#[doc = "Register `FS_CLK_DIV` reader"]
pub struct R(crate::R<FS_CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_CLK_DIV` writer"]
pub struct W(crate::W<FS_CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_CLK_DIV_SPEC>;
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
impl From<crate::W<FS_CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fs_filter_clk_div` reader - Full Speed Filter Clock Divisor"]
pub type FS_FILTER_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fs_filter_clk_div` writer - Full Speed Filter Clock Divisor"]
pub type FS_FILTER_CLK_DIV_W<'a> = crate::FieldWriter<'a, u32, FS_CLK_DIV_SPEC, u8, u8, 8, 0>;
#[doc = "Field `fs_scl_lo_cnt` reader - Full Speed SCL Low Count"]
pub type FS_SCL_LO_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `fs_scl_lo_cnt` writer - Full Speed SCL Low Count"]
pub type FS_SCL_LO_CNT_W<'a> = crate::FieldWriter<'a, u32, FS_CLK_DIV_SPEC, u16, u16, 12, 8>;
#[doc = "Field `fs_scl_hi_cnt` reader - Full Speed SCL High Count"]
pub type FS_SCL_HI_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `fs_scl_hi_cnt` writer - Full Speed SCL High Count"]
pub type FS_SCL_HI_CNT_W<'a> = crate::FieldWriter<'a, u32, FS_CLK_DIV_SPEC, u16, u16, 12, 20>;
impl R {
    #[doc = "Bits 0:7 - Full Speed Filter Clock Divisor"]
    #[inline(always)]
    pub fn fs_filter_clk_div(&self) -> FS_FILTER_CLK_DIV_R {
        FS_FILTER_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:19 - Full Speed SCL Low Count"]
    #[inline(always)]
    pub fn fs_scl_lo_cnt(&self) -> FS_SCL_LO_CNT_R {
        FS_SCL_LO_CNT_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - Full Speed SCL High Count"]
    #[inline(always)]
    pub fn fs_scl_hi_cnt(&self) -> FS_SCL_HI_CNT_R {
        FS_SCL_HI_CNT_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Full Speed Filter Clock Divisor"]
    #[inline(always)]
    pub fn fs_filter_clk_div(&mut self) -> FS_FILTER_CLK_DIV_W {
        FS_FILTER_CLK_DIV_W::new(self)
    }
    #[doc = "Bits 8:19 - Full Speed SCL Low Count"]
    #[inline(always)]
    pub fn fs_scl_lo_cnt(&mut self) -> FS_SCL_LO_CNT_W {
        FS_SCL_LO_CNT_W::new(self)
    }
    #[doc = "Bits 20:31 - Full Speed SCL High Count"]
    #[inline(always)]
    pub fn fs_scl_hi_cnt(&mut self) -> FS_SCL_HI_CNT_W {
        FS_SCL_HI_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Full Speed SCL Clock Settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_clk_div](index.html) module"]
pub struct FS_CLK_DIV_SPEC;
impl crate::RegisterSpec for FS_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_clk_div::R](R) reader structure"]
impl crate::Readable for FS_CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_clk_div::W](W) writer structure"]
impl crate::Writable for FS_CLK_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FS_CLK_DIV to value 0"]
impl crate::Resettable for FS_CLK_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
