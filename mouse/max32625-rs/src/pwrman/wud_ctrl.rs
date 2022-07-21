#[doc = "Register `WUD_CTRL` reader"]
pub struct R(crate::R<WUD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUD_CTRL` writer"]
pub struct W(crate::W<WUD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUD_CTRL_SPEC>;
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
impl From<crate::W<WUD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pad_select` reader - Wake-Up Pad Select"]
pub type PAD_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_select` writer - Wake-Up Pad Select"]
pub type PAD_SELECT_W<'a> = crate::FieldWriter<'a, u32, WUD_CTRL_SPEC, u8, u8, 6, 0>;
#[doc = "Field `pad_mode` reader - Wake-Up Pad Signal Mode"]
pub type PAD_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pad_mode` writer - Wake-Up Pad Signal Mode"]
pub type PAD_MODE_W<'a> = crate::FieldWriter<'a, u32, WUD_CTRL_SPEC, u8, u8, 2, 8>;
#[doc = "Field `clear_all` reader - Clear All WUD Pad States"]
pub type CLEAR_ALL_R = crate::BitReader<bool>;
#[doc = "Field `clear_all` writer - Clear All WUD Pad States"]
pub type CLEAR_ALL_W<'a> = crate::BitWriter<'a, u32, WUD_CTRL_SPEC, bool, 12>;
#[doc = "Field `ctrl_enable` reader - Enable WUD Control Modification"]
pub type CTRL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ctrl_enable` writer - Enable WUD Control Modification"]
pub type CTRL_ENABLE_W<'a> = crate::BitWriter<'a, u32, WUD_CTRL_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:5 - Wake-Up Pad Select"]
    #[inline(always)]
    pub fn pad_select(&self) -> PAD_SELECT_R {
        PAD_SELECT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - Wake-Up Pad Signal Mode"]
    #[inline(always)]
    pub fn pad_mode(&self) -> PAD_MODE_R {
        PAD_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Clear All WUD Pad States"]
    #[inline(always)]
    pub fn clear_all(&self) -> CLEAR_ALL_R {
        CLEAR_ALL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable WUD Control Modification"]
    #[inline(always)]
    pub fn ctrl_enable(&self) -> CTRL_ENABLE_R {
        CTRL_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Wake-Up Pad Select"]
    #[inline(always)]
    pub fn pad_select(&mut self) -> PAD_SELECT_W {
        PAD_SELECT_W::new(self)
    }
    #[doc = "Bits 8:9 - Wake-Up Pad Signal Mode"]
    #[inline(always)]
    pub fn pad_mode(&mut self) -> PAD_MODE_W {
        PAD_MODE_W::new(self)
    }
    #[doc = "Bit 12 - Clear All WUD Pad States"]
    #[inline(always)]
    pub fn clear_all(&mut self) -> CLEAR_ALL_W {
        CLEAR_ALL_W::new(self)
    }
    #[doc = "Bit 16 - Enable WUD Control Modification"]
    #[inline(always)]
    pub fn ctrl_enable(&mut self) -> CTRL_ENABLE_W {
        CTRL_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-Up Detect Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wud_ctrl](index.html) module"]
pub struct WUD_CTRL_SPEC;
impl crate::RegisterSpec for WUD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wud_ctrl::R](R) reader structure"]
impl crate::Readable for WUD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wud_ctrl::W](W) writer structure"]
impl crate::Writable for WUD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUD_CTRL to value 0"]
impl crate::Resettable for WUD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
