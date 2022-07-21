#[doc = "Register `OUT_INT` reader"]
pub struct R(crate::R<OUT_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_INT` writer"]
pub struct W(crate::W<OUT_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_INT_SPEC>;
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
impl From<crate::W<OUT_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `outdav0` reader - Endpoint 0 Data Available Interrupt Flag"]
pub type OUTDAV0_R = crate::BitReader<bool>;
#[doc = "Field `outdav0` writer - Endpoint 0 Data Available Interrupt Flag"]
pub type OUTDAV0_W<'a> = crate::BitWriter1C<'a, u32, OUT_INT_SPEC, bool, 0>;
#[doc = "Field `outdav1` reader - Endpoint 1 Data Available Interrupt Flag"]
pub type OUTDAV1_R = crate::BitReader<bool>;
#[doc = "Field `outdav1` writer - Endpoint 1 Data Available Interrupt Flag"]
pub type OUTDAV1_W<'a> = crate::BitWriter1C<'a, u32, OUT_INT_SPEC, bool, 1>;
#[doc = "Field `outdav2` reader - Endpoint 2 Data Available Interrupt Flag"]
pub type OUTDAV2_R = crate::BitReader<bool>;
#[doc = "Field `outdav2` writer - Endpoint 2 Data Available Interrupt Flag"]
pub type OUTDAV2_W<'a> = crate::BitWriter1C<'a, u32, OUT_INT_SPEC, bool, 2>;
#[doc = "Field `outdav3` reader - Endpoint 3 Data Available Interrupt Flag"]
pub type OUTDAV3_R = crate::BitReader<bool>;
#[doc = "Field `outdav3` writer - Endpoint 3 Data Available Interrupt Flag"]
pub type OUTDAV3_W<'a> = crate::BitWriter1C<'a, u32, OUT_INT_SPEC, bool, 3>;
#[doc = "Field `outdav4` reader - Endpoint 4 Data Available Interrupt Flag"]
pub type OUTDAV4_R = crate::BitReader<bool>;
#[doc = "Field `outdav4` writer - Endpoint 4 Data Available Interrupt Flag"]
pub type OUTDAV4_W<'a> = crate::BitWriter1C<'a, u32, OUT_INT_SPEC, bool, 4>;
#[doc = "Field `outdav5` reader - Endpoint 5 Data Available Interrupt Flag"]
pub type OUTDAV5_R = crate::BitReader<bool>;
#[doc = "Field `outdav5` writer - Endpoint 5 Data Available Interrupt Flag"]
pub type OUTDAV5_W<'a> = crate::BitWriter1C<'a, u32, OUT_INT_SPEC, bool, 5>;
#[doc = "Field `outdav6` reader - Endpoint 6 Data Available Interrupt Flag"]
pub type OUTDAV6_R = crate::BitReader<bool>;
#[doc = "Field `outdav6` writer - Endpoint 6 Data Available Interrupt Flag"]
pub type OUTDAV6_W<'a> = crate::BitWriter1C<'a, u32, OUT_INT_SPEC, bool, 6>;
#[doc = "Field `outdav7` reader - Endpoint 7 Data Available Interrupt Flag"]
pub type OUTDAV7_R = crate::BitReader<bool>;
#[doc = "Field `outdav7` writer - Endpoint 7 Data Available Interrupt Flag"]
pub type OUTDAV7_W<'a> = crate::BitWriter1C<'a, u32, OUT_INT_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Endpoint 0 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav0(&self) -> OUTDAV0_R {
        OUTDAV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav1(&self) -> OUTDAV1_R {
        OUTDAV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav2(&self) -> OUTDAV2_R {
        OUTDAV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav3(&self) -> OUTDAV3_R {
        OUTDAV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav4(&self) -> OUTDAV4_R {
        OUTDAV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav5(&self) -> OUTDAV5_R {
        OUTDAV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav6(&self) -> OUTDAV6_R {
        OUTDAV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav7(&self) -> OUTDAV7_R {
        OUTDAV7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav0(&mut self) -> OUTDAV0_W {
        OUTDAV0_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint 1 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav1(&mut self) -> OUTDAV1_W {
        OUTDAV1_W::new(self)
    }
    #[doc = "Bit 2 - Endpoint 2 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav2(&mut self) -> OUTDAV2_W {
        OUTDAV2_W::new(self)
    }
    #[doc = "Bit 3 - Endpoint 3 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav3(&mut self) -> OUTDAV3_W {
        OUTDAV3_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint 4 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav4(&mut self) -> OUTDAV4_W {
        OUTDAV4_W::new(self)
    }
    #[doc = "Bit 5 - Endpoint 5 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav5(&mut self) -> OUTDAV5_W {
        OUTDAV5_W::new(self)
    }
    #[doc = "Bit 6 - Endpoint 6 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav6(&mut self) -> OUTDAV6_W {
        OUTDAV6_W::new(self)
    }
    #[doc = "Bit 7 - Endpoint 7 Data Available Interrupt Flag"]
    #[inline(always)]
    pub fn outdav7(&mut self) -> OUTDAV7_W {
        OUTDAV7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB OUT Endpoint Data Available Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_int](index.html) module"]
pub struct OUT_INT_SPEC;
impl crate::RegisterSpec for OUT_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_int::R](R) reader structure"]
impl crate::Readable for OUT_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_int::W](W) writer structure"]
impl crate::Writable for OUT_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_INT to value 0"]
impl crate::Resettable for OUT_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
