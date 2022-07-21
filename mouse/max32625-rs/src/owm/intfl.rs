#[doc = "Register `INTFL` reader"]
pub struct R(crate::R<INTFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFL` writer"]
pub struct W(crate::W<INTFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFL_SPEC>;
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
impl From<crate::W<INTFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ow_reset_done` reader - OW Reset Sequence Completed"]
pub type OW_RESET_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ow_reset_done` writer - OW Reset Sequence Completed"]
pub type OW_RESET_DONE_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 0>;
#[doc = "Field `tx_data_empty` reader - Tx Data Empty Interrupt Flag"]
pub type TX_DATA_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `tx_data_empty` writer - Tx Data Empty Interrupt Flag"]
pub type TX_DATA_EMPTY_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 1>;
#[doc = "Field `rx_data_ready` reader - Rx Data Ready Interrupt Flag"]
pub type RX_DATA_READY_R = crate::BitReader<bool>;
#[doc = "Field `rx_data_ready` writer - Rx Data Ready Interrupt Flag"]
pub type RX_DATA_READY_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 2>;
#[doc = "Field `line_short` reader - OW Line Short Detected Interrupt Flag"]
pub type LINE_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `line_short` writer - OW Line Short Detected Interrupt Flag"]
pub type LINE_SHORT_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 3>;
#[doc = "Field `line_low` reader - OW Line Low Detected Interrupt Flag"]
pub type LINE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `line_low` writer - OW Line Low Detected Interrupt Flag"]
pub type LINE_LOW_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - OW Reset Sequence Completed"]
    #[inline(always)]
    pub fn ow_reset_done(&self) -> OW_RESET_DONE_R {
        OW_RESET_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx Data Empty Interrupt Flag"]
    #[inline(always)]
    pub fn tx_data_empty(&self) -> TX_DATA_EMPTY_R {
        TX_DATA_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx Data Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rx_data_ready(&self) -> RX_DATA_READY_R {
        RX_DATA_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Flag"]
    #[inline(always)]
    pub fn line_short(&self) -> LINE_SHORT_R {
        LINE_SHORT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Flag"]
    #[inline(always)]
    pub fn line_low(&self) -> LINE_LOW_R {
        LINE_LOW_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OW Reset Sequence Completed"]
    #[inline(always)]
    pub fn ow_reset_done(&mut self) -> OW_RESET_DONE_W {
        OW_RESET_DONE_W::new(self)
    }
    #[doc = "Bit 1 - Tx Data Empty Interrupt Flag"]
    #[inline(always)]
    pub fn tx_data_empty(&mut self) -> TX_DATA_EMPTY_W {
        TX_DATA_EMPTY_W::new(self)
    }
    #[doc = "Bit 2 - Rx Data Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rx_data_ready(&mut self) -> RX_DATA_READY_W {
        RX_DATA_READY_W::new(self)
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Flag"]
    #[inline(always)]
    pub fn line_short(&mut self) -> LINE_SHORT_W {
        LINE_SHORT_W::new(self)
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Flag"]
    #[inline(always)]
    pub fn line_low(&mut self) -> LINE_LOW_W {
        LINE_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1-Wire Master Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](index.html) module"]
pub struct INTFL_SPEC;
impl crate::RegisterSpec for INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intfl::R](R) reader structure"]
impl crate::Readable for INTFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intfl::W](W) writer structure"]
impl crate::Writable for INTFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for INTFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
