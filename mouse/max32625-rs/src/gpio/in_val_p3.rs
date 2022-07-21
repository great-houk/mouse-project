#[doc = "Register `IN_VAL_P3` reader"]
pub struct R(crate::R<IN_VAL_P3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_VAL_P3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_VAL_P3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_VAL_P3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_VAL_P3` writer"]
pub struct W(crate::W<IN_VAL_P3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_VAL_P3_SPEC>;
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
impl From<crate::W<IN_VAL_P3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_VAL_P3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - P3.0 Input Value"]
pub type PIN0_R = crate::BitReader<bool>;
#[doc = "Field `pin1` reader - P3.1 Input Value"]
pub type PIN1_R = crate::BitReader<bool>;
#[doc = "Field `pin2` reader - P3.2 Input Value"]
pub type PIN2_R = crate::BitReader<bool>;
#[doc = "Field `pin3` reader - P3.3 Input Value"]
pub type PIN3_R = crate::BitReader<bool>;
#[doc = "Field `pin4` reader - P3.4 Input Value"]
pub type PIN4_R = crate::BitReader<bool>;
#[doc = "Field `pin5` reader - P3.5 Input Value"]
pub type PIN5_R = crate::BitReader<bool>;
#[doc = "Field `pin6` reader - P3.6 Input Value"]
pub type PIN6_R = crate::BitReader<bool>;
#[doc = "Field `pin7` reader - P3.7 Input Value"]
pub type PIN7_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - P3.0 Input Value"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P3.1 Input Value"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P3.2 Input Value"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P3.3 Input Value"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P3.4 Input Value"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P3.5 Input Value"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P3.6 Input Value"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P3.7 Input Value"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 1) != 0)
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
#[doc = "Port P3 GPIO Input Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_val_p3](index.html) module"]
pub struct IN_VAL_P3_SPEC;
impl crate::RegisterSpec for IN_VAL_P3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_val_p3::R](R) reader structure"]
impl crate::Readable for IN_VAL_P3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_val_p3::W](W) writer structure"]
impl crate::Writable for IN_VAL_P3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_VAL_P3 to value 0"]
impl crate::Resettable for IN_VAL_P3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
