#[doc = "Register `REG1` reader"]
pub struct R(crate::R<REG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG1` writer"]
pub struct W(crate::W<REG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG1_SPEC>;
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
impl From<crate::W<REG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_clr_io_event_latch` reader - Clear all GPIO Event Seen Latches"]
pub type PWR_CLR_IO_EVENT_LATCH_R = crate::BitReader<bool>;
#[doc = "Field `pwr_clr_io_event_latch` writer - Clear all GPIO Event Seen Latches"]
pub type PWR_CLR_IO_EVENT_LATCH_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 0>;
#[doc = "Field `pwr_clr_io_cfg_latch` reader - Clear all GPIO Configuration Latches"]
pub type PWR_CLR_IO_CFG_LATCH_R = crate::BitReader<bool>;
#[doc = "Field `pwr_clr_io_cfg_latch` writer - Clear all GPIO Configuration Latches"]
pub type PWR_CLR_IO_CFG_LATCH_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 1>;
#[doc = "Field `pwr_mbus_gate` reader - Freeze GPIO MBus State"]
pub type PWR_MBUS_GATE_R = crate::BitReader<bool>;
#[doc = "Field `pwr_mbus_gate` writer - Freeze GPIO MBus State"]
pub type PWR_MBUS_GATE_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 2>;
#[doc = "Field `pwr_discharge_en` reader - Enable Flash Discharge During Powerfail Event"]
pub type PWR_DISCHARGE_EN_R = crate::BitReader<bool>;
#[doc = "Field `pwr_discharge_en` writer - Enable Flash Discharge During Powerfail Event"]
pub type PWR_DISCHARGE_EN_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 3>;
#[doc = "Field `pwr_tvdd12_well` reader - TVDD12 Well Switch"]
pub type PWR_TVDD12_WELL_R = crate::BitReader<bool>;
#[doc = "Field `pwr_tvdd12_well` writer - TVDD12 Well Switch"]
pub type PWR_TVDD12_WELL_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 4>;
#[doc = "Field `pwr_sram_nwell_sw` reader - SRAM n-Well Switch"]
pub type PWR_SRAM_NWELL_SW_R = crate::BitReader<bool>;
#[doc = "Field `pwr_sram_nwell_sw` writer - SRAM n-Well Switch"]
pub type PWR_SRAM_NWELL_SW_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 5>;
#[doc = "Field `pwr_auto_mbus_gate` reader - Automatically Set/Clear GPIO MBus Gate"]
pub type PWR_AUTO_MBUS_GATE_R = crate::BitReader<bool>;
#[doc = "Field `pwr_auto_mbus_gate` writer - Automatically Set/Clear GPIO MBus Gate"]
pub type PWR_AUTO_MBUS_GATE_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 6>;
#[doc = "Field `pwr_svm_vddio_en_run` reader - Enable VDDIO SVM Run for LP2 and LP3"]
pub type PWR_SVM_VDDIO_EN_RUN_R = crate::BitReader<bool>;
#[doc = "Field `pwr_svm_vddio_en_run` writer - Enable VDDIO SVM Run for LP2 and LP3"]
pub type PWR_SVM_VDDIO_EN_RUN_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 8>;
#[doc = "Field `pwr_svm_vddioh_en_run` reader - Enable VDDIOH SVM Run for LP2 and LP3"]
pub type PWR_SVM_VDDIOH_EN_RUN_R = crate::BitReader<bool>;
#[doc = "Field `pwr_svm_vddioh_en_run` writer - Enable VDDIOH SVM Run for LP2 and LP3"]
pub type PWR_SVM_VDDIOH_EN_RUN_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 10>;
#[doc = "Field `pwr_retreg_src_v12` reader - Enable VDDIO SVM Run for LP2 and LP3"]
pub type PWR_RETREG_SRC_V12_R = crate::BitReader<bool>;
#[doc = "Field `pwr_retreg_src_v12` writer - Enable VDDIO SVM Run for LP2 and LP3"]
pub type PWR_RETREG_SRC_V12_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 12>;
#[doc = "Field `pwr_retreg_src_vrtc` reader - Enables VRTC Rail as Retention Power Su"]
pub type PWR_RETREG_SRC_VRTC_R = crate::BitReader<bool>;
#[doc = "Field `pwr_retreg_src_vrtc` writer - Enables VRTC Rail as Retention Power Su"]
pub type PWR_RETREG_SRC_VRTC_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 13>;
#[doc = "Field `pwr_retreg_src_v18` reader - Enables VDD18 Rail as Retention Power Supply"]
pub type PWR_RETREG_SRC_V18_R = crate::BitReader<bool>;
#[doc = "Field `pwr_retreg_src_v18` writer - Enables VDD18 Rail as Retention Power Supply"]
pub type PWR_RETREG_SRC_V18_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 14>;
#[doc = "Field `pwr_vddio_en_iso_por` reader - Enable VDDIO Isolation POR"]
pub type PWR_VDDIO_EN_ISO_POR_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vddio_en_iso_por` writer - Enable VDDIO Isolation POR"]
pub type PWR_VDDIO_EN_ISO_POR_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 16>;
#[doc = "Field `pwr_vddioh_en_iso_por` reader - Enable VDDIOH Isolation POR"]
pub type PWR_VDDIOH_EN_ISO_POR_R = crate::BitReader<bool>;
#[doc = "Field `pwr_vddioh_en_iso_por` writer - Enable VDDIOH Isolation POR"]
pub type PWR_VDDIOH_EN_ISO_POR_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 17>;
#[doc = "Field `pwr_lp0_core_resume_en` reader - Enable Core Resume to Toggle during LP0"]
pub type PWR_LP0_CORE_RESUME_EN_R = crate::BitReader<bool>;
#[doc = "Field `pwr_lp0_core_resume_en` writer - Enable Core Resume to Toggle during LP0"]
pub type PWR_LP0_CORE_RESUME_EN_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 18>;
#[doc = "Field `pwr_lp1_core_rstn_en` reader - Enable Core RSTN to Toggle during LP1"]
pub type PWR_LP1_CORE_RSTN_EN_R = crate::BitReader<bool>;
#[doc = "Field `pwr_lp1_core_rstn_en` writer - Enable Core RSTN to Toggle during LP1"]
pub type PWR_LP1_CORE_RSTN_EN_W<'a> = crate::BitWriter<'a, u32, REG1_SPEC, bool, 19>;
impl R {
    #[doc = "Bit 0 - Clear all GPIO Event Seen Latches"]
    #[inline(always)]
    pub fn pwr_clr_io_event_latch(&self) -> PWR_CLR_IO_EVENT_LATCH_R {
        PWR_CLR_IO_EVENT_LATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear all GPIO Configuration Latches"]
    #[inline(always)]
    pub fn pwr_clr_io_cfg_latch(&self) -> PWR_CLR_IO_CFG_LATCH_R {
        PWR_CLR_IO_CFG_LATCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Freeze GPIO MBus State"]
    #[inline(always)]
    pub fn pwr_mbus_gate(&self) -> PWR_MBUS_GATE_R {
        PWR_MBUS_GATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Flash Discharge During Powerfail Event"]
    #[inline(always)]
    pub fn pwr_discharge_en(&self) -> PWR_DISCHARGE_EN_R {
        PWR_DISCHARGE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TVDD12 Well Switch"]
    #[inline(always)]
    pub fn pwr_tvdd12_well(&self) -> PWR_TVDD12_WELL_R {
        PWR_TVDD12_WELL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM n-Well Switch"]
    #[inline(always)]
    pub fn pwr_sram_nwell_sw(&self) -> PWR_SRAM_NWELL_SW_R {
        PWR_SRAM_NWELL_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatically Set/Clear GPIO MBus Gate"]
    #[inline(always)]
    pub fn pwr_auto_mbus_gate(&self) -> PWR_AUTO_MBUS_GATE_R {
        PWR_AUTO_MBUS_GATE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable VDDIO SVM Run for LP2 and LP3"]
    #[inline(always)]
    pub fn pwr_svm_vddio_en_run(&self) -> PWR_SVM_VDDIO_EN_RUN_R {
        PWR_SVM_VDDIO_EN_RUN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable VDDIOH SVM Run for LP2 and LP3"]
    #[inline(always)]
    pub fn pwr_svm_vddioh_en_run(&self) -> PWR_SVM_VDDIOH_EN_RUN_R {
        PWR_SVM_VDDIOH_EN_RUN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable VDDIO SVM Run for LP2 and LP3"]
    #[inline(always)]
    pub fn pwr_retreg_src_v12(&self) -> PWR_RETREG_SRC_V12_R {
        PWR_RETREG_SRC_V12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables VRTC Rail as Retention Power Su"]
    #[inline(always)]
    pub fn pwr_retreg_src_vrtc(&self) -> PWR_RETREG_SRC_VRTC_R {
        PWR_RETREG_SRC_VRTC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables VDD18 Rail as Retention Power Supply"]
    #[inline(always)]
    pub fn pwr_retreg_src_v18(&self) -> PWR_RETREG_SRC_V18_R {
        PWR_RETREG_SRC_V18_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable VDDIO Isolation POR"]
    #[inline(always)]
    pub fn pwr_vddio_en_iso_por(&self) -> PWR_VDDIO_EN_ISO_POR_R {
        PWR_VDDIO_EN_ISO_POR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable VDDIOH Isolation POR"]
    #[inline(always)]
    pub fn pwr_vddioh_en_iso_por(&self) -> PWR_VDDIOH_EN_ISO_POR_R {
        PWR_VDDIOH_EN_ISO_POR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Core Resume to Toggle during LP0"]
    #[inline(always)]
    pub fn pwr_lp0_core_resume_en(&self) -> PWR_LP0_CORE_RESUME_EN_R {
        PWR_LP0_CORE_RESUME_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Core RSTN to Toggle during LP1"]
    #[inline(always)]
    pub fn pwr_lp1_core_rstn_en(&self) -> PWR_LP1_CORE_RSTN_EN_R {
        PWR_LP1_CORE_RSTN_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear all GPIO Event Seen Latches"]
    #[inline(always)]
    pub fn pwr_clr_io_event_latch(&mut self) -> PWR_CLR_IO_EVENT_LATCH_W {
        PWR_CLR_IO_EVENT_LATCH_W::new(self)
    }
    #[doc = "Bit 1 - Clear all GPIO Configuration Latches"]
    #[inline(always)]
    pub fn pwr_clr_io_cfg_latch(&mut self) -> PWR_CLR_IO_CFG_LATCH_W {
        PWR_CLR_IO_CFG_LATCH_W::new(self)
    }
    #[doc = "Bit 2 - Freeze GPIO MBus State"]
    #[inline(always)]
    pub fn pwr_mbus_gate(&mut self) -> PWR_MBUS_GATE_W {
        PWR_MBUS_GATE_W::new(self)
    }
    #[doc = "Bit 3 - Enable Flash Discharge During Powerfail Event"]
    #[inline(always)]
    pub fn pwr_discharge_en(&mut self) -> PWR_DISCHARGE_EN_W {
        PWR_DISCHARGE_EN_W::new(self)
    }
    #[doc = "Bit 4 - TVDD12 Well Switch"]
    #[inline(always)]
    pub fn pwr_tvdd12_well(&mut self) -> PWR_TVDD12_WELL_W {
        PWR_TVDD12_WELL_W::new(self)
    }
    #[doc = "Bit 5 - SRAM n-Well Switch"]
    #[inline(always)]
    pub fn pwr_sram_nwell_sw(&mut self) -> PWR_SRAM_NWELL_SW_W {
        PWR_SRAM_NWELL_SW_W::new(self)
    }
    #[doc = "Bit 6 - Automatically Set/Clear GPIO MBus Gate"]
    #[inline(always)]
    pub fn pwr_auto_mbus_gate(&mut self) -> PWR_AUTO_MBUS_GATE_W {
        PWR_AUTO_MBUS_GATE_W::new(self)
    }
    #[doc = "Bit 8 - Enable VDDIO SVM Run for LP2 and LP3"]
    #[inline(always)]
    pub fn pwr_svm_vddio_en_run(&mut self) -> PWR_SVM_VDDIO_EN_RUN_W {
        PWR_SVM_VDDIO_EN_RUN_W::new(self)
    }
    #[doc = "Bit 10 - Enable VDDIOH SVM Run for LP2 and LP3"]
    #[inline(always)]
    pub fn pwr_svm_vddioh_en_run(&mut self) -> PWR_SVM_VDDIOH_EN_RUN_W {
        PWR_SVM_VDDIOH_EN_RUN_W::new(self)
    }
    #[doc = "Bit 12 - Enable VDDIO SVM Run for LP2 and LP3"]
    #[inline(always)]
    pub fn pwr_retreg_src_v12(&mut self) -> PWR_RETREG_SRC_V12_W {
        PWR_RETREG_SRC_V12_W::new(self)
    }
    #[doc = "Bit 13 - Enables VRTC Rail as Retention Power Su"]
    #[inline(always)]
    pub fn pwr_retreg_src_vrtc(&mut self) -> PWR_RETREG_SRC_VRTC_W {
        PWR_RETREG_SRC_VRTC_W::new(self)
    }
    #[doc = "Bit 14 - Enables VDD18 Rail as Retention Power Supply"]
    #[inline(always)]
    pub fn pwr_retreg_src_v18(&mut self) -> PWR_RETREG_SRC_V18_W {
        PWR_RETREG_SRC_V18_W::new(self)
    }
    #[doc = "Bit 16 - Enable VDDIO Isolation POR"]
    #[inline(always)]
    pub fn pwr_vddio_en_iso_por(&mut self) -> PWR_VDDIO_EN_ISO_POR_W {
        PWR_VDDIO_EN_ISO_POR_W::new(self)
    }
    #[doc = "Bit 17 - Enable VDDIOH Isolation POR"]
    #[inline(always)]
    pub fn pwr_vddioh_en_iso_por(&mut self) -> PWR_VDDIOH_EN_ISO_POR_W {
        PWR_VDDIOH_EN_ISO_POR_W::new(self)
    }
    #[doc = "Bit 18 - Enable Core Resume to Toggle during LP0"]
    #[inline(always)]
    pub fn pwr_lp0_core_resume_en(&mut self) -> PWR_LP0_CORE_RESUME_EN_W {
        PWR_LP0_CORE_RESUME_EN_W::new(self)
    }
    #[doc = "Bit 19 - Enable Core RSTN to Toggle during LP1"]
    #[inline(always)]
    pub fn pwr_lp1_core_rstn_en(&mut self) -> PWR_LP1_CORE_RSTN_EN_W {
        PWR_LP1_CORE_RSTN_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg1](index.html) module"]
pub struct REG1_SPEC;
impl crate::RegisterSpec for REG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg1::R](R) reader structure"]
impl crate::Readable for REG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg1::W](W) writer structure"]
impl crate::Writable for REG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG1 to value 0"]
impl crate::Resettable for REG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
