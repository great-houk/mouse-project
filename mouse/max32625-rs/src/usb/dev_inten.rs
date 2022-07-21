#[doc = "Register `DEV_INTEN` reader"]
pub struct R(crate::R<DEV_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEV_INTEN` writer"]
pub struct W(crate::W<DEV_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEV_INTEN_SPEC>;
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
impl From<crate::W<DEV_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEV_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dpact` reader - DPLUS Activity Interrupt Flag"]
pub type DPACT_R = crate::BitReader<bool>;
#[doc = "Field `dpact` writer - DPLUS Activity Interrupt Flag"]
pub type DPACT_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 0>;
#[doc = "Field `rwu_dn` reader - Remote Wakeup Done Interrupt Flag"]
pub type RWU_DN_R = crate::BitReader<bool>;
#[doc = "Field `rwu_dn` writer - Remote Wakeup Done Interrupt Flag"]
pub type RWU_DN_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 1>;
#[doc = "Field `bact` reader - USB Bus Activity Interrupt Flag"]
pub type BACT_R = crate::BitReader<bool>;
#[doc = "Field `bact` writer - USB Bus Activity Interrupt Flag"]
pub type BACT_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 2>;
#[doc = "Field `brst` reader - USB Bus Reset In Progress Interrupt Flag"]
pub type BRST_R = crate::BitReader<bool>;
#[doc = "Field `brst` writer - USB Bus Reset In Progress Interrupt Flag"]
pub type BRST_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 3>;
#[doc = "Field `susp` reader - USB Suspend Interrupt Flag"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `susp` writer - USB Suspend Interrupt Flag"]
pub type SUSP_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 4>;
#[doc = "Field `no_vbus` reader - No VBUS Interrupt Flag"]
pub type NO_VBUS_R = crate::BitReader<bool>;
#[doc = "Field `no_vbus` writer - No VBUS Interrupt Flag"]
pub type NO_VBUS_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 5>;
#[doc = "Field `vbus` reader - VBUS Detect Interrupt Flag"]
pub type VBUS_R = crate::BitReader<bool>;
#[doc = "Field `vbus` writer - VBUS Detect Interrupt Flag"]
pub type VBUS_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 6>;
#[doc = "Field `brst_dn` reader - USB Bus Reset Completed Interrupt Flag"]
pub type BRST_DN_R = crate::BitReader<bool>;
#[doc = "Field `brst_dn` writer - USB Bus Reset Completed Interrupt Flag"]
pub type BRST_DN_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 7>;
#[doc = "Field `setup` reader - Setup Packet Interrupt Flag"]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `setup` writer - Setup Packet Interrupt Flag"]
pub type SETUP_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 8>;
#[doc = "Field `ep_in` reader - Endpoint IN Interrupt Flag"]
pub type EP_IN_R = crate::BitReader<bool>;
#[doc = "Field `ep_in` writer - Endpoint IN Interrupt Flag"]
pub type EP_IN_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 9>;
#[doc = "Field `ep_out` reader - Endpoint OUT Interrupt Flag"]
pub type EP_OUT_R = crate::BitReader<bool>;
#[doc = "Field `ep_out` writer - Endpoint OUT Interrupt Flag"]
pub type EP_OUT_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 10>;
#[doc = "Field `ep_nak` reader - Endpoint NAK Interrupt Flag"]
pub type EP_NAK_R = crate::BitReader<bool>;
#[doc = "Field `ep_nak` writer - Endpoint NAK Interrupt Flag"]
pub type EP_NAK_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 11>;
#[doc = "Field `dma_err` reader - DMA Error Interrupt Flag"]
pub type DMA_ERR_R = crate::BitReader<bool>;
#[doc = "Field `dma_err` writer - DMA Error Interrupt Flag"]
pub type DMA_ERR_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 12>;
#[doc = "Field `buf_ovr` reader - Buffer Overflow Interrupt Flag"]
pub type BUF_OVR_R = crate::BitReader<bool>;
#[doc = "Field `buf_ovr` writer - Buffer Overflow Interrupt Flag"]
pub type BUF_OVR_W<'a> = crate::BitWriter<'a, u32, DEV_INTEN_SPEC, bool, 13>;
impl R {
    #[doc = "Bit 0 - DPLUS Activity Interrupt Flag"]
    #[inline(always)]
    pub fn dpact(&self) -> DPACT_R {
        DPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remote Wakeup Done Interrupt Flag"]
    #[inline(always)]
    pub fn rwu_dn(&self) -> RWU_DN_R {
        RWU_DN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB Bus Activity Interrupt Flag"]
    #[inline(always)]
    pub fn bact(&self) -> BACT_R {
        BACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB Bus Reset In Progress Interrupt Flag"]
    #[inline(always)]
    pub fn brst(&self) -> BRST_R {
        BRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB Suspend Interrupt Flag"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No VBUS Interrupt Flag"]
    #[inline(always)]
    pub fn no_vbus(&self) -> NO_VBUS_R {
        NO_VBUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VBUS Detect Interrupt Flag"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Bus Reset Completed Interrupt Flag"]
    #[inline(always)]
    pub fn brst_dn(&self) -> BRST_DN_R {
        BRST_DN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Setup Packet Interrupt Flag"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint IN Interrupt Flag"]
    #[inline(always)]
    pub fn ep_in(&self) -> EP_IN_R {
        EP_IN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint OUT Interrupt Flag"]
    #[inline(always)]
    pub fn ep_out(&self) -> EP_OUT_R {
        EP_OUT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint NAK Interrupt Flag"]
    #[inline(always)]
    pub fn ep_nak(&self) -> EP_NAK_R {
        EP_NAK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err(&self) -> DMA_ERR_R {
        DMA_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr(&self) -> BUF_OVR_R {
        BUF_OVR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DPLUS Activity Interrupt Flag"]
    #[inline(always)]
    pub fn dpact(&mut self) -> DPACT_W {
        DPACT_W::new(self)
    }
    #[doc = "Bit 1 - Remote Wakeup Done Interrupt Flag"]
    #[inline(always)]
    pub fn rwu_dn(&mut self) -> RWU_DN_W {
        RWU_DN_W::new(self)
    }
    #[doc = "Bit 2 - USB Bus Activity Interrupt Flag"]
    #[inline(always)]
    pub fn bact(&mut self) -> BACT_W {
        BACT_W::new(self)
    }
    #[doc = "Bit 3 - USB Bus Reset In Progress Interrupt Flag"]
    #[inline(always)]
    pub fn brst(&mut self) -> BRST_W {
        BRST_W::new(self)
    }
    #[doc = "Bit 4 - USB Suspend Interrupt Flag"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W::new(self)
    }
    #[doc = "Bit 5 - No VBUS Interrupt Flag"]
    #[inline(always)]
    pub fn no_vbus(&mut self) -> NO_VBUS_W {
        NO_VBUS_W::new(self)
    }
    #[doc = "Bit 6 - VBUS Detect Interrupt Flag"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VBUS_W {
        VBUS_W::new(self)
    }
    #[doc = "Bit 7 - USB Bus Reset Completed Interrupt Flag"]
    #[inline(always)]
    pub fn brst_dn(&mut self) -> BRST_DN_W {
        BRST_DN_W::new(self)
    }
    #[doc = "Bit 8 - Setup Packet Interrupt Flag"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W::new(self)
    }
    #[doc = "Bit 9 - Endpoint IN Interrupt Flag"]
    #[inline(always)]
    pub fn ep_in(&mut self) -> EP_IN_W {
        EP_IN_W::new(self)
    }
    #[doc = "Bit 10 - Endpoint OUT Interrupt Flag"]
    #[inline(always)]
    pub fn ep_out(&mut self) -> EP_OUT_W {
        EP_OUT_W::new(self)
    }
    #[doc = "Bit 11 - Endpoint NAK Interrupt Flag"]
    #[inline(always)]
    pub fn ep_nak(&mut self) -> EP_NAK_W {
        EP_NAK_W::new(self)
    }
    #[doc = "Bit 12 - DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err(&mut self) -> DMA_ERR_W {
        DMA_ERR_W::new(self)
    }
    #[doc = "Bit 13 - Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr(&mut self) -> BUF_OVR_W {
        BUF_OVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_inten](index.html) module"]
pub struct DEV_INTEN_SPEC;
impl crate::RegisterSpec for DEV_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_inten::R](R) reader structure"]
impl crate::Readable for DEV_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dev_inten::W](W) writer structure"]
impl crate::Writable for DEV_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEV_INTEN to value 0"]
impl crate::Resettable for DEV_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
