#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_en` reader - UART Enable"]
pub type UART_EN_R = crate::BitReader<bool>;
#[doc = "Field `uart_en` writer - UART Enable"]
pub type UART_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `rx_fifo_en` reader - RX FIFO Enable"]
pub type RX_FIFO_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_en` writer - RX FIFO Enable"]
pub type RX_FIFO_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `tx_fifo_en` reader - TX FIFO Enable"]
pub type TX_FIFO_EN_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_en` writer - TX FIFO Enable"]
pub type TX_FIFO_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `data_size` reader - Data Size"]
pub type DATA_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `data_size` writer - Data Size"]
pub type DATA_SIZE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 4>;
#[doc = "Field `extra_stop` reader - Extra Stop Enable"]
pub type EXTRA_STOP_R = crate::BitReader<bool>;
#[doc = "Field `extra_stop` writer - Extra Stop Enable"]
pub type EXTRA_STOP_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 8>;
#[doc = "Field `parity` reader - Parity Mode"]
pub type PARITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `parity` writer - Parity Mode"]
pub type PARITY_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 12>;
#[doc = "Field `cts_en` reader - CTS Enable"]
pub type CTS_EN_R = crate::BitReader<bool>;
#[doc = "Field `cts_en` writer - CTS Enable"]
pub type CTS_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 16>;
#[doc = "Field `cts_polarity` reader - CTS Polarity"]
pub type CTS_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `cts_polarity` writer - CTS Polarity"]
pub type CTS_POLARITY_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 17>;
#[doc = "Field `rts_en` reader - RTS Enable"]
pub type RTS_EN_R = crate::BitReader<bool>;
#[doc = "Field `rts_en` writer - RTS Enable"]
pub type RTS_EN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 18>;
#[doc = "Field `rts_polarity` reader - RTS Polarity"]
pub type RTS_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `rts_polarity` writer - RTS Polarity"]
pub type RTS_POLARITY_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 19>;
#[doc = "Field `rts_level` reader - RX FIFO LTE Level for RTS Assert"]
pub type RTS_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rts_level` writer - RX FIFO LTE Level for RTS Assert"]
pub type RTS_LEVEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 6, 20>;
impl R {
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_en(&self) -> UART_EN_R {
        UART_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RX_FIFO_EN_R {
        RX_FIFO_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Enable"]
    #[inline(always)]
    pub fn tx_fifo_en(&self) -> TX_FIFO_EN_R {
        TX_FIFO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data Size"]
    #[inline(always)]
    pub fn data_size(&self) -> DATA_SIZE_R {
        DATA_SIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Extra Stop Enable"]
    #[inline(always)]
    pub fn extra_stop(&self) -> EXTRA_STOP_R {
        EXTRA_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Parity Mode"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - CTS Enable"]
    #[inline(always)]
    pub fn cts_en(&self) -> CTS_EN_R {
        CTS_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CTS Polarity"]
    #[inline(always)]
    pub fn cts_polarity(&self) -> CTS_POLARITY_R {
        CTS_POLARITY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RTS Enable"]
    #[inline(always)]
    pub fn rts_en(&self) -> RTS_EN_R {
        RTS_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RTS Polarity"]
    #[inline(always)]
    pub fn rts_polarity(&self) -> RTS_POLARITY_R {
        RTS_POLARITY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:25 - RX FIFO LTE Level for RTS Assert"]
    #[inline(always)]
    pub fn rts_level(&self) -> RTS_LEVEL_R {
        RTS_LEVEL_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_en(&mut self) -> UART_EN_W {
        UART_EN_W::new(self)
    }
    #[doc = "Bit 1 - RX FIFO Enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&mut self) -> RX_FIFO_EN_W {
        RX_FIFO_EN_W::new(self)
    }
    #[doc = "Bit 2 - TX FIFO Enable"]
    #[inline(always)]
    pub fn tx_fifo_en(&mut self) -> TX_FIFO_EN_W {
        TX_FIFO_EN_W::new(self)
    }
    #[doc = "Bits 4:5 - Data Size"]
    #[inline(always)]
    pub fn data_size(&mut self) -> DATA_SIZE_W {
        DATA_SIZE_W::new(self)
    }
    #[doc = "Bit 8 - Extra Stop Enable"]
    #[inline(always)]
    pub fn extra_stop(&mut self) -> EXTRA_STOP_W {
        EXTRA_STOP_W::new(self)
    }
    #[doc = "Bits 12:13 - Parity Mode"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W::new(self)
    }
    #[doc = "Bit 16 - CTS Enable"]
    #[inline(always)]
    pub fn cts_en(&mut self) -> CTS_EN_W {
        CTS_EN_W::new(self)
    }
    #[doc = "Bit 17 - CTS Polarity"]
    #[inline(always)]
    pub fn cts_polarity(&mut self) -> CTS_POLARITY_W {
        CTS_POLARITY_W::new(self)
    }
    #[doc = "Bit 18 - RTS Enable"]
    #[inline(always)]
    pub fn rts_en(&mut self) -> RTS_EN_W {
        RTS_EN_W::new(self)
    }
    #[doc = "Bit 19 - RTS Polarity"]
    #[inline(always)]
    pub fn rts_polarity(&mut self) -> RTS_POLARITY_W {
        RTS_POLARITY_W::new(self)
    }
    #[doc = "Bits 20:25 - RX FIFO LTE Level for RTS Assert"]
    #[inline(always)]
    pub fn rts_level(&mut self) -> RTS_LEVEL_W {
        RTS_LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
