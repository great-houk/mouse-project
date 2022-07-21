#[doc = "Register `FIFO_STAT` reader"]
pub struct R(crate::R<FIFO_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_STAT` writer"]
pub struct W(crate::W<FIFO_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_STAT_SPEC>;
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
impl From<crate::W<FIFO_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_fifo_used` reader - Number of Bytes in Transmit FIFO"]
pub type TX_FIFO_USED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_fifo_used` reader - Number of Bytes in Receive FIFO"]
pub type RX_FIFO_USED_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Number of Bytes in Transmit FIFO"]
    #[inline(always)]
    pub fn tx_fifo_used(&self) -> TX_FIFO_USED_R {
        TX_FIFO_USED_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Number of Bytes in Receive FIFO"]
    #[inline(always)]
    pub fn rx_fifo_used(&self) -> RX_FIFO_USED_R {
        RX_FIFO_USED_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Slave FIFO Status Information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_stat](index.html) module"]
pub struct FIFO_STAT_SPEC;
impl crate::RegisterSpec for FIFO_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_stat::R](R) reader structure"]
impl crate::Readable for FIFO_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_stat::W](W) writer structure"]
impl crate::Writable for FIFO_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_STAT to value 0"]
impl crate::Resettable for FIFO_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
