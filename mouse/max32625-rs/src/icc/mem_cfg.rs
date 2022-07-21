#[doc = "Register `MEM_CFG` reader"]
pub struct R(crate::R<MEM_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_CFG` writer"]
pub struct W(crate::W<MEM_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_CFG_SPEC>;
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
impl From<crate::W<MEM_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cache_size` reader - Instruction Cache Size"]
pub type CACHE_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `main_memory_size` reader - Internal Flash Memory Size"]
pub type MAIN_MEMORY_SIZE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Instruction Cache Size"]
    #[inline(always)]
    pub fn cache_size(&self) -> CACHE_SIZE_R {
        CACHE_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Internal Flash Memory Size"]
    #[inline(always)]
    pub fn main_memory_size(&self) -> MAIN_MEMORY_SIZE_R {
        MAIN_MEMORY_SIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
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
#[doc = "Memory Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_cfg](index.html) module"]
pub struct MEM_CFG_SPEC;
impl crate::RegisterSpec for MEM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_cfg::R](R) reader structure"]
impl crate::Readable for MEM_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_cfg::W](W) writer structure"]
impl crate::Writable for MEM_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MEM_CFG to value 0"]
impl crate::Resettable for MEM_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
