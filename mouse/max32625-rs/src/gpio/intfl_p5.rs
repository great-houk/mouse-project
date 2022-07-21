#[doc = "Register `INTFL_P5` reader"]
pub struct R(crate::R<INTFL_P5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFL_P5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFL_P5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFL_P5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFL_P5` writer"]
pub struct W(crate::W<INTFL_P5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFL_P5_SPEC>;
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
impl From<crate::W<INTFL_P5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFL_P5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - P5.0 External Interrupt Flags"]
pub type PIN0_R = crate::BitReader<bool>;
#[doc = "Field `pin0` writer - P5.0 External Interrupt Flags"]
pub type PIN0_W<'a> = crate::BitWriter1C<'a, u32, INTFL_P5_SPEC, bool, 0>;
#[doc = "Field `pin1` reader - P5.1 External Interrupt Flags"]
pub type PIN1_R = crate::BitReader<bool>;
#[doc = "Field `pin1` writer - P5.1 External Interrupt Flags"]
pub type PIN1_W<'a> = crate::BitWriter1C<'a, u32, INTFL_P5_SPEC, bool, 1>;
#[doc = "Field `pin2` reader - P5.2 External Interrupt Flags"]
pub type PIN2_R = crate::BitReader<bool>;
#[doc = "Field `pin2` writer - P5.2 External Interrupt Flags"]
pub type PIN2_W<'a> = crate::BitWriter1C<'a, u32, INTFL_P5_SPEC, bool, 2>;
#[doc = "Field `pin3` reader - P5.3 External Interrupt Flags"]
pub type PIN3_R = crate::BitReader<bool>;
#[doc = "Field `pin3` writer - P5.3 External Interrupt Flags"]
pub type PIN3_W<'a> = crate::BitWriter1C<'a, u32, INTFL_P5_SPEC, bool, 3>;
#[doc = "Field `pin4` reader - P5.4 External Interrupt Flags"]
pub type PIN4_R = crate::BitReader<bool>;
#[doc = "Field `pin4` writer - P5.4 External Interrupt Flags"]
pub type PIN4_W<'a> = crate::BitWriter1C<'a, u32, INTFL_P5_SPEC, bool, 4>;
#[doc = "Field `pin5` reader - P5.5 External Interrupt Flags"]
pub type PIN5_R = crate::BitReader<bool>;
#[doc = "Field `pin5` writer - P5.5 External Interrupt Flags"]
pub type PIN5_W<'a> = crate::BitWriter1C<'a, u32, INTFL_P5_SPEC, bool, 5>;
#[doc = "Field `pin6` reader - P5.6 External Interrupt Flags"]
pub type PIN6_R = crate::BitReader<bool>;
#[doc = "Field `pin6` writer - P5.6 External Interrupt Flags"]
pub type PIN6_W<'a> = crate::BitWriter1C<'a, u32, INTFL_P5_SPEC, bool, 6>;
#[doc = "Field `pin7` reader - P5.7 External Interrupt Flags"]
pub type PIN7_R = crate::BitReader<bool>;
#[doc = "Field `pin7` writer - P5.7 External Interrupt Flags"]
pub type PIN7_W<'a> = crate::BitWriter1C<'a, u32, INTFL_P5_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P5.0 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P5.1 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P5.2 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P5.3 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P5.4 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P5.5 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P5.6 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P5.7 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P5.0 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W {
        PIN0_W::new(self)
    }
    #[doc = "Bit 1 - P5.1 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W {
        PIN1_W::new(self)
    }
    #[doc = "Bit 2 - P5.2 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W {
        PIN2_W::new(self)
    }
    #[doc = "Bit 3 - P5.3 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W {
        PIN3_W::new(self)
    }
    #[doc = "Bit 4 - P5.4 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W {
        PIN4_W::new(self)
    }
    #[doc = "Bit 5 - P5.5 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W {
        PIN5_W::new(self)
    }
    #[doc = "Bit 6 - P5.6 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W {
        PIN6_W::new(self)
    }
    #[doc = "Bit 7 - P5.7 External Interrupt Flags"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W {
        PIN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port P5 Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl_p5](index.html) module"]
pub struct INTFL_P5_SPEC;
impl crate::RegisterSpec for INTFL_P5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intfl_p5::R](R) reader structure"]
impl crate::Readable for INTFL_P5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intfl_p5::W](W) writer structure"]
impl crate::Writable for INTFL_P5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFL_P5 to value 0"]
impl crate::Resettable for INTFL_P5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
