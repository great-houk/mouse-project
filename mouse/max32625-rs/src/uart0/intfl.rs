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
#[doc = "Field `tx_done` reader - TX Done Interrupt Flag"]
pub type TX_DONE_R = crate::BitReader<bool>;
#[doc = "Field `tx_done` writer - TX Done Interrupt Flag"]
pub type TX_DONE_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 0>;
#[doc = "Field `tx_unstalled` reader - TX Unstalled Interrupt Flag"]
pub type TX_UNSTALLED_R = crate::BitReader<bool>;
#[doc = "Field `tx_unstalled` writer - TX Unstalled Interrupt Flag"]
pub type TX_UNSTALLED_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 1>;
#[doc = "Field `tx_fifo_ae` reader - TX FIFO Almost Empty Interrupt Flag"]
pub type TX_FIFO_AE_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_ae` writer - TX FIFO Almost Empty Interrupt Flag"]
pub type TX_FIFO_AE_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 2>;
#[doc = "Field `rx_fifo_not_empty` reader - RX FIFO Not Empty Interrupt Flag"]
pub type RX_FIFO_NOT_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_not_empty` writer - RX FIFO Not Empty Interrupt Flag"]
pub type RX_FIFO_NOT_EMPTY_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 3>;
#[doc = "Field `rx_stalled` reader - RX Stalled Interrupt Flag"]
pub type RX_STALLED_R = crate::BitReader<bool>;
#[doc = "Field `rx_stalled` writer - RX Stalled Interrupt Flag"]
pub type RX_STALLED_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 4>;
#[doc = "Field `rx_fifo_af` reader - RX FIFO Almost Full Interrupt Flag"]
pub type RX_FIFO_AF_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_af` writer - RX FIFO Almost Full Interrupt Flag"]
pub type RX_FIFO_AF_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 5>;
#[doc = "Field `rx_fifo_overflow` reader - RX FIFO Overflow Interrupt Flag"]
pub type RX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_overflow` writer - RX FIFO Overflow Interrupt Flag"]
pub type RX_FIFO_OVERFLOW_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 6>;
#[doc = "Field `rx_framing_err` reader - RX Framing Error Interrupt Flag"]
pub type RX_FRAMING_ERR_R = crate::BitReader<bool>;
#[doc = "Field `rx_framing_err` writer - RX Framing Error Interrupt Flag"]
pub type RX_FRAMING_ERR_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 7>;
#[doc = "Field `rx_parity_err` reader - RX Parity Error Interrupt Flag"]
pub type RX_PARITY_ERR_R = crate::BitReader<bool>;
#[doc = "Field `rx_parity_err` writer - RX Parity Error Interrupt Flag"]
pub type RX_PARITY_ERR_W<'a> = crate::BitWriter1C<'a, u32, INTFL_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - TX Done Interrupt Flag"]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Unstalled Interrupt Flag"]
    #[inline(always)]
    pub fn tx_unstalled(&self) -> TX_UNSTALLED_R {
        TX_UNSTALLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Almost Empty Interrupt Flag"]
    #[inline(always)]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AE_R {
        TX_FIFO_AE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Not Empty Interrupt Flag"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&self) -> RX_FIFO_NOT_EMPTY_R {
        RX_FIFO_NOT_EMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Stalled Interrupt Flag"]
    #[inline(always)]
    pub fn rx_stalled(&self) -> RX_STALLED_R {
        RX_STALLED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO Almost Full Interrupt Flag"]
    #[inline(always)]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AF_R {
        RX_FIFO_AF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn rx_framing_err(&self) -> RX_FRAMING_ERR_R {
        RX_FRAMING_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn rx_parity_err(&self) -> RX_PARITY_ERR_R {
        RX_PARITY_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Done Interrupt Flag"]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W {
        TX_DONE_W::new(self)
    }
    #[doc = "Bit 1 - TX Unstalled Interrupt Flag"]
    #[inline(always)]
    pub fn tx_unstalled(&mut self) -> TX_UNSTALLED_W {
        TX_UNSTALLED_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO Almost Empty Interrupt Flag"]
    #[inline(always)]
    pub fn tx_fifo_ae(&mut self) -> TX_FIFO_AE_W {
        TX_FIFO_AE_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO Not Empty Interrupt Flag"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&mut self) -> RX_FIFO_NOT_EMPTY_W {
        RX_FIFO_NOT_EMPTY_W::new(self)
    }
    #[doc = "Bit 4 - RX Stalled Interrupt Flag"]
    #[inline(always)]
    pub fn rx_stalled(&mut self) -> RX_STALLED_W {
        RX_STALLED_W::new(self)
    }
    #[doc = "Bit 5 - RX FIFO Almost Full Interrupt Flag"]
    #[inline(always)]
    pub fn rx_fifo_af(&mut self) -> RX_FIFO_AF_W {
        RX_FIFO_AF_W::new(self)
    }
    #[doc = "Bit 6 - RX FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&mut self) -> RX_FIFO_OVERFLOW_W {
        RX_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 7 - RX Framing Error Interrupt Flag"]
    #[inline(always)]
    pub fn rx_framing_err(&mut self) -> RX_FRAMING_ERR_W {
        RX_FRAMING_ERR_W::new(self)
    }
    #[doc = "Bit 8 - RX Parity Error Interrupt Flag"]
    #[inline(always)]
    pub fn rx_parity_err(&mut self) -> RX_PARITY_ERR_W {
        RX_PARITY_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](index.html) module"]
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
