#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ow_reset_done` reader - OW Reset Sequence Completed"]
pub type OW_RESET_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ow_reset_done` writer - OW Reset Sequence Completed"]
pub type OW_RESET_DONE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 0>;
#[doc = "Field `tx_data_empty` reader - Tx Data Empty Interrupt Enable"]
pub type TX_DATA_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `tx_data_empty` writer - Tx Data Empty Interrupt Enable"]
pub type TX_DATA_EMPTY_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 1>;
#[doc = "Field `rx_data_ready` reader - Rx Data Ready Interrupt Enable"]
pub type RX_DATA_READY_R = crate::BitReader<bool>;
#[doc = "Field `rx_data_ready` writer - Rx Data Ready Interrupt Enable"]
pub type RX_DATA_READY_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 2>;
#[doc = "Field `line_short` reader - OW Line Short Detected Interrupt Enable"]
pub type LINE_SHORT_R = crate::BitReader<bool>;
#[doc = "Field `line_short` writer - OW Line Short Detected Interrupt Enable"]
pub type LINE_SHORT_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 3>;
#[doc = "Field `line_low` reader - OW Line Low Detected Interrupt Enable"]
pub type LINE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `line_low` writer - OW Line Low Detected Interrupt Enable"]
pub type LINE_LOW_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - OW Reset Sequence Completed"]
    #[inline(always)]
    pub fn ow_reset_done(&self) -> OW_RESET_DONE_R {
        OW_RESET_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx Data Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tx_data_empty(&self) -> TX_DATA_EMPTY_R {
        TX_DATA_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn rx_data_ready(&self) -> RX_DATA_READY_R {
        RX_DATA_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Enable"]
    #[inline(always)]
    pub fn line_short(&self) -> LINE_SHORT_R {
        LINE_SHORT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Enable"]
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
    #[doc = "Bit 1 - Tx Data Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tx_data_empty(&mut self) -> TX_DATA_EMPTY_W {
        TX_DATA_EMPTY_W::new(self)
    }
    #[doc = "Bit 2 - Rx Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn rx_data_ready(&mut self) -> RX_DATA_READY_W {
        RX_DATA_READY_W::new(self)
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Enable"]
    #[inline(always)]
    pub fn line_short(&mut self) -> LINE_SHORT_W {
        LINE_SHORT_W::new(self)
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Enable"]
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
#[doc = "1-Wire Master Interrupt Enables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
