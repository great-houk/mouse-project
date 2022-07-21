#[doc = "Register `SPIB_REQ` reader"]
pub struct R(crate::R<SPIB_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIB_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIB_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIB_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIB_REQ` writer"]
pub struct W(crate::W<SPIB_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIB_REQ_SPEC>;
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
impl From<crate::W<SPIB_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIB_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `core_io_req` reader - SPI Bridge Core I/O Request"]
pub type CORE_IO_REQ_R = crate::BitReader<bool>;
#[doc = "Field `quad_io_req` reader - SPI Bridge Quad I/O Request"]
pub type QUAD_IO_REQ_R = crate::BitReader<bool>;
#[doc = "Field `fast_mode` reader - SPI Bridge Fast Mode Request"]
pub type FAST_MODE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 4 - SPI Bridge Core I/O Request"]
    #[inline(always)]
    pub fn core_io_req(&self) -> CORE_IO_REQ_R {
        CORE_IO_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI Bridge Quad I/O Request"]
    #[inline(always)]
    pub fn quad_io_req(&self) -> QUAD_IO_REQ_R {
        QUAD_IO_REQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI Bridge Fast Mode Request"]
    #[inline(always)]
    pub fn fast_mode(&self) -> FAST_MODE_R {
        FAST_MODE_R::new(((self.bits >> 12) & 1) != 0)
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
#[doc = "SPI Bridge I/O Mode Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spib_req](index.html) module"]
pub struct SPIB_REQ_SPEC;
impl crate::RegisterSpec for SPIB_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spib_req::R](R) reader structure"]
impl crate::Readable for SPIB_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spib_req::W](W) writer structure"]
impl crate::Writable for SPIB_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIB_REQ to value 0"]
impl crate::Resettable for SPIB_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
