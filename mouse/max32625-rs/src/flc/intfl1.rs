#[doc = "Register `INTFL1` reader"]
pub struct R(crate::R<INTFL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFL1` writer"]
pub struct W(crate::W<INTFL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFL1_SPEC>;
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
impl From<crate::W<INTFL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sram_addr_wrapped` reader - SRAM Address Wrapped Interrupt Flag"]
pub type SRAM_ADDR_WRAPPED_R = crate::BitReader<bool>;
#[doc = "Field `sram_addr_wrapped` writer - SRAM Address Wrapped Interrupt Flag"]
pub type SRAM_ADDR_WRAPPED_W<'a> = crate::BitWriter1C<'a, u32, INTFL1_SPEC, bool, 0>;
#[doc = "Field `invalid_flash_addr` reader - Invalid Flash Address Interrupt Flag"]
pub type INVALID_FLASH_ADDR_R = crate::BitReader<bool>;
#[doc = "Field `invalid_flash_addr` writer - Invalid Flash Address Interrupt Flag"]
pub type INVALID_FLASH_ADDR_W<'a> = crate::BitWriter1C<'a, u32, INTFL1_SPEC, bool, 1>;
#[doc = "Field `flash_read_locked` reader - Flash Read from Locked Area Interrupt Flag"]
pub type FLASH_READ_LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `flash_read_locked` writer - Flash Read from Locked Area Interrupt Flag"]
pub type FLASH_READ_LOCKED_W<'a> = crate::BitWriter1C<'a, u32, INTFL1_SPEC, bool, 2>;
#[doc = "Field `trim_update_done` reader - Trim Update Complete Interrupt Flag"]
pub type TRIM_UPDATE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `trim_update_done` writer - Trim Update Complete Interrupt Flag"]
pub type TRIM_UPDATE_DONE_W<'a> = crate::BitWriter1C<'a, u32, INTFL1_SPEC, bool, 3>;
#[doc = "Field `flc_state_done` reader - FLC State Machine Reached DONE Interrupt Flag"]
pub type FLC_STATE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `flc_state_done` writer - FLC State Machine Reached DONE Interrupt Flag"]
pub type FLC_STATE_DONE_W<'a> = crate::BitWriter1C<'a, u32, INTFL1_SPEC, bool, 4>;
#[doc = "Field `flc_prog_complete` reader - Program (Write or Erase) Operation Completed Interrupt Flag"]
pub type FLC_PROG_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `flc_prog_complete` writer - Program (Write or Erase) Operation Completed Interrupt Flag"]
pub type FLC_PROG_COMPLETE_W<'a> = crate::BitWriter1C<'a, u32, INTFL1_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - SRAM Address Wrapped Interrupt Flag"]
    #[inline(always)]
    pub fn sram_addr_wrapped(&self) -> SRAM_ADDR_WRAPPED_R {
        SRAM_ADDR_WRAPPED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalid Flash Address Interrupt Flag"]
    #[inline(always)]
    pub fn invalid_flash_addr(&self) -> INVALID_FLASH_ADDR_R {
        INVALID_FLASH_ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Read from Locked Area Interrupt Flag"]
    #[inline(always)]
    pub fn flash_read_locked(&self) -> FLASH_READ_LOCKED_R {
        FLASH_READ_LOCKED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trim Update Complete Interrupt Flag"]
    #[inline(always)]
    pub fn trim_update_done(&self) -> TRIM_UPDATE_DONE_R {
        TRIM_UPDATE_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FLC State Machine Reached DONE Interrupt Flag"]
    #[inline(always)]
    pub fn flc_state_done(&self) -> FLC_STATE_DONE_R {
        FLC_STATE_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Program (Write or Erase) Operation Completed Interrupt Flag"]
    #[inline(always)]
    pub fn flc_prog_complete(&self) -> FLC_PROG_COMPLETE_R {
        FLC_PROG_COMPLETE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM Address Wrapped Interrupt Flag"]
    #[inline(always)]
    pub fn sram_addr_wrapped(&mut self) -> SRAM_ADDR_WRAPPED_W {
        SRAM_ADDR_WRAPPED_W::new(self)
    }
    #[doc = "Bit 1 - Invalid Flash Address Interrupt Flag"]
    #[inline(always)]
    pub fn invalid_flash_addr(&mut self) -> INVALID_FLASH_ADDR_W {
        INVALID_FLASH_ADDR_W::new(self)
    }
    #[doc = "Bit 2 - Flash Read from Locked Area Interrupt Flag"]
    #[inline(always)]
    pub fn flash_read_locked(&mut self) -> FLASH_READ_LOCKED_W {
        FLASH_READ_LOCKED_W::new(self)
    }
    #[doc = "Bit 3 - Trim Update Complete Interrupt Flag"]
    #[inline(always)]
    pub fn trim_update_done(&mut self) -> TRIM_UPDATE_DONE_W {
        TRIM_UPDATE_DONE_W::new(self)
    }
    #[doc = "Bit 4 - FLC State Machine Reached DONE Interrupt Flag"]
    #[inline(always)]
    pub fn flc_state_done(&mut self) -> FLC_STATE_DONE_W {
        FLC_STATE_DONE_W::new(self)
    }
    #[doc = "Bit 5 - Program (Write or Erase) Operation Completed Interrupt Flag"]
    #[inline(always)]
    pub fn flc_prog_complete(&mut self) -> FLC_PROG_COMPLETE_W {
        FLC_PROG_COMPLETE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl1](index.html) module"]
pub struct INTFL1_SPEC;
impl crate::RegisterSpec for INTFL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intfl1::R](R) reader structure"]
impl crate::Readable for INTFL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intfl1::W](W) writer structure"]
impl crate::Writable for INTFL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFL1 to value 0"]
impl crate::Resettable for INTFL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
