#[doc = "Register `MODE_CTRL` reader"]
pub struct R(crate::R<MODE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE_CTRL` writer"]
pub struct W(crate::W<MODE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_CTRL_SPEC>;
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
impl From<crate::W<MODE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mode_clocks` reader - Mode Clocks"]
pub type MODE_CLOCKS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `mode_clocks` writer - Mode Clocks"]
pub type MODE_CLOCKS_W<'a> = crate::FieldWriter<'a, u32, MODE_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `no_cmd_mode` reader - No Command Mode"]
pub type NO_CMD_MODE_R = crate::BitReader<bool>;
#[doc = "Field `no_cmd_mode` writer - No Command Mode"]
pub type NO_CMD_MODE_W<'a> = crate::BitWriter<'a, u32, MODE_CTRL_SPEC, bool, 8>;
impl R {
    #[doc = "Bits 0:3 - Mode Clocks"]
    #[inline(always)]
    pub fn mode_clocks(&self) -> MODE_CLOCKS_R {
        MODE_CLOCKS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - No Command Mode"]
    #[inline(always)]
    pub fn no_cmd_mode(&self) -> NO_CMD_MODE_R {
        NO_CMD_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode Clocks"]
    #[inline(always)]
    pub fn mode_clocks(&mut self) -> MODE_CLOCKS_W {
        MODE_CLOCKS_W::new(self)
    }
    #[doc = "Bit 8 - No Command Mode"]
    #[inline(always)]
    pub fn no_cmd_mode(&mut self) -> NO_CMD_MODE_W {
        NO_CMD_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIX Mode Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_ctrl](index.html) module"]
pub struct MODE_CTRL_SPEC;
impl crate::RegisterSpec for MODE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode_ctrl::R](R) reader structure"]
impl crate::Readable for MODE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode_ctrl::W](W) writer structure"]
impl crate::Writable for MODE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE_CTRL to value 0"]
impl crate::Resettable for MODE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
