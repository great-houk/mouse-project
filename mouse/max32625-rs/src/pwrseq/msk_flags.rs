#[doc = "Register `MSK_FLAGS` reader"]
pub struct R(crate::R<MSK_FLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSK_FLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSK_FLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSK_FLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSK_FLAGS` writer"]
pub struct W(crate::W<MSK_FLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSK_FLAGS_SPEC>;
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
impl From<crate::W<MSK_FLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSK_FLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_sys_reboot` reader - Mask for system reboot detect"]
pub type PWR_SYS_REBOOT_R = crate::BitReader<bool>;
#[doc = "Field `pwr_sys_reboot` writer - Mask for system reboot detect"]
pub type PWR_SYS_REBOOT_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 1>;
#[doc = "Field `pwr_power_fail` reader - Mask for previous power fail detect"]
pub type PWR_POWER_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `pwr_power_fail` writer - Mask for previous power fail detect"]
pub type PWR_POWER_FAIL_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 2>;
#[doc = "Field `pwr_boot_fail` reader - Mask for previous boot fail detect"]
pub type PWR_BOOT_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `pwr_boot_fail` writer - Mask for previous boot fail detect"]
pub type PWR_BOOT_FAIL_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 3>;
#[doc = "Field `pwr_flash_discharge` reader - Mask for flash discharge event"]
pub type PWR_FLASH_DISCHARGE_R = crate::BitReader<bool>;
#[doc = "Field `pwr_flash_discharge` writer - Mask for flash discharge event"]
pub type PWR_FLASH_DISCHARGE_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 4>;
#[doc = "Field `pwr_iowakeup` reader - Mask for GPIO wakeup event detect"]
pub type PWR_IOWAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `pwr_iowakeup` writer - Mask for GPIO wakeup event detect"]
pub type PWR_IOWAKEUP_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 5>;
#[doc = "Field `pwr_vdd12_rst_bad` reader - Mask for VDD12 rst event"]
pub type PWR_VDD12_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vdd12_rst_bad` writer - Mask for VDD12 rst event"]
pub type PWR_VDD12_RST_BAD_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 6>;
#[doc = "Field `pwr_vdd18_rst_bad` reader - Mask for VDD18 rst event"]
pub type PWR_VDD18_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vdd18_rst_bad` writer - Mask for VDD18 rst event"]
pub type PWR_VDD18_RST_BAD_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 7>;
#[doc = "Field `pwr_vrtc_rst_bad` reader - Mask for VRTC rst event"]
pub type PWR_VRTC_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vrtc_rst_bad` writer - Mask for VRTC rst event"]
pub type PWR_VRTC_RST_BAD_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 8>;
#[doc = "Field `pwr_vddb_rst_bad` reader - Mask for VDDB rst event"]
pub type PWR_VDDB_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vddb_rst_bad` writer - Mask for VDDB rst event"]
pub type PWR_VDDB_RST_BAD_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 9>;
#[doc = "Field `pwr_tvdd12_rst_bad` reader - Mask for TVDD12 rst event"]
pub type PWR_TVDD12_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_tvdd12_rst_bad` writer - Mask for TVDD12 rst event"]
pub type PWR_TVDD12_RST_BAD_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 10>;
#[doc = "Field `pwr_por18z_fail_latch` reader - Mask for POR18 powerfail event"]
pub type PWR_POR18Z_FAIL_LATCH_R = crate::BitReader<bool>;
#[doc = "Field `pwr_por18z_fail_latch` writer - Mask for POR18 powerfail event"]
pub type PWR_POR18Z_FAIL_LATCH_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 11>;
#[doc = "Field `rtc_cmpr0` reader - Mask for RTC compare 0 event"]
pub type RTC_CMPR0_R = crate::BitReader<bool>;
#[doc = "Field `rtc_cmpr0` writer - Mask for RTC compare 0 event"]
pub type RTC_CMPR0_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 12>;
#[doc = "Field `rtc_cmpr1` reader - Mask for RTC compare 1 event"]
pub type RTC_CMPR1_R = crate::BitReader<bool>;
#[doc = "Field `rtc_cmpr1` writer - Mask for RTC compare 1 event"]
pub type RTC_CMPR1_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 13>;
#[doc = "Field `rtc_prescale_cmp` reader - Mask for RTC prescale compare event"]
pub type RTC_PRESCALE_CMP_R = crate::BitReader<bool>;
#[doc = "Field `rtc_prescale_cmp` writer - Mask for RTC prescale compare event"]
pub type RTC_PRESCALE_CMP_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 14>;
#[doc = "Field `rtc_rollover` reader - Mask for RTC rollover event"]
pub type RTC_ROLLOVER_R = crate::BitReader<bool>;
#[doc = "Field `rtc_rollover` writer - Mask for RTC rollover event"]
pub type RTC_ROLLOVER_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 15>;
#[doc = "Field `pwr_usb_plug_wakeup` reader - Mask for USB plug connect event"]
pub type PWR_USB_PLUG_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `pwr_usb_plug_wakeup` writer - Mask for USB plug connect event"]
pub type PWR_USB_PLUG_WAKEUP_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 16>;
#[doc = "Field `pwr_usb_remove_wakeup` reader - Mask for USB plug disconnect event"]
pub type PWR_USB_REMOVE_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `pwr_usb_remove_wakeup` writer - Mask for USB plug disconnect event"]
pub type PWR_USB_REMOVE_WAKEUP_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 17>;
#[doc = "Field `pwr_tvdd12_bad` reader - Mask for TVDD12 power fail event"]
pub type PWR_TVDD12_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_tvdd12_bad` writer - Mask for TVDD12 power fail event"]
pub type PWR_TVDD12_BAD_W<'a> = crate::BitWriter<'a, u32, MSK_FLAGS_SPEC, bool, 18>;
#[doc = "Field `pwr_vddio_rst_bad` reader - Mask for VDDIO Comparator Tripped"]
pub type PWR_VDDIO_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vddio_rst_bad` writer - Mask for VDDIO Comparator Tripped"]
pub type PWR_VDDIO_RST_BAD_W<'a> = crate::BitWriter1C<'a, u32, MSK_FLAGS_SPEC, bool, 19>;
#[doc = "Field `pwr_vddioh_rst_bad` reader - Mask for VDDIOH Comparator Tripped"]
pub type PWR_VDDIOH_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vddioh_rst_bad` writer - Mask for VDDIOH Comparator Tripped"]
pub type PWR_VDDIOH_RST_BAD_W<'a> = crate::BitWriter1C<'a, u32, MSK_FLAGS_SPEC, bool, 20>;
#[doc = "Field `pwr_isoz_vddio_fail` reader - Mask for VDDIO Isolation POR Tripped"]
pub type PWR_ISOZ_VDDIO_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `pwr_isoz_vddio_fail` writer - Mask for VDDIO Isolation POR Tripped"]
pub type PWR_ISOZ_VDDIO_FAIL_W<'a> = crate::BitWriter1C<'a, u32, MSK_FLAGS_SPEC, bool, 21>;
#[doc = "Field `pwr_isoz_vddioh_fail` reader - Mask for VDDIOH Isolation POR Tripped"]
pub type PWR_ISOZ_VDDIOH_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `pwr_isoz_vddioh_fail` writer - Mask for VDDIOH Isolation POR Tripped"]
pub type PWR_ISOZ_VDDIOH_FAIL_W<'a> = crate::BitWriter1C<'a, u32, MSK_FLAGS_SPEC, bool, 22>;
#[doc = "Field `pwr_nanoring_wakeup_flag` reader - Mask for Wakeup Triggered by Nanoring Wakeup"]
pub type PWR_NANORING_WAKEUP_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `pwr_nanoring_wakeup_flag` writer - Mask for Wakeup Triggered by Nanoring Wakeup"]
pub type PWR_NANORING_WAKEUP_FLAG_W<'a> = crate::BitWriter1C<'a, u32, MSK_FLAGS_SPEC, bool, 23>;
#[doc = "Field `pwr_watchdog_rstn_flag` reader - Mask for Power Sequencer Reset Triggered by Watchdog Flag"]
pub type PWR_WATCHDOG_RSTN_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `pwr_watchdog_rstn_flag` writer - Mask for Power Sequencer Reset Triggered by Watchdog Flag"]
pub type PWR_WATCHDOG_RSTN_FLAG_W<'a> = crate::BitWriter1C<'a, u32, MSK_FLAGS_SPEC, bool, 24>;
impl R {
    #[doc = "Bit 1 - Mask for system reboot detect"]
    #[inline(always)]
    pub fn pwr_sys_reboot(&self) -> PWR_SYS_REBOOT_R {
        PWR_SYS_REBOOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask for previous power fail detect"]
    #[inline(always)]
    pub fn pwr_power_fail(&self) -> PWR_POWER_FAIL_R {
        PWR_POWER_FAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask for previous boot fail detect"]
    #[inline(always)]
    pub fn pwr_boot_fail(&self) -> PWR_BOOT_FAIL_R {
        PWR_BOOT_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask for flash discharge event"]
    #[inline(always)]
    pub fn pwr_flash_discharge(&self) -> PWR_FLASH_DISCHARGE_R {
        PWR_FLASH_DISCHARGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask for GPIO wakeup event detect"]
    #[inline(always)]
    pub fn pwr_iowakeup(&self) -> PWR_IOWAKEUP_R {
        PWR_IOWAKEUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask for VDD12 rst event"]
    #[inline(always)]
    pub fn pwr_vdd12_rst_bad(&self) -> PWR_VDD12_RST_BAD_R {
        PWR_VDD12_RST_BAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask for VDD18 rst event"]
    #[inline(always)]
    pub fn pwr_vdd18_rst_bad(&self) -> PWR_VDD18_RST_BAD_R {
        PWR_VDD18_RST_BAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask for VRTC rst event"]
    #[inline(always)]
    pub fn pwr_vrtc_rst_bad(&self) -> PWR_VRTC_RST_BAD_R {
        PWR_VRTC_RST_BAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask for VDDB rst event"]
    #[inline(always)]
    pub fn pwr_vddb_rst_bad(&self) -> PWR_VDDB_RST_BAD_R {
        PWR_VDDB_RST_BAD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask for TVDD12 rst event"]
    #[inline(always)]
    pub fn pwr_tvdd12_rst_bad(&self) -> PWR_TVDD12_RST_BAD_R {
        PWR_TVDD12_RST_BAD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask for POR18 powerfail event"]
    #[inline(always)]
    pub fn pwr_por18z_fail_latch(&self) -> PWR_POR18Z_FAIL_LATCH_R {
        PWR_POR18Z_FAIL_LATCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mask for RTC compare 0 event"]
    #[inline(always)]
    pub fn rtc_cmpr0(&self) -> RTC_CMPR0_R {
        RTC_CMPR0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mask for RTC compare 1 event"]
    #[inline(always)]
    pub fn rtc_cmpr1(&self) -> RTC_CMPR1_R {
        RTC_CMPR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mask for RTC prescale compare event"]
    #[inline(always)]
    pub fn rtc_prescale_cmp(&self) -> RTC_PRESCALE_CMP_R {
        RTC_PRESCALE_CMP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mask for RTC rollover event"]
    #[inline(always)]
    pub fn rtc_rollover(&self) -> RTC_ROLLOVER_R {
        RTC_ROLLOVER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask for USB plug connect event"]
    #[inline(always)]
    pub fn pwr_usb_plug_wakeup(&self) -> PWR_USB_PLUG_WAKEUP_R {
        PWR_USB_PLUG_WAKEUP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Mask for USB plug disconnect event"]
    #[inline(always)]
    pub fn pwr_usb_remove_wakeup(&self) -> PWR_USB_REMOVE_WAKEUP_R {
        PWR_USB_REMOVE_WAKEUP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mask for TVDD12 power fail event"]
    #[inline(always)]
    pub fn pwr_tvdd12_bad(&self) -> PWR_TVDD12_BAD_R {
        PWR_TVDD12_BAD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Mask for VDDIO Comparator Tripped"]
    #[inline(always)]
    pub fn pwr_vddio_rst_bad(&self) -> PWR_VDDIO_RST_BAD_R {
        PWR_VDDIO_RST_BAD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Mask for VDDIOH Comparator Tripped"]
    #[inline(always)]
    pub fn pwr_vddioh_rst_bad(&self) -> PWR_VDDIOH_RST_BAD_R {
        PWR_VDDIOH_RST_BAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Mask for VDDIO Isolation POR Tripped"]
    #[inline(always)]
    pub fn pwr_isoz_vddio_fail(&self) -> PWR_ISOZ_VDDIO_FAIL_R {
        PWR_ISOZ_VDDIO_FAIL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Mask for VDDIOH Isolation POR Tripped"]
    #[inline(always)]
    pub fn pwr_isoz_vddioh_fail(&self) -> PWR_ISOZ_VDDIOH_FAIL_R {
        PWR_ISOZ_VDDIOH_FAIL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Mask for Wakeup Triggered by Nanoring Wakeup"]
    #[inline(always)]
    pub fn pwr_nanoring_wakeup_flag(&self) -> PWR_NANORING_WAKEUP_FLAG_R {
        PWR_NANORING_WAKEUP_FLAG_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask for Power Sequencer Reset Triggered by Watchdog Flag"]
    #[inline(always)]
    pub fn pwr_watchdog_rstn_flag(&self) -> PWR_WATCHDOG_RSTN_FLAG_R {
        PWR_WATCHDOG_RSTN_FLAG_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mask for system reboot detect"]
    #[inline(always)]
    pub fn pwr_sys_reboot(&mut self) -> PWR_SYS_REBOOT_W {
        PWR_SYS_REBOOT_W::new(self)
    }
    #[doc = "Bit 2 - Mask for previous power fail detect"]
    #[inline(always)]
    pub fn pwr_power_fail(&mut self) -> PWR_POWER_FAIL_W {
        PWR_POWER_FAIL_W::new(self)
    }
    #[doc = "Bit 3 - Mask for previous boot fail detect"]
    #[inline(always)]
    pub fn pwr_boot_fail(&mut self) -> PWR_BOOT_FAIL_W {
        PWR_BOOT_FAIL_W::new(self)
    }
    #[doc = "Bit 4 - Mask for flash discharge event"]
    #[inline(always)]
    pub fn pwr_flash_discharge(&mut self) -> PWR_FLASH_DISCHARGE_W {
        PWR_FLASH_DISCHARGE_W::new(self)
    }
    #[doc = "Bit 5 - Mask for GPIO wakeup event detect"]
    #[inline(always)]
    pub fn pwr_iowakeup(&mut self) -> PWR_IOWAKEUP_W {
        PWR_IOWAKEUP_W::new(self)
    }
    #[doc = "Bit 6 - Mask for VDD12 rst event"]
    #[inline(always)]
    pub fn pwr_vdd12_rst_bad(&mut self) -> PWR_VDD12_RST_BAD_W {
        PWR_VDD12_RST_BAD_W::new(self)
    }
    #[doc = "Bit 7 - Mask for VDD18 rst event"]
    #[inline(always)]
    pub fn pwr_vdd18_rst_bad(&mut self) -> PWR_VDD18_RST_BAD_W {
        PWR_VDD18_RST_BAD_W::new(self)
    }
    #[doc = "Bit 8 - Mask for VRTC rst event"]
    #[inline(always)]
    pub fn pwr_vrtc_rst_bad(&mut self) -> PWR_VRTC_RST_BAD_W {
        PWR_VRTC_RST_BAD_W::new(self)
    }
    #[doc = "Bit 9 - Mask for VDDB rst event"]
    #[inline(always)]
    pub fn pwr_vddb_rst_bad(&mut self) -> PWR_VDDB_RST_BAD_W {
        PWR_VDDB_RST_BAD_W::new(self)
    }
    #[doc = "Bit 10 - Mask for TVDD12 rst event"]
    #[inline(always)]
    pub fn pwr_tvdd12_rst_bad(&mut self) -> PWR_TVDD12_RST_BAD_W {
        PWR_TVDD12_RST_BAD_W::new(self)
    }
    #[doc = "Bit 11 - Mask for POR18 powerfail event"]
    #[inline(always)]
    pub fn pwr_por18z_fail_latch(&mut self) -> PWR_POR18Z_FAIL_LATCH_W {
        PWR_POR18Z_FAIL_LATCH_W::new(self)
    }
    #[doc = "Bit 12 - Mask for RTC compare 0 event"]
    #[inline(always)]
    pub fn rtc_cmpr0(&mut self) -> RTC_CMPR0_W {
        RTC_CMPR0_W::new(self)
    }
    #[doc = "Bit 13 - Mask for RTC compare 1 event"]
    #[inline(always)]
    pub fn rtc_cmpr1(&mut self) -> RTC_CMPR1_W {
        RTC_CMPR1_W::new(self)
    }
    #[doc = "Bit 14 - Mask for RTC prescale compare event"]
    #[inline(always)]
    pub fn rtc_prescale_cmp(&mut self) -> RTC_PRESCALE_CMP_W {
        RTC_PRESCALE_CMP_W::new(self)
    }
    #[doc = "Bit 15 - Mask for RTC rollover event"]
    #[inline(always)]
    pub fn rtc_rollover(&mut self) -> RTC_ROLLOVER_W {
        RTC_ROLLOVER_W::new(self)
    }
    #[doc = "Bit 16 - Mask for USB plug connect event"]
    #[inline(always)]
    pub fn pwr_usb_plug_wakeup(&mut self) -> PWR_USB_PLUG_WAKEUP_W {
        PWR_USB_PLUG_WAKEUP_W::new(self)
    }
    #[doc = "Bit 17 - Mask for USB plug disconnect event"]
    #[inline(always)]
    pub fn pwr_usb_remove_wakeup(&mut self) -> PWR_USB_REMOVE_WAKEUP_W {
        PWR_USB_REMOVE_WAKEUP_W::new(self)
    }
    #[doc = "Bit 18 - Mask for TVDD12 power fail event"]
    #[inline(always)]
    pub fn pwr_tvdd12_bad(&mut self) -> PWR_TVDD12_BAD_W {
        PWR_TVDD12_BAD_W::new(self)
    }
    #[doc = "Bit 19 - Mask for VDDIO Comparator Tripped"]
    #[inline(always)]
    pub fn pwr_vddio_rst_bad(&mut self) -> PWR_VDDIO_RST_BAD_W {
        PWR_VDDIO_RST_BAD_W::new(self)
    }
    #[doc = "Bit 20 - Mask for VDDIOH Comparator Tripped"]
    #[inline(always)]
    pub fn pwr_vddioh_rst_bad(&mut self) -> PWR_VDDIOH_RST_BAD_W {
        PWR_VDDIOH_RST_BAD_W::new(self)
    }
    #[doc = "Bit 21 - Mask for VDDIO Isolation POR Tripped"]
    #[inline(always)]
    pub fn pwr_isoz_vddio_fail(&mut self) -> PWR_ISOZ_VDDIO_FAIL_W {
        PWR_ISOZ_VDDIO_FAIL_W::new(self)
    }
    #[doc = "Bit 22 - Mask for VDDIOH Isolation POR Tripped"]
    #[inline(always)]
    pub fn pwr_isoz_vddioh_fail(&mut self) -> PWR_ISOZ_VDDIOH_FAIL_W {
        PWR_ISOZ_VDDIOH_FAIL_W::new(self)
    }
    #[doc = "Bit 23 - Mask for Wakeup Triggered by Nanoring Wakeup"]
    #[inline(always)]
    pub fn pwr_nanoring_wakeup_flag(&mut self) -> PWR_NANORING_WAKEUP_FLAG_W {
        PWR_NANORING_WAKEUP_FLAG_W::new(self)
    }
    #[doc = "Bit 24 - Mask for Power Sequencer Reset Triggered by Watchdog Flag"]
    #[inline(always)]
    pub fn pwr_watchdog_rstn_flag(&mut self) -> PWR_WATCHDOG_RSTN_FLAG_W {
        PWR_WATCHDOG_RSTN_FLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Flags Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msk_flags](index.html) module"]
pub struct MSK_FLAGS_SPEC;
impl crate::RegisterSpec for MSK_FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msk_flags::R](R) reader structure"]
impl crate::Readable for MSK_FLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msk_flags::W](W) writer structure"]
impl crate::Writable for MSK_FLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSK_FLAGS to value 0"]
impl crate::Resettable for MSK_FLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
