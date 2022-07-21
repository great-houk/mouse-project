#[doc = "Register `IN_INT` reader"]
pub struct R(crate::R<IN_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_INT` writer"]
pub struct W(crate::W<IN_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_INT_SPEC>;
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
impl From<crate::W<IN_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `inbav0` reader - Endpoint 0 Buffer Available Interrupt Flag"]
pub type INBAV0_R = crate::BitReader<bool>;
#[doc = "Field `inbav0` writer - Endpoint 0 Buffer Available Interrupt Flag"]
pub type INBAV0_W<'a> = crate::BitWriter1C<'a, u32, IN_INT_SPEC, bool, 0>;
#[doc = "Field `inbav1` reader - Endpoint 1 Buffer Available Interrupt Flag"]
pub type INBAV1_R = crate::BitReader<bool>;
#[doc = "Field `inbav1` writer - Endpoint 1 Buffer Available Interrupt Flag"]
pub type INBAV1_W<'a> = crate::BitWriter1C<'a, u32, IN_INT_SPEC, bool, 1>;
#[doc = "Field `inbav2` reader - Endpoint 2 Buffer Available Interrupt Flag"]
pub type INBAV2_R = crate::BitReader<bool>;
#[doc = "Field `inbav2` writer - Endpoint 2 Buffer Available Interrupt Flag"]
pub type INBAV2_W<'a> = crate::BitWriter1C<'a, u32, IN_INT_SPEC, bool, 2>;
#[doc = "Field `inbav3` reader - Endpoint 3 Buffer Available Interrupt Flag"]
pub type INBAV3_R = crate::BitReader<bool>;
#[doc = "Field `inbav3` writer - Endpoint 3 Buffer Available Interrupt Flag"]
pub type INBAV3_W<'a> = crate::BitWriter1C<'a, u32, IN_INT_SPEC, bool, 3>;
#[doc = "Field `inbav4` reader - Endpoint 4 Buffer Available Interrupt Flag"]
pub type INBAV4_R = crate::BitReader<bool>;
#[doc = "Field `inbav4` writer - Endpoint 4 Buffer Available Interrupt Flag"]
pub type INBAV4_W<'a> = crate::BitWriter1C<'a, u32, IN_INT_SPEC, bool, 4>;
#[doc = "Field `inbav5` reader - Endpoint 5 Buffer Available Interrupt Flag"]
pub type INBAV5_R = crate::BitReader<bool>;
#[doc = "Field `inbav5` writer - Endpoint 5 Buffer Available Interrupt Flag"]
pub type INBAV5_W<'a> = crate::BitWriter1C<'a, u32, IN_INT_SPEC, bool, 5>;
#[doc = "Field `inbav6` reader - Endpoint 6 Buffer Available Interrupt Flag"]
pub type INBAV6_R = crate::BitReader<bool>;
#[doc = "Field `inbav6` writer - Endpoint 6 Buffer Available Interrupt Flag"]
pub type INBAV6_W<'a> = crate::BitWriter1C<'a, u32, IN_INT_SPEC, bool, 6>;
#[doc = "Field `inbav7` reader - Endpoint 7 Buffer Available Interrupt Flag"]
pub type INBAV7_R = crate::BitReader<bool>;
#[doc = "Field `inbav7` writer - Endpoint 7 Buffer Available Interrupt Flag"]
pub type INBAV7_W<'a> = crate::BitWriter1C<'a, u32, IN_INT_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Endpoint 0 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav0(&self) -> INBAV0_R {
        INBAV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav1(&self) -> INBAV1_R {
        INBAV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav2(&self) -> INBAV2_R {
        INBAV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav3(&self) -> INBAV3_R {
        INBAV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav4(&self) -> INBAV4_R {
        INBAV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav5(&self) -> INBAV5_R {
        INBAV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav6(&self) -> INBAV6_R {
        INBAV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav7(&self) -> INBAV7_R {
        INBAV7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav0(&mut self) -> INBAV0_W {
        INBAV0_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint 1 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav1(&mut self) -> INBAV1_W {
        INBAV1_W::new(self)
    }
    #[doc = "Bit 2 - Endpoint 2 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav2(&mut self) -> INBAV2_W {
        INBAV2_W::new(self)
    }
    #[doc = "Bit 3 - Endpoint 3 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav3(&mut self) -> INBAV3_W {
        INBAV3_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint 4 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav4(&mut self) -> INBAV4_W {
        INBAV4_W::new(self)
    }
    #[doc = "Bit 5 - Endpoint 5 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav5(&mut self) -> INBAV5_W {
        INBAV5_W::new(self)
    }
    #[doc = "Bit 6 - Endpoint 6 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav6(&mut self) -> INBAV6_W {
        INBAV6_W::new(self)
    }
    #[doc = "Bit 7 - Endpoint 7 Buffer Available Interrupt Flag"]
    #[inline(always)]
    pub fn inbav7(&mut self) -> INBAV7_W {
        INBAV7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB IN Endpoint Buffer Available Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_int](index.html) module"]
pub struct IN_INT_SPEC;
impl crate::RegisterSpec for IN_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_int::R](R) reader structure"]
impl crate::Readable for IN_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_int::W](W) writer structure"]
impl crate::Writable for IN_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IN_INT to value 0"]
impl crate::Resettable for IN_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
