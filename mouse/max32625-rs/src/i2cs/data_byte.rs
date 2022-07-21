#[doc = "Register `DATA_BYTE` reader"]
pub struct R(crate::R<DATA_BYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_BYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_BYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_BYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_BYTE` writer"]
pub struct W(crate::W<DATA_BYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_BYTE_SPEC>;
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
impl From<crate::W<DATA_BYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_BYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data_field` reader - Data Field"]
pub type DATA_FIELD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `data_field` writer - Data Field"]
pub type DATA_FIELD_W<'a> = crate::FieldWriter<'a, u32, DATA_BYTE_SPEC, u8, u8, 8, 0>;
#[doc = "Field `read_only_fl` reader - Read Only Flag"]
pub type READ_ONLY_FL_R = crate::BitReader<bool>;
#[doc = "Field `read_only_fl` writer - Read Only Flag"]
pub type READ_ONLY_FL_W<'a> = crate::BitWriter<'a, u32, DATA_BYTE_SPEC, bool, 8>;
#[doc = "Field `data_updated_fl` reader - Byte Updated Flag"]
pub type DATA_UPDATED_FL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Data Field"]
    #[inline(always)]
    pub fn data_field(&self) -> DATA_FIELD_R {
        DATA_FIELD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Read Only Flag"]
    #[inline(always)]
    pub fn read_only_fl(&self) -> READ_ONLY_FL_R {
        READ_ONLY_FL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Byte Updated Flag"]
    #[inline(always)]
    pub fn data_updated_fl(&self) -> DATA_UPDATED_FL_R {
        DATA_UPDATED_FL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Field"]
    #[inline(always)]
    pub fn data_field(&mut self) -> DATA_FIELD_W {
        DATA_FIELD_W::new(self)
    }
    #[doc = "Bit 8 - Read Only Flag"]
    #[inline(always)]
    pub fn read_only_fl(&mut self) -> READ_ONLY_FL_W {
        READ_ONLY_FL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Data Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_byte](index.html) module"]
pub struct DATA_BYTE_SPEC;
impl crate::RegisterSpec for DATA_BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_byte::R](R) reader structure"]
impl crate::Readable for DATA_BYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_byte::W](W) writer structure"]
impl crate::Writable for DATA_BYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_BYTE to value 0"]
impl crate::Resettable for DATA_BYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
