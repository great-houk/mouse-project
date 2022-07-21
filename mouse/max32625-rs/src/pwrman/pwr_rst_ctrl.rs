#[doc = "Register `PWR_RST_CTRL` reader"]
pub struct R(crate::R<PWR_RST_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_RST_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_RST_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_RST_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_RST_CTRL` writer"]
pub struct W(crate::W<PWR_RST_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_RST_CTRL_SPEC>;
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
impl From<crate::W<PWR_RST_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_RST_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `afe_powered` reader - AFE Powered"]
pub type AFE_POWERED_R = crate::BitReader<bool>;
#[doc = "Field `afe_powered` writer - AFE Powered"]
pub type AFE_POWERED_W<'a> = crate::BitWriter<'a, u32, PWR_RST_CTRL_SPEC, bool, 2>;
#[doc = "Field `io_active` reader - I/O Active"]
pub type IO_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `io_active` writer - I/O Active"]
pub type IO_ACTIVE_W<'a> = crate::BitWriter<'a, u32, PWR_RST_CTRL_SPEC, bool, 3>;
#[doc = "Field `usb_powered` reader - USB Powered"]
pub type USB_POWERED_R = crate::BitReader<bool>;
#[doc = "Field `usb_powered` writer - USB Powered"]
pub type USB_POWERED_W<'a> = crate::BitWriter<'a, u32, PWR_RST_CTRL_SPEC, bool, 4>;
#[doc = "Field `pullups_enabled` reader - Static Pullups Enabled"]
pub type PULLUPS_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `pullups_enabled` writer - Static Pullups Enabled"]
pub type PULLUPS_ENABLED_W<'a> = crate::BitWriter<'a, u32, PWR_RST_CTRL_SPEC, bool, 5>;
#[doc = "Field `firmware_reset` reader - Firmware Initiated Reset"]
pub type FIRMWARE_RESET_R = crate::BitReader<bool>;
#[doc = "Field `firmware_reset` writer - Firmware Initiated Reset"]
pub type FIRMWARE_RESET_W<'a> = crate::BitWriter<'a, u32, PWR_RST_CTRL_SPEC, bool, 8>;
#[doc = "Field `arm_lockup_reset` reader - ARM Lockup Reset"]
pub type ARM_LOCKUP_RESET_R = crate::BitReader<bool>;
#[doc = "Field `arm_lockup_reset` writer - ARM Lockup Reset"]
pub type ARM_LOCKUP_RESET_W<'a> = crate::BitWriter<'a, u32, PWR_RST_CTRL_SPEC, bool, 9>;
#[doc = "Field `tamper_detect` reader - Reset Caused By - Tamper Detect"]
pub type TAMPER_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `arm_lockup` reader - Reset Caused By - ARM Core Lockup"]
pub type ARM_LOCKUP_R = crate::BitReader<bool>;
#[doc = "Field `fw_command_arm` reader - Reset Caused By - Firmware Commanded Reset"]
pub type FW_COMMAND_ARM_R = crate::BitReader<bool>;
#[doc = "Field `watchdog_timeout` reader - Reset Caused By - Watchdog Timeout"]
pub type WATCHDOG_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `fw_command_sysman` reader - Reset Caused By - Firmware Initiated Reset from SysMan"]
pub type FW_COMMAND_SYSMAN_R = crate::BitReader<bool>;
#[doc = "Field `srstn_assertion` reader - Reset Caused By - External System Reset (SRSTn)"]
pub type SRSTN_ASSERTION_R = crate::BitReader<bool>;
#[doc = "Field `por` reader - Reset Caused By - Power On Reset (POR)"]
pub type POR_R = crate::BitReader<bool>;
#[doc = "Field `low_power_mode` reader - Power Manager Dynamic Clock Gating Enable"]
pub type LOW_POWER_MODE_R = crate::BitReader<bool>;
#[doc = "Field `low_power_mode` writer - Power Manager Dynamic Clock Gating Enable"]
pub type LOW_POWER_MODE_W<'a> = crate::BitWriter<'a, u32, PWR_RST_CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 2 - AFE Powered"]
    #[inline(always)]
    pub fn afe_powered(&self) -> AFE_POWERED_R {
        AFE_POWERED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O Active"]
    #[inline(always)]
    pub fn io_active(&self) -> IO_ACTIVE_R {
        IO_ACTIVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB Powered"]
    #[inline(always)]
    pub fn usb_powered(&self) -> USB_POWERED_R {
        USB_POWERED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Static Pullups Enabled"]
    #[inline(always)]
    pub fn pullups_enabled(&self) -> PULLUPS_ENABLED_R {
        PULLUPS_ENABLED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Firmware Initiated Reset"]
    #[inline(always)]
    pub fn firmware_reset(&self) -> FIRMWARE_RESET_R {
        FIRMWARE_RESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ARM Lockup Reset"]
    #[inline(always)]
    pub fn arm_lockup_reset(&self) -> ARM_LOCKUP_RESET_R {
        ARM_LOCKUP_RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset Caused By - Tamper Detect"]
    #[inline(always)]
    pub fn tamper_detect(&self) -> TAMPER_DETECT_R {
        TAMPER_DETECT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset Caused By - ARM Core Lockup"]
    #[inline(always)]
    pub fn arm_lockup(&self) -> ARM_LOCKUP_R {
        ARM_LOCKUP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Caused By - Firmware Commanded Reset"]
    #[inline(always)]
    pub fn fw_command_arm(&self) -> FW_COMMAND_ARM_R {
        FW_COMMAND_ARM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset Caused By - Watchdog Timeout"]
    #[inline(always)]
    pub fn watchdog_timeout(&self) -> WATCHDOG_TIMEOUT_R {
        WATCHDOG_TIMEOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset Caused By - Firmware Initiated Reset from SysMan"]
    #[inline(always)]
    pub fn fw_command_sysman(&self) -> FW_COMMAND_SYSMAN_R {
        FW_COMMAND_SYSMAN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset Caused By - External System Reset (SRSTn)"]
    #[inline(always)]
    pub fn srstn_assertion(&self) -> SRSTN_ASSERTION_R {
        SRSTN_ASSERTION_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset Caused By - Power On Reset (POR)"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Power Manager Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn low_power_mode(&self) -> LOW_POWER_MODE_R {
        LOW_POWER_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - AFE Powered"]
    #[inline(always)]
    pub fn afe_powered(&mut self) -> AFE_POWERED_W {
        AFE_POWERED_W::new(self)
    }
    #[doc = "Bit 3 - I/O Active"]
    #[inline(always)]
    pub fn io_active(&mut self) -> IO_ACTIVE_W {
        IO_ACTIVE_W::new(self)
    }
    #[doc = "Bit 4 - USB Powered"]
    #[inline(always)]
    pub fn usb_powered(&mut self) -> USB_POWERED_W {
        USB_POWERED_W::new(self)
    }
    #[doc = "Bit 5 - Static Pullups Enabled"]
    #[inline(always)]
    pub fn pullups_enabled(&mut self) -> PULLUPS_ENABLED_W {
        PULLUPS_ENABLED_W::new(self)
    }
    #[doc = "Bit 8 - Firmware Initiated Reset"]
    #[inline(always)]
    pub fn firmware_reset(&mut self) -> FIRMWARE_RESET_W {
        FIRMWARE_RESET_W::new(self)
    }
    #[doc = "Bit 9 - ARM Lockup Reset"]
    #[inline(always)]
    pub fn arm_lockup_reset(&mut self) -> ARM_LOCKUP_RESET_W {
        ARM_LOCKUP_RESET_W::new(self)
    }
    #[doc = "Bit 31 - Power Manager Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn low_power_mode(&mut self) -> LOW_POWER_MODE_W {
        LOW_POWER_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Reset Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_rst_ctrl](index.html) module"]
pub struct PWR_RST_CTRL_SPEC;
impl crate::RegisterSpec for PWR_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_rst_ctrl::R](R) reader structure"]
impl crate::Readable for PWR_RST_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_rst_ctrl::W](W) writer structure"]
impl crate::Writable for PWR_RST_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_RST_CTRL to value 0"]
impl crate::Resettable for PWR_RST_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
