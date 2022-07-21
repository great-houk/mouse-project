#[doc = "Register `RESYNC` reader"]
pub struct R(crate::R<RESYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESYNC` writer"]
pub struct W(crate::W<RESYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESYNC_SPEC>;
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
impl From<crate::W<RESYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pt0` reader - Resync control for PT0"]
pub type PT0_R = crate::BitReader<bool>;
#[doc = "Field `pt0` writer - Resync control for PT0"]
pub type PT0_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 0>;
#[doc = "Field `pt1` reader - Resync control for PT1"]
pub type PT1_R = crate::BitReader<bool>;
#[doc = "Field `pt1` writer - Resync control for PT1"]
pub type PT1_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 1>;
#[doc = "Field `pt2` reader - Resync control for PT2"]
pub type PT2_R = crate::BitReader<bool>;
#[doc = "Field `pt2` writer - Resync control for PT2"]
pub type PT2_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 2>;
#[doc = "Field `pt3` reader - Resync control for PT3"]
pub type PT3_R = crate::BitReader<bool>;
#[doc = "Field `pt3` writer - Resync control for PT3"]
pub type PT3_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 3>;
#[doc = "Field `pt4` reader - Resync control for PT4"]
pub type PT4_R = crate::BitReader<bool>;
#[doc = "Field `pt4` writer - Resync control for PT4"]
pub type PT4_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 4>;
#[doc = "Field `pt5` reader - Resync control for PT5"]
pub type PT5_R = crate::BitReader<bool>;
#[doc = "Field `pt5` writer - Resync control for PT5"]
pub type PT5_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 5>;
#[doc = "Field `pt6` reader - Resync control for PT6"]
pub type PT6_R = crate::BitReader<bool>;
#[doc = "Field `pt6` writer - Resync control for PT6"]
pub type PT6_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 6>;
#[doc = "Field `pt7` reader - Resync control for PT7"]
pub type PT7_R = crate::BitReader<bool>;
#[doc = "Field `pt7` writer - Resync control for PT7"]
pub type PT7_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 7>;
#[doc = "Field `pt8` reader - Resync control for PT8"]
pub type PT8_R = crate::BitReader<bool>;
#[doc = "Field `pt8` writer - Resync control for PT8"]
pub type PT8_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 8>;
#[doc = "Field `pt9` reader - Resync control for PT9"]
pub type PT9_R = crate::BitReader<bool>;
#[doc = "Field `pt9` writer - Resync control for PT9"]
pub type PT9_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 9>;
#[doc = "Field `pt10` reader - Resync control for PT10"]
pub type PT10_R = crate::BitReader<bool>;
#[doc = "Field `pt10` writer - Resync control for PT10"]
pub type PT10_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 10>;
#[doc = "Field `pt11` reader - Resync control for PT11"]
pub type PT11_R = crate::BitReader<bool>;
#[doc = "Field `pt11` writer - Resync control for PT11"]
pub type PT11_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 11>;
#[doc = "Field `pt12` reader - Resync control for PT12"]
pub type PT12_R = crate::BitReader<bool>;
#[doc = "Field `pt12` writer - Resync control for PT12"]
pub type PT12_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 12>;
#[doc = "Field `pt13` reader - Resync control for PT13"]
pub type PT13_R = crate::BitReader<bool>;
#[doc = "Field `pt13` writer - Resync control for PT13"]
pub type PT13_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 13>;
#[doc = "Field `pt14` reader - Resync control for PT14"]
pub type PT14_R = crate::BitReader<bool>;
#[doc = "Field `pt14` writer - Resync control for PT14"]
pub type PT14_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 14>;
#[doc = "Field `pt15` reader - Resync control for PT15"]
pub type PT15_R = crate::BitReader<bool>;
#[doc = "Field `pt15` writer - Resync control for PT15"]
pub type PT15_W<'a> = crate::BitWriter<'a, u32, RESYNC_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - Resync control for PT0"]
    #[inline(always)]
    pub fn pt0(&self) -> PT0_R {
        PT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resync control for PT1"]
    #[inline(always)]
    pub fn pt1(&self) -> PT1_R {
        PT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Resync control for PT2"]
    #[inline(always)]
    pub fn pt2(&self) -> PT2_R {
        PT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Resync control for PT3"]
    #[inline(always)]
    pub fn pt3(&self) -> PT3_R {
        PT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Resync control for PT4"]
    #[inline(always)]
    pub fn pt4(&self) -> PT4_R {
        PT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Resync control for PT5"]
    #[inline(always)]
    pub fn pt5(&self) -> PT5_R {
        PT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Resync control for PT6"]
    #[inline(always)]
    pub fn pt6(&self) -> PT6_R {
        PT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Resync control for PT7"]
    #[inline(always)]
    pub fn pt7(&self) -> PT7_R {
        PT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Resync control for PT8"]
    #[inline(always)]
    pub fn pt8(&self) -> PT8_R {
        PT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Resync control for PT9"]
    #[inline(always)]
    pub fn pt9(&self) -> PT9_R {
        PT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Resync control for PT10"]
    #[inline(always)]
    pub fn pt10(&self) -> PT10_R {
        PT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Resync control for PT11"]
    #[inline(always)]
    pub fn pt11(&self) -> PT11_R {
        PT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Resync control for PT12"]
    #[inline(always)]
    pub fn pt12(&self) -> PT12_R {
        PT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Resync control for PT13"]
    #[inline(always)]
    pub fn pt13(&self) -> PT13_R {
        PT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Resync control for PT14"]
    #[inline(always)]
    pub fn pt14(&self) -> PT14_R {
        PT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Resync control for PT15"]
    #[inline(always)]
    pub fn pt15(&self) -> PT15_R {
        PT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Resync control for PT0"]
    #[inline(always)]
    pub fn pt0(&mut self) -> PT0_W {
        PT0_W::new(self)
    }
    #[doc = "Bit 1 - Resync control for PT1"]
    #[inline(always)]
    pub fn pt1(&mut self) -> PT1_W {
        PT1_W::new(self)
    }
    #[doc = "Bit 2 - Resync control for PT2"]
    #[inline(always)]
    pub fn pt2(&mut self) -> PT2_W {
        PT2_W::new(self)
    }
    #[doc = "Bit 3 - Resync control for PT3"]
    #[inline(always)]
    pub fn pt3(&mut self) -> PT3_W {
        PT3_W::new(self)
    }
    #[doc = "Bit 4 - Resync control for PT4"]
    #[inline(always)]
    pub fn pt4(&mut self) -> PT4_W {
        PT4_W::new(self)
    }
    #[doc = "Bit 5 - Resync control for PT5"]
    #[inline(always)]
    pub fn pt5(&mut self) -> PT5_W {
        PT5_W::new(self)
    }
    #[doc = "Bit 6 - Resync control for PT6"]
    #[inline(always)]
    pub fn pt6(&mut self) -> PT6_W {
        PT6_W::new(self)
    }
    #[doc = "Bit 7 - Resync control for PT7"]
    #[inline(always)]
    pub fn pt7(&mut self) -> PT7_W {
        PT7_W::new(self)
    }
    #[doc = "Bit 8 - Resync control for PT8"]
    #[inline(always)]
    pub fn pt8(&mut self) -> PT8_W {
        PT8_W::new(self)
    }
    #[doc = "Bit 9 - Resync control for PT9"]
    #[inline(always)]
    pub fn pt9(&mut self) -> PT9_W {
        PT9_W::new(self)
    }
    #[doc = "Bit 10 - Resync control for PT10"]
    #[inline(always)]
    pub fn pt10(&mut self) -> PT10_W {
        PT10_W::new(self)
    }
    #[doc = "Bit 11 - Resync control for PT11"]
    #[inline(always)]
    pub fn pt11(&mut self) -> PT11_W {
        PT11_W::new(self)
    }
    #[doc = "Bit 12 - Resync control for PT12"]
    #[inline(always)]
    pub fn pt12(&mut self) -> PT12_W {
        PT12_W::new(self)
    }
    #[doc = "Bit 13 - Resync control for PT13"]
    #[inline(always)]
    pub fn pt13(&mut self) -> PT13_W {
        PT13_W::new(self)
    }
    #[doc = "Bit 14 - Resync control for PT14"]
    #[inline(always)]
    pub fn pt14(&mut self) -> PT14_W {
        PT14_W::new(self)
    }
    #[doc = "Bit 15 - Resync control for PT15"]
    #[inline(always)]
    pub fn pt15(&mut self) -> PT15_W {
        PT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Resync (All Pulse Trains) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resync](index.html) module"]
pub struct RESYNC_SPEC;
impl crate::RegisterSpec for RESYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resync::R](R) reader structure"]
impl crate::Readable for RESYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resync::W](W) writer structure"]
impl crate::Writable for RESYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESYNC to value 0"]
impl crate::Resettable for RESYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
