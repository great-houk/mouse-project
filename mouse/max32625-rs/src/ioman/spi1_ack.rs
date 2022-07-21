#[doc = "Register `SPI1_ACK` reader"]
pub struct R(crate::R<SPI1_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI1_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI1_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI1_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI1_ACK` writer"]
pub struct W(crate::W<SPI1_ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI1_ACK_SPEC>;
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
impl From<crate::W<SPI1_ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI1_ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `core_io_ack` reader - SPI Master 1 Core I/O Acknowledge"]
pub type CORE_IO_ACK_R = crate::BitReader<bool>;
#[doc = "Field `ss0_io_ack` reader - SPI Master 1 SS\\[0\\]
I/O Acknowledge"]
pub type SS0_IO_ACK_R = crate::BitReader<bool>;
#[doc = "Field `ss1_io_ack` reader - SPI Master 1 SS\\[1\\]
I/O Acknowledge"]
pub type SS1_IO_ACK_R = crate::BitReader<bool>;
#[doc = "Field `ss2_io_ack` reader - SPI Master 1 SS\\[2\\]
I/O Acknowledge"]
pub type SS2_IO_ACK_R = crate::BitReader<bool>;
#[doc = "Field `quad_io_ack` reader - SPI Master 1 Quad I/O Acknowledge"]
pub type QUAD_IO_ACK_R = crate::BitReader<bool>;
#[doc = "Field `fast_mode` reader - SPI Master 1 Fast Mode Acknowledge"]
pub type FAST_MODE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 4 - SPI Master 1 Core I/O Acknowledge"]
    #[inline(always)]
    pub fn core_io_ack(&self) -> CORE_IO_ACK_R {
        CORE_IO_ACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI Master 1 SS\\[0\\]
I/O Acknowledge"]
    #[inline(always)]
    pub fn ss0_io_ack(&self) -> SS0_IO_ACK_R {
        SS0_IO_ACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI Master 1 SS\\[1\\]
I/O Acknowledge"]
    #[inline(always)]
    pub fn ss1_io_ack(&self) -> SS1_IO_ACK_R {
        SS1_IO_ACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI Master 1 SS\\[2\\]
I/O Acknowledge"]
    #[inline(always)]
    pub fn ss2_io_ack(&self) -> SS2_IO_ACK_R {
        SS2_IO_ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI Master 1 Quad I/O Acknowledge"]
    #[inline(always)]
    pub fn quad_io_ack(&self) -> QUAD_IO_ACK_R {
        QUAD_IO_ACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - SPI Master 1 Fast Mode Acknowledge"]
    #[inline(always)]
    pub fn fast_mode(&self) -> FAST_MODE_R {
        FAST_MODE_R::new(((self.bits >> 24) & 1) != 0)
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
#[doc = "SPI Master 1 I/O Mode Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_ack](index.html) module"]
pub struct SPI1_ACK_SPEC;
impl crate::RegisterSpec for SPI1_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi1_ack::R](R) reader structure"]
impl crate::Readable for SPI1_ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi1_ack::W](W) writer structure"]
impl crate::Writable for SPI1_ACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI1_ACK to value 0"]
impl crate::Resettable for SPI1_ACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
