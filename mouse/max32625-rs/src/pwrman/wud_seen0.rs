#[doc = "Register `WUD_SEEN0` reader"]
pub struct R(crate::R<WUD_SEEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUD_SEEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUD_SEEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUD_SEEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUD_SEEN0` writer"]
pub struct W(crate::W<WUD_SEEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUD_SEEN0_SPEC>;
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
impl From<crate::W<WUD_SEEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUD_SEEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpio0` reader - Wake-Up Detect Status for P0.0"]
pub type GPIO0_R = crate::BitReader<bool>;
#[doc = "Field `gpio1` reader - Wake-Up Detect Status for P0.1"]
pub type GPIO1_R = crate::BitReader<bool>;
#[doc = "Field `gpio2` reader - Wake-Up Detect Status for P0.2"]
pub type GPIO2_R = crate::BitReader<bool>;
#[doc = "Field `gpio3` reader - Wake-Up Detect Status for P0.3"]
pub type GPIO3_R = crate::BitReader<bool>;
#[doc = "Field `gpio4` reader - Wake-Up Detect Status for P0.4"]
pub type GPIO4_R = crate::BitReader<bool>;
#[doc = "Field `gpio5` reader - Wake-Up Detect Status for P0.5"]
pub type GPIO5_R = crate::BitReader<bool>;
#[doc = "Field `gpio6` reader - Wake-Up Detect Status for P0.6"]
pub type GPIO6_R = crate::BitReader<bool>;
#[doc = "Field `gpio7` reader - Wake-Up Detect Status for P0.7"]
pub type GPIO7_R = crate::BitReader<bool>;
#[doc = "Field `gpio8` reader - Wake-Up Detect Status for P1.0"]
pub type GPIO8_R = crate::BitReader<bool>;
#[doc = "Field `gpio9` reader - Wake-Up Detect Status for P1.1"]
pub type GPIO9_R = crate::BitReader<bool>;
#[doc = "Field `gpio10` reader - Wake-Up Detect Status for P1.2"]
pub type GPIO10_R = crate::BitReader<bool>;
#[doc = "Field `gpio11` reader - Wake-Up Detect Status for P1.3"]
pub type GPIO11_R = crate::BitReader<bool>;
#[doc = "Field `gpio12` reader - Wake-Up Detect Status for P1.4"]
pub type GPIO12_R = crate::BitReader<bool>;
#[doc = "Field `gpio13` reader - Wake-Up Detect Status for P1.5"]
pub type GPIO13_R = crate::BitReader<bool>;
#[doc = "Field `gpio14` reader - Wake-Up Detect Status for P1.6"]
pub type GPIO14_R = crate::BitReader<bool>;
#[doc = "Field `gpio15` reader - Wake-Up Detect Status for P1.7"]
pub type GPIO15_R = crate::BitReader<bool>;
#[doc = "Field `gpio16` reader - Wake-Up Detect Status for P2.0"]
pub type GPIO16_R = crate::BitReader<bool>;
#[doc = "Field `gpio17` reader - Wake-Up Detect Status for P2.1"]
pub type GPIO17_R = crate::BitReader<bool>;
#[doc = "Field `gpio18` reader - Wake-Up Detect Status for P2.2"]
pub type GPIO18_R = crate::BitReader<bool>;
#[doc = "Field `gpio19` reader - Wake-Up Detect Status for P2.3"]
pub type GPIO19_R = crate::BitReader<bool>;
#[doc = "Field `gpio20` reader - Wake-Up Detect Status for P2.4"]
pub type GPIO20_R = crate::BitReader<bool>;
#[doc = "Field `gpio21` reader - Wake-Up Detect Status for P2.5"]
pub type GPIO21_R = crate::BitReader<bool>;
#[doc = "Field `gpio22` reader - Wake-Up Detect Status for P2.6"]
pub type GPIO22_R = crate::BitReader<bool>;
#[doc = "Field `gpio23` reader - Wake-Up Detect Status for P2.7"]
pub type GPIO23_R = crate::BitReader<bool>;
#[doc = "Field `gpio24` reader - Wake-Up Detect Status for P3.0"]
pub type GPIO24_R = crate::BitReader<bool>;
#[doc = "Field `gpio25` reader - Wake-Up Detect Status for P3.1"]
pub type GPIO25_R = crate::BitReader<bool>;
#[doc = "Field `gpio26` reader - Wake-Up Detect Status for P3.2"]
pub type GPIO26_R = crate::BitReader<bool>;
#[doc = "Field `gpio27` reader - Wake-Up Detect Status for P3.3"]
pub type GPIO27_R = crate::BitReader<bool>;
#[doc = "Field `gpio28` reader - Wake-Up Detect Status for P3.4"]
pub type GPIO28_R = crate::BitReader<bool>;
#[doc = "Field `gpio29` reader - Wake-Up Detect Status for P3.5"]
pub type GPIO29_R = crate::BitReader<bool>;
#[doc = "Field `gpio30` reader - Wake-Up Detect Status for P3.6"]
pub type GPIO30_R = crate::BitReader<bool>;
#[doc = "Field `gpio31` reader - Wake-Up Detect Status for P3.7"]
pub type GPIO31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Wake-Up Detect Status for P0.0"]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-Up Detect Status for P0.1"]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Detect Status for P0.2"]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake-Up Detect Status for P0.3"]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Detect Status for P0.4"]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake-Up Detect Status for P0.5"]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake-Up Detect Status for P0.6"]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO6_R {
        GPIO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wake-Up Detect Status for P0.7"]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO7_R {
        GPIO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-Up Detect Status for P1.0"]
    #[inline(always)]
    pub fn gpio8(&self) -> GPIO8_R {
        GPIO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wake-Up Detect Status for P1.1"]
    #[inline(always)]
    pub fn gpio9(&self) -> GPIO9_R {
        GPIO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wake-Up Detect Status for P1.2"]
    #[inline(always)]
    pub fn gpio10(&self) -> GPIO10_R {
        GPIO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake-Up Detect Status for P1.3"]
    #[inline(always)]
    pub fn gpio11(&self) -> GPIO11_R {
        GPIO11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wake-Up Detect Status for P1.4"]
    #[inline(always)]
    pub fn gpio12(&self) -> GPIO12_R {
        GPIO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wake-Up Detect Status for P1.5"]
    #[inline(always)]
    pub fn gpio13(&self) -> GPIO13_R {
        GPIO13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wake-Up Detect Status for P1.6"]
    #[inline(always)]
    pub fn gpio14(&self) -> GPIO14_R {
        GPIO14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wake-Up Detect Status for P1.7"]
    #[inline(always)]
    pub fn gpio15(&self) -> GPIO15_R {
        GPIO15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wake-Up Detect Status for P2.0"]
    #[inline(always)]
    pub fn gpio16(&self) -> GPIO16_R {
        GPIO16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake-Up Detect Status for P2.1"]
    #[inline(always)]
    pub fn gpio17(&self) -> GPIO17_R {
        GPIO17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake-Up Detect Status for P2.2"]
    #[inline(always)]
    pub fn gpio18(&self) -> GPIO18_R {
        GPIO18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake-Up Detect Status for P2.3"]
    #[inline(always)]
    pub fn gpio19(&self) -> GPIO19_R {
        GPIO19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wake-Up Detect Status for P2.4"]
    #[inline(always)]
    pub fn gpio20(&self) -> GPIO20_R {
        GPIO20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wake-Up Detect Status for P2.5"]
    #[inline(always)]
    pub fn gpio21(&self) -> GPIO21_R {
        GPIO21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wake-Up Detect Status for P2.6"]
    #[inline(always)]
    pub fn gpio22(&self) -> GPIO22_R {
        GPIO22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Wake-Up Detect Status for P2.7"]
    #[inline(always)]
    pub fn gpio23(&self) -> GPIO23_R {
        GPIO23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Wake-Up Detect Status for P3.0"]
    #[inline(always)]
    pub fn gpio24(&self) -> GPIO24_R {
        GPIO24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wake-Up Detect Status for P3.1"]
    #[inline(always)]
    pub fn gpio25(&self) -> GPIO25_R {
        GPIO25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake-Up Detect Status for P3.2"]
    #[inline(always)]
    pub fn gpio26(&self) -> GPIO26_R {
        GPIO26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wake-Up Detect Status for P3.3"]
    #[inline(always)]
    pub fn gpio27(&self) -> GPIO27_R {
        GPIO27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake-Up Detect Status for P3.4"]
    #[inline(always)]
    pub fn gpio28(&self) -> GPIO28_R {
        GPIO28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wake-Up Detect Status for P3.5"]
    #[inline(always)]
    pub fn gpio29(&self) -> GPIO29_R {
        GPIO29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wake-Up Detect Status for P3.6"]
    #[inline(always)]
    pub fn gpio30(&self) -> GPIO30_R {
        GPIO30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Wake-Up Detect Status for P3.7"]
    #[inline(always)]
    pub fn gpio31(&self) -> GPIO31_R {
        GPIO31_R::new(((self.bits >> 31) & 1) != 0)
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
#[doc = "Wake-up Detect Status for P0/P1/P2/P3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wud_seen0](index.html) module"]
pub struct WUD_SEEN0_SPEC;
impl crate::RegisterSpec for WUD_SEEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wud_seen0::R](R) reader structure"]
impl crate::Readable for WUD_SEEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wud_seen0::W](W) writer structure"]
impl crate::Writable for WUD_SEEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUD_SEEN0 to value 0"]
impl crate::Resettable for WUD_SEEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
