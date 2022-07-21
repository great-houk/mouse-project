#[doc = "Register `OWM_REQ` reader"]
pub struct R(crate::R<OWM_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OWM_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OWM_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OWM_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OWM_REQ` writer"]
pub struct W(crate::W<OWM_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OWM_REQ_SPEC>;
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
impl From<crate::W<OWM_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OWM_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mapping_req` reader - 1-Wire Line I/O Request"]
pub type MAPPING_REQ_R = crate::BitReader<bool>;
#[doc = "Field `mapping_req` writer - 1-Wire Line I/O Request"]
pub type MAPPING_REQ_W<'a> = crate::BitWriter<'a, u32, OWM_REQ_SPEC, bool, 4>;
#[doc = "Field `epu_io_req` reader - External Pullup Control Line I/O Request"]
pub type EPU_IO_REQ_R = crate::BitReader<bool>;
#[doc = "Field `epu_io_req` writer - External Pullup Control Line I/O Request"]
pub type EPU_IO_REQ_W<'a> = crate::BitWriter<'a, u32, OWM_REQ_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 4 - 1-Wire Line I/O Request"]
    #[inline(always)]
    pub fn mapping_req(&self) -> MAPPING_REQ_R {
        MAPPING_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Pullup Control Line I/O Request"]
    #[inline(always)]
    pub fn epu_io_req(&self) -> EPU_IO_REQ_R {
        EPU_IO_REQ_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 1-Wire Line I/O Request"]
    #[inline(always)]
    pub fn mapping_req(&mut self) -> MAPPING_REQ_W {
        MAPPING_REQ_W::new(self)
    }
    #[doc = "Bit 5 - External Pullup Control Line I/O Request"]
    #[inline(always)]
    pub fn epu_io_req(&mut self) -> EPU_IO_REQ_W {
        EPU_IO_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1-Wire Master I/O Mode Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [owm_req](index.html) module"]
pub struct OWM_REQ_SPEC;
impl crate::RegisterSpec for OWM_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [owm_req::R](R) reader structure"]
impl crate::Readable for OWM_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [owm_req::W](W) writer structure"]
impl crate::Writable for OWM_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OWM_REQ to value 0"]
impl crate::Resettable for OWM_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
