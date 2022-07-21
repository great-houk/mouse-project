#[doc = "Register `LOOP` reader"]
pub struct R(crate::R<LOOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOOP` writer"]
pub struct W(crate::W<LOOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOOP_SPEC>;
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
impl From<crate::W<LOOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `counter_0` reader - CH1 Loop Counter 1"]
pub type COUNTER_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `counter_0` writer - CH1 Loop Counter 1"]
pub type COUNTER_0_W<'a> = crate::FieldWriter<'a, u32, LOOP_SPEC, u16, u16, 16, 0>;
#[doc = "Field `counter_1` reader - CH1 Loop Counter 0"]
pub type COUNTER_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `counter_1` writer - CH1 Loop Counter 0"]
pub type COUNTER_1_W<'a> = crate::FieldWriter<'a, u32, LOOP_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:15 - CH1 Loop Counter 1"]
    #[inline(always)]
    pub fn counter_0(&self) -> COUNTER_0_R {
        COUNTER_0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CH1 Loop Counter 0"]
    #[inline(always)]
    pub fn counter_1(&self) -> COUNTER_1_R {
        COUNTER_1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH1 Loop Counter 1"]
    #[inline(always)]
    pub fn counter_0(&mut self) -> COUNTER_0_W {
        COUNTER_0_W::new(self)
    }
    #[doc = "Bits 16:31 - CH1 Loop Counter 0"]
    #[inline(always)]
    pub fn counter_1(&mut self) -> COUNTER_1_W {
        COUNTER_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Loop Counters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loop_](index.html) module"]
pub struct LOOP_SPEC;
impl crate::RegisterSpec for LOOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loop_::R](R) reader structure"]
impl crate::Readable for LOOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loop_::W](W) writer structure"]
impl crate::Writable for LOOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOOP to value 0"]
impl crate::Resettable for LOOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
