#[doc = "Register `UART3_REQ` reader"]
pub struct R(crate::R<UART3_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART3_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART3_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART3_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART3_REQ` writer"]
pub struct W(crate::W<UART3_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART3_REQ_SPEC>;
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
impl From<crate::W<UART3_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART3_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `io_map` reader - UART3 TX/RX I/O Mapping Select"]
pub type IO_MAP_R = crate::BitReader<bool>;
#[doc = "Field `io_map` writer - UART3 TX/RX I/O Mapping Select"]
pub type IO_MAP_W<'a> = crate::BitWriter<'a, u32, UART3_REQ_SPEC, bool, 0>;
#[doc = "Field `cts_map` reader - UART3 CTS I/O Mapping Select"]
pub type CTS_MAP_R = crate::BitReader<bool>;
#[doc = "Field `cts_map` writer - UART3 CTS I/O Mapping Select"]
pub type CTS_MAP_W<'a> = crate::BitWriter<'a, u32, UART3_REQ_SPEC, bool, 1>;
#[doc = "Field `rts_map` reader - UART3 RTS I/O Mapping Select"]
pub type RTS_MAP_R = crate::BitReader<bool>;
#[doc = "Field `rts_map` writer - UART3 RTS I/O Mapping Select"]
pub type RTS_MAP_W<'a> = crate::BitWriter<'a, u32, UART3_REQ_SPEC, bool, 2>;
#[doc = "Field `io_req` reader - UART3 TX/RX I/O Request"]
pub type IO_REQ_R = crate::BitReader<bool>;
#[doc = "Field `io_req` writer - UART3 TX/RX I/O Request"]
pub type IO_REQ_W<'a> = crate::BitWriter<'a, u32, UART3_REQ_SPEC, bool, 4>;
#[doc = "Field `cts_io_req` reader - UART3 CTS I/O Request"]
pub type CTS_IO_REQ_R = crate::BitReader<bool>;
#[doc = "Field `cts_io_req` writer - UART3 CTS I/O Request"]
pub type CTS_IO_REQ_W<'a> = crate::BitWriter<'a, u32, UART3_REQ_SPEC, bool, 5>;
#[doc = "Field `rts_io_req` reader - UART3 RTS I/O Request"]
pub type RTS_IO_REQ_R = crate::BitReader<bool>;
#[doc = "Field `rts_io_req` writer - UART3 RTS I/O Request"]
pub type RTS_IO_REQ_W<'a> = crate::BitWriter<'a, u32, UART3_REQ_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - UART3 TX/RX I/O Mapping Select"]
    #[inline(always)]
    pub fn io_map(&self) -> IO_MAP_R {
        IO_MAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART3 CTS I/O Mapping Select"]
    #[inline(always)]
    pub fn cts_map(&self) -> CTS_MAP_R {
        CTS_MAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART3 RTS I/O Mapping Select"]
    #[inline(always)]
    pub fn rts_map(&self) -> RTS_MAP_R {
        RTS_MAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - UART3 TX/RX I/O Request"]
    #[inline(always)]
    pub fn io_req(&self) -> IO_REQ_R {
        IO_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART3 CTS I/O Request"]
    #[inline(always)]
    pub fn cts_io_req(&self) -> CTS_IO_REQ_R {
        CTS_IO_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART3 RTS I/O Request"]
    #[inline(always)]
    pub fn rts_io_req(&self) -> RTS_IO_REQ_R {
        RTS_IO_REQ_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART3 TX/RX I/O Mapping Select"]
    #[inline(always)]
    pub fn io_map(&mut self) -> IO_MAP_W {
        IO_MAP_W::new(self)
    }
    #[doc = "Bit 1 - UART3 CTS I/O Mapping Select"]
    #[inline(always)]
    pub fn cts_map(&mut self) -> CTS_MAP_W {
        CTS_MAP_W::new(self)
    }
    #[doc = "Bit 2 - UART3 RTS I/O Mapping Select"]
    #[inline(always)]
    pub fn rts_map(&mut self) -> RTS_MAP_W {
        RTS_MAP_W::new(self)
    }
    #[doc = "Bit 4 - UART3 TX/RX I/O Request"]
    #[inline(always)]
    pub fn io_req(&mut self) -> IO_REQ_W {
        IO_REQ_W::new(self)
    }
    #[doc = "Bit 5 - UART3 CTS I/O Request"]
    #[inline(always)]
    pub fn cts_io_req(&mut self) -> CTS_IO_REQ_W {
        CTS_IO_REQ_W::new(self)
    }
    #[doc = "Bit 6 - UART3 RTS I/O Request"]
    #[inline(always)]
    pub fn rts_io_req(&mut self) -> RTS_IO_REQ_W {
        RTS_IO_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART3 I/O Mode Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart3_req](index.html) module"]
pub struct UART3_REQ_SPEC;
impl crate::RegisterSpec for UART3_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart3_req::R](R) reader structure"]
impl crate::Readable for UART3_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart3_req::W](W) writer structure"]
impl crate::Writable for UART3_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART3_REQ to value 0"]
impl crate::Resettable for UART3_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
