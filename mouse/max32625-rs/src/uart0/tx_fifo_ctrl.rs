#[doc = "Register `TX_FIFO_CTRL` reader"]
pub struct R(crate::R<TX_FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_FIFO_CTRL` writer"]
pub struct W(crate::W<TX_FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_FIFO_CTRL_SPEC>;
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
impl From<crate::W<TX_FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fifo_entry` reader - TX FIFO Entries"]
pub type FIFO_ENTRY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fifo_ae_lvl` reader - TX FIFO AE Level"]
pub type FIFO_AE_LVL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fifo_ae_lvl` writer - TX FIFO AE Level"]
pub type FIFO_AE_LVL_W<'a> = crate::FieldWriter<'a, u32, TX_FIFO_CTRL_SPEC, u8, u8, 6, 16>;
impl R {
    #[doc = "Bits 0:4 - TX FIFO Entries"]
    #[inline(always)]
    pub fn fifo_entry(&self) -> FIFO_ENTRY_R {
        FIFO_ENTRY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - TX FIFO AE Level"]
    #[inline(always)]
    pub fn fifo_ae_lvl(&self) -> FIFO_AE_LVL_R {
        FIFO_AE_LVL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:21 - TX FIFO AE Level"]
    #[inline(always)]
    pub fn fifo_ae_lvl(&mut self) -> FIFO_AE_LVL_W {
        FIFO_AE_LVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART TX Fifo Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_ctrl](index.html) module"]
pub struct TX_FIFO_CTRL_SPEC;
impl crate::RegisterSpec for TX_FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_fifo_ctrl::R](R) reader structure"]
impl crate::Readable for TX_FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_fifo_ctrl::W](W) writer structure"]
impl crate::Writable for TX_FIFO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_FIFO_CTRL to value 0"]
impl crate::Resettable for TX_FIFO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
