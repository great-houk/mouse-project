#[doc = "Register `I2CM1_REQ` reader"]
pub struct R(crate::R<I2CM1_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CM1_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CM1_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CM1_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CM1_REQ` writer"]
pub struct W(crate::W<I2CM1_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CM1_REQ_SPEC>;
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
impl From<crate::W<I2CM1_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CM1_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mapping_req` reader - I2C Master 1 I/O Request"]
pub type MAPPING_REQ_R = crate::BitReader<bool>;
#[doc = "Field `mapping_req` writer - I2C Master 1 I/O Request"]
pub type MAPPING_REQ_W<'a> = crate::BitWriter<'a, u32, I2CM1_REQ_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 4 - I2C Master 1 I/O Request"]
    #[inline(always)]
    pub fn mapping_req(&self) -> MAPPING_REQ_R {
        MAPPING_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - I2C Master 1 I/O Request"]
    #[inline(always)]
    pub fn mapping_req(&mut self) -> MAPPING_REQ_W {
        MAPPING_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master 1 I/O Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cm1_req](index.html) module"]
pub struct I2CM1_REQ_SPEC;
impl crate::RegisterSpec for I2CM1_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cm1_req::R](R) reader structure"]
impl crate::Readable for I2CM1_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cm1_req::W](W) writer structure"]
impl crate::Writable for I2CM1_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2CM1_REQ to value 0"]
impl crate::Resettable for I2CM1_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
