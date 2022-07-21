#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `enable` reader - PMU Channel Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `enable` writer - PMU Channel Enable"]
pub type ENABLE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 0>;
#[doc = "Field `ll_stopped` reader - Linked List Engine Status"]
pub type LL_STOPPED_R = crate::BitReader<bool>;
#[doc = "Field `ll_stopped` writer - Linked List Engine Status"]
pub type LL_STOPPED_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 2>;
#[doc = "Field `manual` reader - Manual Mode Enable"]
pub type MANUAL_R = crate::BitReader<bool>;
#[doc = "Field `manual` writer - Manual Mode Enable"]
pub type MANUAL_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 3>;
#[doc = "Field `bus_error` reader - AHB Bus Error Interrupt Flag"]
pub type BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `bus_error` writer - AHB Bus Error Interrupt Flag"]
pub type BUS_ERROR_W<'a> = crate::BitWriter1C<'a, u32, CFG_SPEC, bool, 4>;
#[doc = "Field `to_stat` reader - AHB Bus Timeout Interrupt Flag"]
pub type TO_STAT_R = crate::BitReader<bool>;
#[doc = "Field `to_stat` writer - AHB Bus Timeout Interrupt Flag"]
pub type TO_STAT_W<'a> = crate::BitWriter1C<'a, u32, CFG_SPEC, bool, 6>;
#[doc = "Field `to_sel` reader - Time Out Interval Select"]
pub type TO_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `to_sel` writer - Time Out Interval Select"]
pub type TO_SEL_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 3, 11>;
#[doc = "Field `ps_sel` reader - Time Out Interval Prescale Select"]
pub type PS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ps_sel` writer - Time Out Interval Prescale Select"]
pub type PS_SEL_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, 14>;
#[doc = "Field `interrupt` reader - Descriptor Interrupt Flag"]
pub type INTERRUPT_R = crate::BitReader<bool>;
#[doc = "Field `interrupt` writer - Descriptor Interrupt Flag"]
pub type INTERRUPT_W<'a> = crate::BitWriter1C<'a, u32, CFG_SPEC, bool, 16>;
#[doc = "Field `int_en` reader - PMU Channel Interrupt Enable"]
pub type INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `int_en` writer - PMU Channel Interrupt Enable"]
pub type INT_EN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, bool, 17>;
#[doc = "Field `burst_size` reader - DMA Maximum Burst Size"]
pub type BURST_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `burst_size` writer - DMA Maximum Burst Size"]
pub type BURST_SIZE_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 5, 24>;
impl R {
    #[doc = "Bit 0 - PMU Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Linked List Engine Status"]
    #[inline(always)]
    pub fn ll_stopped(&self) -> LL_STOPPED_R {
        LL_STOPPED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Manual Mode Enable"]
    #[inline(always)]
    pub fn manual(&self) -> MANUAL_R {
        MANUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn bus_error(&self) -> BUS_ERROR_R {
        BUS_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - AHB Bus Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn to_stat(&self) -> TO_STAT_R {
        TO_STAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Time Out Interval Select"]
    #[inline(always)]
    pub fn to_sel(&self) -> TO_SEL_R {
        TO_SEL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Time Out Interval Prescale Select"]
    #[inline(always)]
    pub fn ps_sel(&self) -> PS_SEL_R {
        PS_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Descriptor Interrupt Flag"]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PMU Channel Interrupt Enable"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:28 - DMA Maximum Burst Size"]
    #[inline(always)]
    pub fn burst_size(&self) -> BURST_SIZE_R {
        BURST_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PMU Channel Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Linked List Engine Status"]
    #[inline(always)]
    pub fn ll_stopped(&mut self) -> LL_STOPPED_W {
        LL_STOPPED_W::new(self)
    }
    #[doc = "Bit 3 - Manual Mode Enable"]
    #[inline(always)]
    pub fn manual(&mut self) -> MANUAL_W {
        MANUAL_W::new(self)
    }
    #[doc = "Bit 4 - AHB Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn bus_error(&mut self) -> BUS_ERROR_W {
        BUS_ERROR_W::new(self)
    }
    #[doc = "Bit 6 - AHB Bus Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn to_stat(&mut self) -> TO_STAT_W {
        TO_STAT_W::new(self)
    }
    #[doc = "Bits 11:13 - Time Out Interval Select"]
    #[inline(always)]
    pub fn to_sel(&mut self) -> TO_SEL_W {
        TO_SEL_W::new(self)
    }
    #[doc = "Bits 14:15 - Time Out Interval Prescale Select"]
    #[inline(always)]
    pub fn ps_sel(&mut self) -> PS_SEL_W {
        PS_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Descriptor Interrupt Flag"]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W {
        INTERRUPT_W::new(self)
    }
    #[doc = "Bit 17 - PMU Channel Interrupt Enable"]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W {
        INT_EN_W::new(self)
    }
    #[doc = "Bits 24:28 - DMA Maximum Burst Size"]
    #[inline(always)]
    pub fn burst_size(&mut self) -> BURST_SIZE_W {
        BURST_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
