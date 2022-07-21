#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `byte0` reader - Updated Byte 0"]
pub type BYTE0_R = crate::BitReader<bool>;
#[doc = "Field `byte0` writer - Updated Byte 0"]
pub type BYTE0_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 0>;
#[doc = "Field `byte1` reader - Updated Byte 1"]
pub type BYTE1_R = crate::BitReader<bool>;
#[doc = "Field `byte1` writer - Updated Byte 1"]
pub type BYTE1_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 1>;
#[doc = "Field `byte2` reader - Updated Byte 2"]
pub type BYTE2_R = crate::BitReader<bool>;
#[doc = "Field `byte2` writer - Updated Byte 2"]
pub type BYTE2_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 2>;
#[doc = "Field `byte3` reader - Updated Byte 3"]
pub type BYTE3_R = crate::BitReader<bool>;
#[doc = "Field `byte3` writer - Updated Byte 3"]
pub type BYTE3_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 3>;
#[doc = "Field `byte4` reader - Updated Byte 4"]
pub type BYTE4_R = crate::BitReader<bool>;
#[doc = "Field `byte4` writer - Updated Byte 4"]
pub type BYTE4_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 4>;
#[doc = "Field `byte5` reader - Updated Byte 5"]
pub type BYTE5_R = crate::BitReader<bool>;
#[doc = "Field `byte5` writer - Updated Byte 5"]
pub type BYTE5_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 5>;
#[doc = "Field `byte6` reader - Updated Byte 6"]
pub type BYTE6_R = crate::BitReader<bool>;
#[doc = "Field `byte6` writer - Updated Byte 6"]
pub type BYTE6_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 6>;
#[doc = "Field `byte7` reader - Updated Byte 7"]
pub type BYTE7_R = crate::BitReader<bool>;
#[doc = "Field `byte7` writer - Updated Byte 7"]
pub type BYTE7_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 7>;
#[doc = "Field `byte8` reader - Updated Byte 8"]
pub type BYTE8_R = crate::BitReader<bool>;
#[doc = "Field `byte8` writer - Updated Byte 8"]
pub type BYTE8_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 8>;
#[doc = "Field `byte9` reader - Updated Byte 9"]
pub type BYTE9_R = crate::BitReader<bool>;
#[doc = "Field `byte9` writer - Updated Byte 9"]
pub type BYTE9_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 9>;
#[doc = "Field `byte10` reader - Updated Byte 10"]
pub type BYTE10_R = crate::BitReader<bool>;
#[doc = "Field `byte10` writer - Updated Byte 10"]
pub type BYTE10_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 10>;
#[doc = "Field `byte11` reader - Updated Byte 11"]
pub type BYTE11_R = crate::BitReader<bool>;
#[doc = "Field `byte11` writer - Updated Byte 11"]
pub type BYTE11_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 11>;
#[doc = "Field `byte12` reader - Updated Byte 12"]
pub type BYTE12_R = crate::BitReader<bool>;
#[doc = "Field `byte12` writer - Updated Byte 12"]
pub type BYTE12_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 12>;
#[doc = "Field `byte13` reader - Updated Byte 13"]
pub type BYTE13_R = crate::BitReader<bool>;
#[doc = "Field `byte13` writer - Updated Byte 13"]
pub type BYTE13_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 13>;
#[doc = "Field `byte14` reader - Updated Byte 14"]
pub type BYTE14_R = crate::BitReader<bool>;
#[doc = "Field `byte14` writer - Updated Byte 14"]
pub type BYTE14_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 14>;
#[doc = "Field `byte15` reader - Updated Byte 15"]
pub type BYTE15_R = crate::BitReader<bool>;
#[doc = "Field `byte15` writer - Updated Byte 15"]
pub type BYTE15_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 15>;
#[doc = "Field `byte16` reader - Updated Byte 16"]
pub type BYTE16_R = crate::BitReader<bool>;
#[doc = "Field `byte16` writer - Updated Byte 16"]
pub type BYTE16_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 16>;
#[doc = "Field `byte17` reader - Updated Byte 17"]
pub type BYTE17_R = crate::BitReader<bool>;
#[doc = "Field `byte17` writer - Updated Byte 17"]
pub type BYTE17_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 17>;
#[doc = "Field `byte18` reader - Updated Byte 18"]
pub type BYTE18_R = crate::BitReader<bool>;
#[doc = "Field `byte18` writer - Updated Byte 18"]
pub type BYTE18_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 18>;
#[doc = "Field `byte19` reader - Updated Byte 19"]
pub type BYTE19_R = crate::BitReader<bool>;
#[doc = "Field `byte19` writer - Updated Byte 19"]
pub type BYTE19_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 19>;
#[doc = "Field `byte20` reader - Updated Byte 20"]
pub type BYTE20_R = crate::BitReader<bool>;
#[doc = "Field `byte20` writer - Updated Byte 20"]
pub type BYTE20_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 20>;
#[doc = "Field `byte21` reader - Updated Byte 21"]
pub type BYTE21_R = crate::BitReader<bool>;
#[doc = "Field `byte21` writer - Updated Byte 21"]
pub type BYTE21_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 21>;
#[doc = "Field `byte22` reader - Updated Byte 22"]
pub type BYTE22_R = crate::BitReader<bool>;
#[doc = "Field `byte22` writer - Updated Byte 22"]
pub type BYTE22_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 22>;
#[doc = "Field `byte23` reader - Updated Byte 23"]
pub type BYTE23_R = crate::BitReader<bool>;
#[doc = "Field `byte23` writer - Updated Byte 23"]
pub type BYTE23_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 23>;
#[doc = "Field `byte24` reader - Updated Byte 24"]
pub type BYTE24_R = crate::BitReader<bool>;
#[doc = "Field `byte24` writer - Updated Byte 24"]
pub type BYTE24_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 24>;
#[doc = "Field `byte25` reader - Updated Byte 25"]
pub type BYTE25_R = crate::BitReader<bool>;
#[doc = "Field `byte25` writer - Updated Byte 25"]
pub type BYTE25_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 25>;
#[doc = "Field `byte26` reader - Updated Byte 26"]
pub type BYTE26_R = crate::BitReader<bool>;
#[doc = "Field `byte26` writer - Updated Byte 26"]
pub type BYTE26_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 26>;
#[doc = "Field `byte27` reader - Updated Byte 27"]
pub type BYTE27_R = crate::BitReader<bool>;
#[doc = "Field `byte27` writer - Updated Byte 27"]
pub type BYTE27_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 27>;
#[doc = "Field `byte28` reader - Updated Byte 28"]
pub type BYTE28_R = crate::BitReader<bool>;
#[doc = "Field `byte28` writer - Updated Byte 28"]
pub type BYTE28_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 28>;
#[doc = "Field `byte29` reader - Updated Byte 29"]
pub type BYTE29_R = crate::BitReader<bool>;
#[doc = "Field `byte29` writer - Updated Byte 29"]
pub type BYTE29_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 29>;
#[doc = "Field `byte30` reader - Updated Byte 30"]
pub type BYTE30_R = crate::BitReader<bool>;
#[doc = "Field `byte30` writer - Updated Byte 30"]
pub type BYTE30_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 30>;
#[doc = "Field `byte31` reader - Updated Byte 31"]
pub type BYTE31_R = crate::BitReader<bool>;
#[doc = "Field `byte31` writer - Updated Byte 31"]
pub type BYTE31_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Updated Byte 0"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Updated Byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Updated Byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Updated Byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Updated Byte 4"]
    #[inline(always)]
    pub fn byte4(&self) -> BYTE4_R {
        BYTE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Updated Byte 5"]
    #[inline(always)]
    pub fn byte5(&self) -> BYTE5_R {
        BYTE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Updated Byte 6"]
    #[inline(always)]
    pub fn byte6(&self) -> BYTE6_R {
        BYTE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Updated Byte 7"]
    #[inline(always)]
    pub fn byte7(&self) -> BYTE7_R {
        BYTE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Updated Byte 8"]
    #[inline(always)]
    pub fn byte8(&self) -> BYTE8_R {
        BYTE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Updated Byte 9"]
    #[inline(always)]
    pub fn byte9(&self) -> BYTE9_R {
        BYTE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Updated Byte 10"]
    #[inline(always)]
    pub fn byte10(&self) -> BYTE10_R {
        BYTE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Updated Byte 11"]
    #[inline(always)]
    pub fn byte11(&self) -> BYTE11_R {
        BYTE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Updated Byte 12"]
    #[inline(always)]
    pub fn byte12(&self) -> BYTE12_R {
        BYTE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Updated Byte 13"]
    #[inline(always)]
    pub fn byte13(&self) -> BYTE13_R {
        BYTE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Updated Byte 14"]
    #[inline(always)]
    pub fn byte14(&self) -> BYTE14_R {
        BYTE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Updated Byte 15"]
    #[inline(always)]
    pub fn byte15(&self) -> BYTE15_R {
        BYTE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Updated Byte 16"]
    #[inline(always)]
    pub fn byte16(&self) -> BYTE16_R {
        BYTE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Updated Byte 17"]
    #[inline(always)]
    pub fn byte17(&self) -> BYTE17_R {
        BYTE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Updated Byte 18"]
    #[inline(always)]
    pub fn byte18(&self) -> BYTE18_R {
        BYTE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Updated Byte 19"]
    #[inline(always)]
    pub fn byte19(&self) -> BYTE19_R {
        BYTE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Updated Byte 20"]
    #[inline(always)]
    pub fn byte20(&self) -> BYTE20_R {
        BYTE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Updated Byte 21"]
    #[inline(always)]
    pub fn byte21(&self) -> BYTE21_R {
        BYTE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Updated Byte 22"]
    #[inline(always)]
    pub fn byte22(&self) -> BYTE22_R {
        BYTE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Updated Byte 23"]
    #[inline(always)]
    pub fn byte23(&self) -> BYTE23_R {
        BYTE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Updated Byte 24"]
    #[inline(always)]
    pub fn byte24(&self) -> BYTE24_R {
        BYTE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Updated Byte 25"]
    #[inline(always)]
    pub fn byte25(&self) -> BYTE25_R {
        BYTE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Updated Byte 26"]
    #[inline(always)]
    pub fn byte26(&self) -> BYTE26_R {
        BYTE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Updated Byte 27"]
    #[inline(always)]
    pub fn byte27(&self) -> BYTE27_R {
        BYTE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Updated Byte 28"]
    #[inline(always)]
    pub fn byte28(&self) -> BYTE28_R {
        BYTE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Updated Byte 29"]
    #[inline(always)]
    pub fn byte29(&self) -> BYTE29_R {
        BYTE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Updated Byte 30"]
    #[inline(always)]
    pub fn byte30(&self) -> BYTE30_R {
        BYTE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Updated Byte 31"]
    #[inline(always)]
    pub fn byte31(&self) -> BYTE31_R {
        BYTE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Updated Byte 0"]
    #[inline(always)]
    pub fn byte0(&mut self) -> BYTE0_W {
        BYTE0_W::new(self)
    }
    #[doc = "Bit 1 - Updated Byte 1"]
    #[inline(always)]
    pub fn byte1(&mut self) -> BYTE1_W {
        BYTE1_W::new(self)
    }
    #[doc = "Bit 2 - Updated Byte 2"]
    #[inline(always)]
    pub fn byte2(&mut self) -> BYTE2_W {
        BYTE2_W::new(self)
    }
    #[doc = "Bit 3 - Updated Byte 3"]
    #[inline(always)]
    pub fn byte3(&mut self) -> BYTE3_W {
        BYTE3_W::new(self)
    }
    #[doc = "Bit 4 - Updated Byte 4"]
    #[inline(always)]
    pub fn byte4(&mut self) -> BYTE4_W {
        BYTE4_W::new(self)
    }
    #[doc = "Bit 5 - Updated Byte 5"]
    #[inline(always)]
    pub fn byte5(&mut self) -> BYTE5_W {
        BYTE5_W::new(self)
    }
    #[doc = "Bit 6 - Updated Byte 6"]
    #[inline(always)]
    pub fn byte6(&mut self) -> BYTE6_W {
        BYTE6_W::new(self)
    }
    #[doc = "Bit 7 - Updated Byte 7"]
    #[inline(always)]
    pub fn byte7(&mut self) -> BYTE7_W {
        BYTE7_W::new(self)
    }
    #[doc = "Bit 8 - Updated Byte 8"]
    #[inline(always)]
    pub fn byte8(&mut self) -> BYTE8_W {
        BYTE8_W::new(self)
    }
    #[doc = "Bit 9 - Updated Byte 9"]
    #[inline(always)]
    pub fn byte9(&mut self) -> BYTE9_W {
        BYTE9_W::new(self)
    }
    #[doc = "Bit 10 - Updated Byte 10"]
    #[inline(always)]
    pub fn byte10(&mut self) -> BYTE10_W {
        BYTE10_W::new(self)
    }
    #[doc = "Bit 11 - Updated Byte 11"]
    #[inline(always)]
    pub fn byte11(&mut self) -> BYTE11_W {
        BYTE11_W::new(self)
    }
    #[doc = "Bit 12 - Updated Byte 12"]
    #[inline(always)]
    pub fn byte12(&mut self) -> BYTE12_W {
        BYTE12_W::new(self)
    }
    #[doc = "Bit 13 - Updated Byte 13"]
    #[inline(always)]
    pub fn byte13(&mut self) -> BYTE13_W {
        BYTE13_W::new(self)
    }
    #[doc = "Bit 14 - Updated Byte 14"]
    #[inline(always)]
    pub fn byte14(&mut self) -> BYTE14_W {
        BYTE14_W::new(self)
    }
    #[doc = "Bit 15 - Updated Byte 15"]
    #[inline(always)]
    pub fn byte15(&mut self) -> BYTE15_W {
        BYTE15_W::new(self)
    }
    #[doc = "Bit 16 - Updated Byte 16"]
    #[inline(always)]
    pub fn byte16(&mut self) -> BYTE16_W {
        BYTE16_W::new(self)
    }
    #[doc = "Bit 17 - Updated Byte 17"]
    #[inline(always)]
    pub fn byte17(&mut self) -> BYTE17_W {
        BYTE17_W::new(self)
    }
    #[doc = "Bit 18 - Updated Byte 18"]
    #[inline(always)]
    pub fn byte18(&mut self) -> BYTE18_W {
        BYTE18_W::new(self)
    }
    #[doc = "Bit 19 - Updated Byte 19"]
    #[inline(always)]
    pub fn byte19(&mut self) -> BYTE19_W {
        BYTE19_W::new(self)
    }
    #[doc = "Bit 20 - Updated Byte 20"]
    #[inline(always)]
    pub fn byte20(&mut self) -> BYTE20_W {
        BYTE20_W::new(self)
    }
    #[doc = "Bit 21 - Updated Byte 21"]
    #[inline(always)]
    pub fn byte21(&mut self) -> BYTE21_W {
        BYTE21_W::new(self)
    }
    #[doc = "Bit 22 - Updated Byte 22"]
    #[inline(always)]
    pub fn byte22(&mut self) -> BYTE22_W {
        BYTE22_W::new(self)
    }
    #[doc = "Bit 23 - Updated Byte 23"]
    #[inline(always)]
    pub fn byte23(&mut self) -> BYTE23_W {
        BYTE23_W::new(self)
    }
    #[doc = "Bit 24 - Updated Byte 24"]
    #[inline(always)]
    pub fn byte24(&mut self) -> BYTE24_W {
        BYTE24_W::new(self)
    }
    #[doc = "Bit 25 - Updated Byte 25"]
    #[inline(always)]
    pub fn byte25(&mut self) -> BYTE25_W {
        BYTE25_W::new(self)
    }
    #[doc = "Bit 26 - Updated Byte 26"]
    #[inline(always)]
    pub fn byte26(&mut self) -> BYTE26_W {
        BYTE26_W::new(self)
    }
    #[doc = "Bit 27 - Updated Byte 27"]
    #[inline(always)]
    pub fn byte27(&mut self) -> BYTE27_W {
        BYTE27_W::new(self)
    }
    #[doc = "Bit 28 - Updated Byte 28"]
    #[inline(always)]
    pub fn byte28(&mut self) -> BYTE28_W {
        BYTE28_W::new(self)
    }
    #[doc = "Bit 29 - Updated Byte 29"]
    #[inline(always)]
    pub fn byte29(&mut self) -> BYTE29_W {
        BYTE29_W::new(self)
    }
    #[doc = "Bit 30 - Updated Byte 30"]
    #[inline(always)]
    pub fn byte30(&mut self) -> BYTE30_W {
        BYTE30_W::new(self)
    }
    #[doc = "Bit 31 - Updated Byte 31"]
    #[inline(always)]
    pub fn byte31(&mut self) -> BYTE31_W {
        BYTE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Interrupt Enable/Disable Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
