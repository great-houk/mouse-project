#[doc = "Register `INTFL` reader"]
pub struct R(crate::R<INTFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFL` writer"]
pub struct W(crate::W<INTFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFL_SPEC>;
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
impl From<crate::W<INTFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pt0` reader - Pulse Train 0 Stopped Interrupt Flag"]
pub type PT0_R = crate::BitReader<bool>;
#[doc = "Field `pt0` writer - Pulse Train 0 Stopped Interrupt Flag"]
pub type PT0_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 0>;
#[doc = "Field `pt1` reader - Pulse Train 1 Stopped Interrupt Flag"]
pub type PT1_R = crate::BitReader<bool>;
#[doc = "Field `pt1` writer - Pulse Train 1 Stopped Interrupt Flag"]
pub type PT1_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 1>;
#[doc = "Field `pt2` reader - Pulse Train 2 Stopped Interrupt Flag"]
pub type PT2_R = crate::BitReader<bool>;
#[doc = "Field `pt2` writer - Pulse Train 2 Stopped Interrupt Flag"]
pub type PT2_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 2>;
#[doc = "Field `pt3` reader - Pulse Train 3 Stopped Interrupt Flag"]
pub type PT3_R = crate::BitReader<bool>;
#[doc = "Field `pt3` writer - Pulse Train 3 Stopped Interrupt Flag"]
pub type PT3_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 3>;
#[doc = "Field `pt4` reader - Pulse Train 4 Stopped Interrupt Flag"]
pub type PT4_R = crate::BitReader<bool>;
#[doc = "Field `pt4` writer - Pulse Train 4 Stopped Interrupt Flag"]
pub type PT4_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 4>;
#[doc = "Field `pt5` reader - Pulse Train 5 Stopped Interrupt Flag"]
pub type PT5_R = crate::BitReader<bool>;
#[doc = "Field `pt5` writer - Pulse Train 5 Stopped Interrupt Flag"]
pub type PT5_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 5>;
#[doc = "Field `pt6` reader - Pulse Train 6 Stopped Interrupt Flag"]
pub type PT6_R = crate::BitReader<bool>;
#[doc = "Field `pt6` writer - Pulse Train 6 Stopped Interrupt Flag"]
pub type PT6_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 6>;
#[doc = "Field `pt7` reader - Pulse Train 7 Stopped Interrupt Flag"]
pub type PT7_R = crate::BitReader<bool>;
#[doc = "Field `pt7` writer - Pulse Train 7 Stopped Interrupt Flag"]
pub type PT7_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 7>;
#[doc = "Field `pt8` reader - Pulse Train 8 Stopped Interrupt Flag"]
pub type PT8_R = crate::BitReader<bool>;
#[doc = "Field `pt8` writer - Pulse Train 8 Stopped Interrupt Flag"]
pub type PT8_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 8>;
#[doc = "Field `pt9` reader - Pulse Train 9 Stopped Interrupt Flag"]
pub type PT9_R = crate::BitReader<bool>;
#[doc = "Field `pt9` writer - Pulse Train 9 Stopped Interrupt Flag"]
pub type PT9_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 9>;
#[doc = "Field `pt10` reader - Pulse Train 10 Stopped Interrupt Flag"]
pub type PT10_R = crate::BitReader<bool>;
#[doc = "Field `pt10` writer - Pulse Train 10 Stopped Interrupt Flag"]
pub type PT10_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 10>;
#[doc = "Field `pt11` reader - Pulse Train 11 Stopped Interrupt Flag"]
pub type PT11_R = crate::BitReader<bool>;
#[doc = "Field `pt11` writer - Pulse Train 11 Stopped Interrupt Flag"]
pub type PT11_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 11>;
#[doc = "Field `pt12` reader - Pulse Train 12 Stopped Interrupt Flag"]
pub type PT12_R = crate::BitReader<bool>;
#[doc = "Field `pt12` writer - Pulse Train 12 Stopped Interrupt Flag"]
pub type PT12_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 12>;
#[doc = "Field `pt13` reader - Pulse Train 13 Stopped Interrupt Flag"]
pub type PT13_R = crate::BitReader<bool>;
#[doc = "Field `pt13` writer - Pulse Train 13 Stopped Interrupt Flag"]
pub type PT13_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 13>;
#[doc = "Field `pt14` reader - Pulse Train 14 Stopped Interrupt Flag"]
pub type PT14_R = crate::BitReader<bool>;
#[doc = "Field `pt14` writer - Pulse Train 14 Stopped Interrupt Flag"]
pub type PT14_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 14>;
#[doc = "Field `pt15` reader - Pulse Train 15 Stopped Interrupt Flag"]
pub type PT15_R = crate::BitReader<bool>;
#[doc = "Field `pt15` writer - Pulse Train 15 Stopped Interrupt Flag"]
pub type PT15_W<'a> = crate::BitWriter<'a, u32, INTFL_SPEC, bool, 15>;
impl R {
    #[doc = "Bit 0 - Pulse Train 0 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt0(&self) -> PT0_R {
        PT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pulse Train 1 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt1(&self) -> PT1_R {
        PT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pulse Train 2 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt2(&self) -> PT2_R {
        PT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pulse Train 3 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt3(&self) -> PT3_R {
        PT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pulse Train 4 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt4(&self) -> PT4_R {
        PT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pulse Train 5 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt5(&self) -> PT5_R {
        PT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pulse Train 6 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt6(&self) -> PT6_R {
        PT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pulse Train 7 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt7(&self) -> PT7_R {
        PT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pulse Train 8 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt8(&self) -> PT8_R {
        PT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pulse Train 9 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt9(&self) -> PT9_R {
        PT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pulse Train 10 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt10(&self) -> PT10_R {
        PT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pulse Train 11 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt11(&self) -> PT11_R {
        PT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pulse Train 12 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt12(&self) -> PT12_R {
        PT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pulse Train 13 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt13(&self) -> PT13_R {
        PT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pulse Train 14 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt14(&self) -> PT14_R {
        PT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pulse Train 15 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt15(&self) -> PT15_R {
        PT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pulse Train 0 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt0(&mut self) -> PT0_W {
        PT0_W::new(self)
    }
    #[doc = "Bit 1 - Pulse Train 1 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt1(&mut self) -> PT1_W {
        PT1_W::new(self)
    }
    #[doc = "Bit 2 - Pulse Train 2 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt2(&mut self) -> PT2_W {
        PT2_W::new(self)
    }
    #[doc = "Bit 3 - Pulse Train 3 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt3(&mut self) -> PT3_W {
        PT3_W::new(self)
    }
    #[doc = "Bit 4 - Pulse Train 4 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt4(&mut self) -> PT4_W {
        PT4_W::new(self)
    }
    #[doc = "Bit 5 - Pulse Train 5 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt5(&mut self) -> PT5_W {
        PT5_W::new(self)
    }
    #[doc = "Bit 6 - Pulse Train 6 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt6(&mut self) -> PT6_W {
        PT6_W::new(self)
    }
    #[doc = "Bit 7 - Pulse Train 7 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt7(&mut self) -> PT7_W {
        PT7_W::new(self)
    }
    #[doc = "Bit 8 - Pulse Train 8 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt8(&mut self) -> PT8_W {
        PT8_W::new(self)
    }
    #[doc = "Bit 9 - Pulse Train 9 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt9(&mut self) -> PT9_W {
        PT9_W::new(self)
    }
    #[doc = "Bit 10 - Pulse Train 10 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt10(&mut self) -> PT10_W {
        PT10_W::new(self)
    }
    #[doc = "Bit 11 - Pulse Train 11 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt11(&mut self) -> PT11_W {
        PT11_W::new(self)
    }
    #[doc = "Bit 12 - Pulse Train 12 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt12(&mut self) -> PT12_W {
        PT12_W::new(self)
    }
    #[doc = "Bit 13 - Pulse Train 13 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt13(&mut self) -> PT13_W {
        PT13_W::new(self)
    }
    #[doc = "Bit 14 - Pulse Train 14 Stopped Interrupt Flag"]
    #[inline(always)]
    pub fn pt14(&mut self) -> PT14_W {
        PT14_W::new(self)
    }
    #[doc = "Bit 15 - Pulse Train 15 Stopped Interrupt Flag"]
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
#[doc = "Pulse Train Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](index.html) module"]
pub struct INTFL_SPEC;
impl crate::RegisterSpec for INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intfl::R](R) reader structure"]
impl crate::Readable for INTFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intfl::W](W) writer structure"]
impl crate::Writable for INTFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for INTFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
