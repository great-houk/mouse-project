#[doc = "Register `UART0_ACK` reader"]
pub struct R(crate::R<UART0_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART0_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART0_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART0_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART0_ACK` writer"]
pub struct W(crate::W<UART0_ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART0_ACK_SPEC>;
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
impl From<crate::W<UART0_ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART0_ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `io_map` reader - UART0 TX/RX I/O Mapping Acknowledge"]
pub type IO_MAP_R = crate::BitReader<bool>;
#[doc = "Field `cts_map` reader - UART0 CTS I/O Mapping Acknowledge"]
pub type CTS_MAP_R = crate::BitReader<bool>;
#[doc = "Field `rts_map` reader - UART0 RTS I/O Mapping Acknowledge"]
pub type RTS_MAP_R = crate::BitReader<bool>;
#[doc = "Field `io_req` reader - UART0 TX/RX I/O Acknowledge"]
pub type IO_REQ_R = crate::BitReader<bool>;
#[doc = "Field `cts_io_req` reader - UART0 CTS I/O Acknowledge"]
pub type CTS_IO_REQ_R = crate::BitReader<bool>;
#[doc = "Field `rts_io_req` reader - UART0 RTS I/O Acknowledge"]
pub type RTS_IO_REQ_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - UART0 TX/RX I/O Mapping Acknowledge"]
    #[inline(always)]
    pub fn io_map(&self) -> IO_MAP_R {
        IO_MAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART0 CTS I/O Mapping Acknowledge"]
    #[inline(always)]
    pub fn cts_map(&self) -> CTS_MAP_R {
        CTS_MAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART0 RTS I/O Mapping Acknowledge"]
    #[inline(always)]
    pub fn rts_map(&self) -> RTS_MAP_R {
        RTS_MAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - UART0 TX/RX I/O Acknowledge"]
    #[inline(always)]
    pub fn io_req(&self) -> IO_REQ_R {
        IO_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART0 CTS I/O Acknowledge"]
    #[inline(always)]
    pub fn cts_io_req(&self) -> CTS_IO_REQ_R {
        CTS_IO_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART0 RTS I/O Acknowledge"]
    #[inline(always)]
    pub fn rts_io_req(&self) -> RTS_IO_REQ_R {
        RTS_IO_REQ_R::new(((self.bits >> 6) & 1) != 0)
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
#[doc = "UART0 I/O Mode Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_ack](index.html) module"]
pub struct UART0_ACK_SPEC;
impl crate::RegisterSpec for UART0_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart0_ack::R](R) reader structure"]
impl crate::Readable for UART0_ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart0_ack::W](W) writer structure"]
impl crate::Writable for UART0_ACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART0_ACK to value 0"]
impl crate::Resettable for UART0_ACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
