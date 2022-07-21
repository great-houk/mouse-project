#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `jtag_lock_window` reader - Debug Locked - Hardware Window"]
pub type JTAG_LOCK_WINDOW_R = crate::BitReader<bool>;
#[doc = "Field `jtag_lock_static` reader - Debug Locked - Firmware Lockout"]
pub type JTAG_LOCK_STATIC_R = crate::BitReader<bool>;
#[doc = "Field `auto_lock` reader - Debug Locked - Auto Lock"]
pub type AUTO_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `trim_update_done` reader - Trim Update Done"]
pub type TRIM_UPDATE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `info_block_valid` reader - Info Block Valid"]
pub type INFO_BLOCK_VALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Debug Locked - Hardware Window"]
    #[inline(always)]
    pub fn jtag_lock_window(&self) -> JTAG_LOCK_WINDOW_R {
        JTAG_LOCK_WINDOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Locked - Firmware Lockout"]
    #[inline(always)]
    pub fn jtag_lock_static(&self) -> JTAG_LOCK_STATIC_R {
        JTAG_LOCK_STATIC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Locked - Auto Lock"]
    #[inline(always)]
    pub fn auto_lock(&self) -> AUTO_LOCK_R {
        AUTO_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 29 - Trim Update Done"]
    #[inline(always)]
    pub fn trim_update_done(&self) -> TRIM_UPDATE_DONE_R {
        TRIM_UPDATE_DONE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Info Block Valid"]
    #[inline(always)]
    pub fn info_block_valid(&self) -> INFO_BLOCK_VALID_R {
        INFO_BLOCK_VALID_R::new(((self.bits >> 30) & 1) != 0)
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
#[doc = "Security Status Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
