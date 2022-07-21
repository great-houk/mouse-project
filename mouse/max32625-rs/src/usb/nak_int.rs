#[doc = "Register `NAK_INT` reader"]
pub struct R(crate::R<NAK_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NAK_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NAK_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NAK_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NAK_INT` writer"]
pub struct W(crate::W<NAK_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NAK_INT_SPEC>;
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
impl From<crate::W<NAK_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NAK_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `nak0` reader - Endpoint 0 NAK Interrupt Flag"]
pub type NAK0_R = crate::BitReader<bool>;
#[doc = "Field `nak0` writer - Endpoint 0 NAK Interrupt Flag"]
pub type NAK0_W<'a> = crate::BitWriter1C<'a, u32, NAK_INT_SPEC, bool, 0>;
#[doc = "Field `nak1` reader - Endpoint 1 NAK Interrupt Flag"]
pub type NAK1_R = crate::BitReader<bool>;
#[doc = "Field `nak1` writer - Endpoint 1 NAK Interrupt Flag"]
pub type NAK1_W<'a> = crate::BitWriter1C<'a, u32, NAK_INT_SPEC, bool, 1>;
#[doc = "Field `nak2` reader - Endpoint 2 NAK Interrupt Flag"]
pub type NAK2_R = crate::BitReader<bool>;
#[doc = "Field `nak2` writer - Endpoint 2 NAK Interrupt Flag"]
pub type NAK2_W<'a> = crate::BitWriter1C<'a, u32, NAK_INT_SPEC, bool, 2>;
#[doc = "Field `nak3` reader - Endpoint 3 NAK Interrupt Flag"]
pub type NAK3_R = crate::BitReader<bool>;
#[doc = "Field `nak3` writer - Endpoint 3 NAK Interrupt Flag"]
pub type NAK3_W<'a> = crate::BitWriter1C<'a, u32, NAK_INT_SPEC, bool, 3>;
#[doc = "Field `nak4` reader - Endpoint 4 NAK Interrupt Flag"]
pub type NAK4_R = crate::BitReader<bool>;
#[doc = "Field `nak4` writer - Endpoint 4 NAK Interrupt Flag"]
pub type NAK4_W<'a> = crate::BitWriter1C<'a, u32, NAK_INT_SPEC, bool, 4>;
#[doc = "Field `nak5` reader - Endpoint 5 NAK Interrupt Flag"]
pub type NAK5_R = crate::BitReader<bool>;
#[doc = "Field `nak5` writer - Endpoint 5 NAK Interrupt Flag"]
pub type NAK5_W<'a> = crate::BitWriter1C<'a, u32, NAK_INT_SPEC, bool, 5>;
#[doc = "Field `nak6` reader - Endpoint 6 NAK Interrupt Flag"]
pub type NAK6_R = crate::BitReader<bool>;
#[doc = "Field `nak6` writer - Endpoint 6 NAK Interrupt Flag"]
pub type NAK6_W<'a> = crate::BitWriter1C<'a, u32, NAK_INT_SPEC, bool, 6>;
#[doc = "Field `nak7` reader - Endpoint 7 NAK Interrupt Flag"]
pub type NAK7_R = crate::BitReader<bool>;
#[doc = "Field `nak7` writer - Endpoint 7 NAK Interrupt Flag"]
pub type NAK7_W<'a> = crate::BitWriter1C<'a, u32, NAK_INT_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Endpoint 0 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak0(&self) -> NAK0_R {
        NAK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak1(&self) -> NAK1_R {
        NAK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak2(&self) -> NAK2_R {
        NAK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak3(&self) -> NAK3_R {
        NAK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak4(&self) -> NAK4_R {
        NAK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak5(&self) -> NAK5_R {
        NAK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak6(&self) -> NAK6_R {
        NAK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak7(&self) -> NAK7_R {
        NAK7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak0(&mut self) -> NAK0_W {
        NAK0_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint 1 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak1(&mut self) -> NAK1_W {
        NAK1_W::new(self)
    }
    #[doc = "Bit 2 - Endpoint 2 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak2(&mut self) -> NAK2_W {
        NAK2_W::new(self)
    }
    #[doc = "Bit 3 - Endpoint 3 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak3(&mut self) -> NAK3_W {
        NAK3_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint 4 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak4(&mut self) -> NAK4_W {
        NAK4_W::new(self)
    }
    #[doc = "Bit 5 - Endpoint 5 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak5(&mut self) -> NAK5_W {
        NAK5_W::new(self)
    }
    #[doc = "Bit 6 - Endpoint 6 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak6(&mut self) -> NAK6_W {
        NAK6_W::new(self)
    }
    #[doc = "Bit 7 - Endpoint 7 NAK Interrupt Flag"]
    #[inline(always)]
    pub fn nak7(&mut self) -> NAK7_W {
        NAK7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB IN Endpoint NAK Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nak_int](index.html) module"]
pub struct NAK_INT_SPEC;
impl crate::RegisterSpec for NAK_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nak_int::R](R) reader structure"]
impl crate::Readable for NAK_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nak_int::W](W) writer structure"]
impl crate::Writable for NAK_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NAK_INT to value 0"]
impl crate::Resettable for NAK_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
