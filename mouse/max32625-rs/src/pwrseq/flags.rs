#[doc = "Register `FLAGS` reader"]
pub struct R(crate::R<FLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLAGS` writer"]
pub struct W(crate::W<FLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLAGS_SPEC>;
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
impl From<crate::W<FLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_first_boot` reader - Initial Boot event detected flag"]
pub type PWR_FIRST_BOOT_R = crate::BitReader<bool>;
#[doc = "Field `pwr_sys_reboot` reader - Firmware Reset event detected flag"]
pub type PWR_SYS_REBOOT_R = crate::BitReader<bool>;
#[doc = "Field `pwr_power_fail` reader - Power Fail event detected flag"]
pub type PWR_POWER_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `pwr_power_fail` writer - Power Fail event detected flag"]
pub type PWR_POWER_FAIL_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 2>;
#[doc = "Field `pwr_boot_fail` reader - Boot Fail event detected flag"]
pub type PWR_BOOT_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `pwr_boot_fail` writer - Boot Fail event detected flag"]
pub type PWR_BOOT_FAIL_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 3>;
#[doc = "Field `pwr_flash_discharge` reader - Flash Discharged During Powerfail event detected flag"]
pub type PWR_FLASH_DISCHARGE_R = crate::BitReader<bool>;
#[doc = "Field `pwr_flash_discharge` writer - Flash Discharged During Powerfail event detected flag"]
pub type PWR_FLASH_DISCHARGE_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 4>;
#[doc = "Field `pwr_iowakeup` reader - GPIO Wakeup event detected flag"]
pub type PWR_IOWAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `pwr_iowakeup` writer - GPIO Wakeup event detected flag"]
pub type PWR_IOWAKEUP_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 5>;
#[doc = "Field `pwr_vdd12_rst_bad` reader - VDD12_SW Comparator Tripped event detected flag"]
pub type PWR_VDD12_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vdd12_rst_bad` writer - VDD12_SW Comparator Tripped event detected flag"]
pub type PWR_VDD12_RST_BAD_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 6>;
#[doc = "Field `pwr_vdd18_rst_bad` reader - VDD18_SW Comparator Tripped event detected flag"]
pub type PWR_VDD18_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vdd18_rst_bad` writer - VDD18_SW Comparator Tripped event detected flag"]
pub type PWR_VDD18_RST_BAD_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 7>;
#[doc = "Field `pwr_vrtc_rst_bad` reader - VRTC Comparator Tripped event detected flag"]
pub type PWR_VRTC_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vrtc_rst_bad` writer - VRTC Comparator Tripped event detected flag"]
pub type PWR_VRTC_RST_BAD_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 8>;
#[doc = "Field `pwr_vddb_rst_bad` reader - VDDB Comparator Tripped event detected flag"]
pub type PWR_VDDB_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vddb_rst_bad` writer - VDDB Comparator Tripped event detected flag"]
pub type PWR_VDDB_RST_BAD_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 9>;
#[doc = "Field `pwr_tvdd12_rst_bad` reader - TVDD12 Comparator Tripped event detected flag"]
pub type PWR_TVDD12_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_tvdd12_rst_bad` writer - TVDD12 Comparator Tripped event detected flag"]
pub type PWR_TVDD12_RST_BAD_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 10>;
#[doc = "Field `pwr_por18z_fail_latch` reader - POR18 and POR18_bg have been tripped"]
pub type PWR_POR18Z_FAIL_LATCH_R = crate::BitReader<bool>;
#[doc = "Field `pwr_por18z_fail_latch` writer - POR18 and POR18_bg have been tripped"]
pub type PWR_POR18Z_FAIL_LATCH_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 11>;
#[doc = "Field `rtc_cmpr0` reader - RTC Comparator 0 Match event detected flag"]
pub type RTC_CMPR0_R = crate::BitReader<bool>;
#[doc = "Field `rtc_cmpr1` reader - RTC Comparator 1 Match event detected flag"]
pub type RTC_CMPR1_R = crate::BitReader<bool>;
#[doc = "Field `rtc_prescale_cmp` reader - RTC Prescale Comparator Match event detected flag"]
pub type RTC_PRESCALE_CMP_R = crate::BitReader<bool>;
#[doc = "Field `rtc_rollover` reader - RTC Rollover event detected flag"]
pub type RTC_ROLLOVER_R = crate::BitReader<bool>;
#[doc = "Field `pwr_usb_plug_wakeup` reader - USB Power Connect Wakeup event detected flag"]
pub type PWR_USB_PLUG_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `pwr_usb_plug_wakeup` writer - USB Power Connect Wakeup event detected flag"]
pub type PWR_USB_PLUG_WAKEUP_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 16>;
#[doc = "Field `pwr_usb_remove_wakeup` reader - USB Power Remove Wakeup event detected flag"]
pub type PWR_USB_REMOVE_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `pwr_usb_remove_wakeup` writer - USB Power Remove Wakeup event detected flag"]
pub type PWR_USB_REMOVE_WAKEUP_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 17>;
#[doc = "Field `pwr_tvdd12_bad` reader - Retention Regulator POR Tripped event detected flag"]
pub type PWR_TVDD12_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_tvdd12_bad` writer - Retention Regulator POR Tripped event detected flag"]
pub type PWR_TVDD12_BAD_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 18>;
#[doc = "Field `pwr_vddio_rst_bad` reader - VDDIO Comparator Tripped"]
pub type PWR_VDDIO_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vddio_rst_bad` writer - VDDIO Comparator Tripped"]
pub type PWR_VDDIO_RST_BAD_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 19>;
#[doc = "Field `pwr_vddioh_rst_bad` reader - VDDIOH Comparator Tripped"]
pub type PWR_VDDIOH_RST_BAD_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vddioh_rst_bad` writer - VDDIOH Comparator Tripped"]
pub type PWR_VDDIOH_RST_BAD_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 20>;
#[doc = "Field `pwr_isoz_vddio_fail` reader - VDDIO Isolation POR Tripped"]
pub type PWR_ISOZ_VDDIO_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `pwr_isoz_vddio_fail` writer - VDDIO Isolation POR Tripped"]
pub type PWR_ISOZ_VDDIO_FAIL_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 21>;
#[doc = "Field `pwr_isoz_vddioh_fail` reader - VDDIOH Isolation POR Tripped"]
pub type PWR_ISOZ_VDDIOH_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `pwr_isoz_vddioh_fail` writer - VDDIOH Isolation POR Tripped"]
pub type PWR_ISOZ_VDDIOH_FAIL_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 22>;
#[doc = "Field `pwr_nanoring_wakeup_flag` reader - Wakeup Triggered by Nanoring Wakeup"]
pub type PWR_NANORING_WAKEUP_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `pwr_nanoring_wakeup_flag` writer - Wakeup Triggered by Nanoring Wakeup"]
pub type PWR_NANORING_WAKEUP_FLAG_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 23>;
#[doc = "Field `pwr_watchdog_rstn_flag` reader - Power Sequencer Reset Triggered by Watchdog Flag"]
pub type PWR_WATCHDOG_RSTN_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `pwr_watchdog_rstn_flag` writer - Power Sequencer Reset Triggered by Watchdog Flag"]
pub type PWR_WATCHDOG_RSTN_FLAG_W<'a> = crate::BitWriter1C<'a, u32, FLAGS_SPEC, bool, 24>;
impl R {
    #[doc = "Bit 0 - Initial Boot event detected flag"]
    #[inline(always)]
    pub fn pwr_first_boot(&self) -> PWR_FIRST_BOOT_R {
        PWR_FIRST_BOOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Firmware Reset event detected flag"]
    #[inline(always)]
    pub fn pwr_sys_reboot(&self) -> PWR_SYS_REBOOT_R {
        PWR_SYS_REBOOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Fail event detected flag"]
    #[inline(always)]
    pub fn pwr_power_fail(&self) -> PWR_POWER_FAIL_R {
        PWR_POWER_FAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Boot Fail event detected flag"]
    #[inline(always)]
    pub fn pwr_boot_fail(&self) -> PWR_BOOT_FAIL_R {
        PWR_BOOT_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Discharged During Powerfail event detected flag"]
    #[inline(always)]
    pub fn pwr_flash_discharge(&self) -> PWR_FLASH_DISCHARGE_R {
        PWR_FLASH_DISCHARGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_iowakeup(&self) -> PWR_IOWAKEUP_R {
        PWR_IOWAKEUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VDD12_SW Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vdd12_rst_bad(&self) -> PWR_VDD12_RST_BAD_R {
        PWR_VDD12_RST_BAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VDD18_SW Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vdd18_rst_bad(&self) -> PWR_VDD18_RST_BAD_R {
        PWR_VDD18_RST_BAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VRTC Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vrtc_rst_bad(&self) -> PWR_VRTC_RST_BAD_R {
        PWR_VRTC_RST_BAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDB Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vddb_rst_bad(&self) -> PWR_VDDB_RST_BAD_R {
        PWR_VDDB_RST_BAD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TVDD12 Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_tvdd12_rst_bad(&self) -> PWR_TVDD12_RST_BAD_R {
        PWR_TVDD12_RST_BAD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - POR18 and POR18_bg have been tripped"]
    #[inline(always)]
    pub fn pwr_por18z_fail_latch(&self) -> PWR_POR18Z_FAIL_LATCH_R {
        PWR_POR18Z_FAIL_LATCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RTC Comparator 0 Match event detected flag"]
    #[inline(always)]
    pub fn rtc_cmpr0(&self) -> RTC_CMPR0_R {
        RTC_CMPR0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RTC Comparator 1 Match event detected flag"]
    #[inline(always)]
    pub fn rtc_cmpr1(&self) -> RTC_CMPR1_R {
        RTC_CMPR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC Prescale Comparator Match event detected flag"]
    #[inline(always)]
    pub fn rtc_prescale_cmp(&self) -> RTC_PRESCALE_CMP_R {
        RTC_PRESCALE_CMP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC Rollover event detected flag"]
    #[inline(always)]
    pub fn rtc_rollover(&self) -> RTC_ROLLOVER_R {
        RTC_ROLLOVER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - USB Power Connect Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_usb_plug_wakeup(&self) -> PWR_USB_PLUG_WAKEUP_R {
        PWR_USB_PLUG_WAKEUP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USB Power Remove Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_usb_remove_wakeup(&self) -> PWR_USB_REMOVE_WAKEUP_R {
        PWR_USB_REMOVE_WAKEUP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Retention Regulator POR Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_tvdd12_bad(&self) -> PWR_TVDD12_BAD_R {
        PWR_TVDD12_BAD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - VDDIO Comparator Tripped"]
    #[inline(always)]
    pub fn pwr_vddio_rst_bad(&self) -> PWR_VDDIO_RST_BAD_R {
        PWR_VDDIO_RST_BAD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VDDIOH Comparator Tripped"]
    #[inline(always)]
    pub fn pwr_vddioh_rst_bad(&self) -> PWR_VDDIOH_RST_BAD_R {
        PWR_VDDIOH_RST_BAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VDDIO Isolation POR Tripped"]
    #[inline(always)]
    pub fn pwr_isoz_vddio_fail(&self) -> PWR_ISOZ_VDDIO_FAIL_R {
        PWR_ISOZ_VDDIO_FAIL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - VDDIOH Isolation POR Tripped"]
    #[inline(always)]
    pub fn pwr_isoz_vddioh_fail(&self) -> PWR_ISOZ_VDDIOH_FAIL_R {
        PWR_ISOZ_VDDIOH_FAIL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Wakeup Triggered by Nanoring Wakeup"]
    #[inline(always)]
    pub fn pwr_nanoring_wakeup_flag(&self) -> PWR_NANORING_WAKEUP_FLAG_R {
        PWR_NANORING_WAKEUP_FLAG_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Power Sequencer Reset Triggered by Watchdog Flag"]
    #[inline(always)]
    pub fn pwr_watchdog_rstn_flag(&self) -> PWR_WATCHDOG_RSTN_FLAG_R {
        PWR_WATCHDOG_RSTN_FLAG_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Power Fail event detected flag"]
    #[inline(always)]
    pub fn pwr_power_fail(&mut self) -> PWR_POWER_FAIL_W {
        PWR_POWER_FAIL_W::new(self)
    }
    #[doc = "Bit 3 - Boot Fail event detected flag"]
    #[inline(always)]
    pub fn pwr_boot_fail(&mut self) -> PWR_BOOT_FAIL_W {
        PWR_BOOT_FAIL_W::new(self)
    }
    #[doc = "Bit 4 - Flash Discharged During Powerfail event detected flag"]
    #[inline(always)]
    pub fn pwr_flash_discharge(&mut self) -> PWR_FLASH_DISCHARGE_W {
        PWR_FLASH_DISCHARGE_W::new(self)
    }
    #[doc = "Bit 5 - GPIO Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_iowakeup(&mut self) -> PWR_IOWAKEUP_W {
        PWR_IOWAKEUP_W::new(self)
    }
    #[doc = "Bit 6 - VDD12_SW Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vdd12_rst_bad(&mut self) -> PWR_VDD12_RST_BAD_W {
        PWR_VDD12_RST_BAD_W::new(self)
    }
    #[doc = "Bit 7 - VDD18_SW Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vdd18_rst_bad(&mut self) -> PWR_VDD18_RST_BAD_W {
        PWR_VDD18_RST_BAD_W::new(self)
    }
    #[doc = "Bit 8 - VRTC Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vrtc_rst_bad(&mut self) -> PWR_VRTC_RST_BAD_W {
        PWR_VRTC_RST_BAD_W::new(self)
    }
    #[doc = "Bit 9 - VDDB Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vddb_rst_bad(&mut self) -> PWR_VDDB_RST_BAD_W {
        PWR_VDDB_RST_BAD_W::new(self)
    }
    #[doc = "Bit 10 - TVDD12 Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_tvdd12_rst_bad(&mut self) -> PWR_TVDD12_RST_BAD_W {
        PWR_TVDD12_RST_BAD_W::new(self)
    }
    #[doc = "Bit 11 - POR18 and POR18_bg have been tripped"]
    #[inline(always)]
    pub fn pwr_por18z_fail_latch(&mut self) -> PWR_POR18Z_FAIL_LATCH_W {
        PWR_POR18Z_FAIL_LATCH_W::new(self)
    }
    #[doc = "Bit 16 - USB Power Connect Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_usb_plug_wakeup(&mut self) -> PWR_USB_PLUG_WAKEUP_W {
        PWR_USB_PLUG_WAKEUP_W::new(self)
    }
    #[doc = "Bit 17 - USB Power Remove Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_usb_remove_wakeup(&mut self) -> PWR_USB_REMOVE_WAKEUP_W {
        PWR_USB_REMOVE_WAKEUP_W::new(self)
    }
    #[doc = "Bit 18 - Retention Regulator POR Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_tvdd12_bad(&mut self) -> PWR_TVDD12_BAD_W {
        PWR_TVDD12_BAD_W::new(self)
    }
    #[doc = "Bit 19 - VDDIO Comparator Tripped"]
    #[inline(always)]
    pub fn pwr_vddio_rst_bad(&mut self) -> PWR_VDDIO_RST_BAD_W {
        PWR_VDDIO_RST_BAD_W::new(self)
    }
    #[doc = "Bit 20 - VDDIOH Comparator Tripped"]
    #[inline(always)]
    pub fn pwr_vddioh_rst_bad(&mut self) -> PWR_VDDIOH_RST_BAD_W {
        PWR_VDDIOH_RST_BAD_W::new(self)
    }
    #[doc = "Bit 21 - VDDIO Isolation POR Tripped"]
    #[inline(always)]
    pub fn pwr_isoz_vddio_fail(&mut self) -> PWR_ISOZ_VDDIO_FAIL_W {
        PWR_ISOZ_VDDIO_FAIL_W::new(self)
    }
    #[doc = "Bit 22 - VDDIOH Isolation POR Tripped"]
    #[inline(always)]
    pub fn pwr_isoz_vddioh_fail(&mut self) -> PWR_ISOZ_VDDIOH_FAIL_W {
        PWR_ISOZ_VDDIOH_FAIL_W::new(self)
    }
    #[doc = "Bit 23 - Wakeup Triggered by Nanoring Wakeup"]
    #[inline(always)]
    pub fn pwr_nanoring_wakeup_flag(&mut self) -> PWR_NANORING_WAKEUP_FLAG_W {
        PWR_NANORING_WAKEUP_FLAG_W::new(self)
    }
    #[doc = "Bit 24 - Power Sequencer Reset Triggered by Watchdog Flag"]
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
#[doc = "Power Sequencer Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flags](index.html) module"]
pub struct FLAGS_SPEC;
impl crate::RegisterSpec for FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flags::R](R) reader structure"]
impl crate::Readable for FLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flags::W](W) writer structure"]
impl crate::Writable for FLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLAGS to value 0"]
impl crate::Resettable for FLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
