#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Period from WDT Clear to Interrupt Flag Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INT_PERIOD_A {
    #[doc = "0: 2^31 WDT clocks"]
    _2_31_CLKS = 0,
    #[doc = "1: 2^30 WDT clocks"]
    _2_30_CLKS = 1,
    #[doc = "2: 2^29 WDT clocks"]
    _2_29_CLKS = 2,
    #[doc = "3: 2^28 WDT clocks"]
    _2_28_CLKS = 3,
    #[doc = "4: 2^27 WDT clocks"]
    _2_27_CLKS = 4,
    #[doc = "5: 2^26 WDT clocks"]
    _2_26_CLKS = 5,
    #[doc = "6: 2^25 WDT clocks"]
    _2_25_CLKS = 6,
    #[doc = "7: 2^24 WDT clocks"]
    _2_24_CLKS = 7,
    #[doc = "8: 2^23 WDT clocks"]
    _2_23_CLKS = 8,
    #[doc = "9: 2^22 WDT clocks"]
    _2_22_CLKS = 9,
    #[doc = "10: 2^21 WDT clocks"]
    _2_21_CLKS = 10,
    #[doc = "11: 2^20 WDT clocks"]
    _2_20_CLKS = 11,
    #[doc = "12: 2^19 WDT clocks"]
    _2_19_CLKS = 12,
    #[doc = "13: 2^18 WDT clocks"]
    _2_18_CLKS = 13,
    #[doc = "14: 2^17 WDT clocks"]
    _2_17_CLKS = 14,
    #[doc = "15: 2^16 WDT clocks"]
    _2_16_CLKS = 15,
}
impl From<INT_PERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: INT_PERIOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `int_period` reader - Period from WDT Clear to Interrupt Flag Set"]
pub type INT_PERIOD_R = crate::FieldReader<u8, INT_PERIOD_A>;
impl INT_PERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_PERIOD_A {
        match self.bits {
            0 => INT_PERIOD_A::_2_31_CLKS,
            1 => INT_PERIOD_A::_2_30_CLKS,
            2 => INT_PERIOD_A::_2_29_CLKS,
            3 => INT_PERIOD_A::_2_28_CLKS,
            4 => INT_PERIOD_A::_2_27_CLKS,
            5 => INT_PERIOD_A::_2_26_CLKS,
            6 => INT_PERIOD_A::_2_25_CLKS,
            7 => INT_PERIOD_A::_2_24_CLKS,
            8 => INT_PERIOD_A::_2_23_CLKS,
            9 => INT_PERIOD_A::_2_22_CLKS,
            10 => INT_PERIOD_A::_2_21_CLKS,
            11 => INT_PERIOD_A::_2_20_CLKS,
            12 => INT_PERIOD_A::_2_19_CLKS,
            13 => INT_PERIOD_A::_2_18_CLKS,
            14 => INT_PERIOD_A::_2_17_CLKS,
            15 => INT_PERIOD_A::_2_16_CLKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2_31_CLKS`"]
    #[inline(always)]
    pub fn is_2_31_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_31_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_30_CLKS`"]
    #[inline(always)]
    pub fn is_2_30_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_30_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_29_CLKS`"]
    #[inline(always)]
    pub fn is_2_29_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_29_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_28_CLKS`"]
    #[inline(always)]
    pub fn is_2_28_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_28_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_27_CLKS`"]
    #[inline(always)]
    pub fn is_2_27_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_27_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_26_CLKS`"]
    #[inline(always)]
    pub fn is_2_26_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_26_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_25_CLKS`"]
    #[inline(always)]
    pub fn is_2_25_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_25_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_24_CLKS`"]
    #[inline(always)]
    pub fn is_2_24_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_24_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_23_CLKS`"]
    #[inline(always)]
    pub fn is_2_23_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_23_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_22_CLKS`"]
    #[inline(always)]
    pub fn is_2_22_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_22_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_21_CLKS`"]
    #[inline(always)]
    pub fn is_2_21_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_21_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_20_CLKS`"]
    #[inline(always)]
    pub fn is_2_20_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_20_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_19_CLKS`"]
    #[inline(always)]
    pub fn is_2_19_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_19_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_18_CLKS`"]
    #[inline(always)]
    pub fn is_2_18_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_18_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_17_CLKS`"]
    #[inline(always)]
    pub fn is_2_17_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_17_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_16_CLKS`"]
    #[inline(always)]
    pub fn is_2_16_clks(&self) -> bool {
        *self == INT_PERIOD_A::_2_16_CLKS
    }
}
#[doc = "Field `int_period` writer - Period from WDT Clear to Interrupt Flag Set"]
pub type INT_PERIOD_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, INT_PERIOD_A, 4, 0>;
impl<'a> INT_PERIOD_W<'a> {
    #[doc = "2^31 WDT clocks"]
    #[inline(always)]
    pub fn _2_31_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_31_CLKS)
    }
    #[doc = "2^30 WDT clocks"]
    #[inline(always)]
    pub fn _2_30_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_30_CLKS)
    }
    #[doc = "2^29 WDT clocks"]
    #[inline(always)]
    pub fn _2_29_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_29_CLKS)
    }
    #[doc = "2^28 WDT clocks"]
    #[inline(always)]
    pub fn _2_28_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_28_CLKS)
    }
    #[doc = "2^27 WDT clocks"]
    #[inline(always)]
    pub fn _2_27_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_27_CLKS)
    }
    #[doc = "2^26 WDT clocks"]
    #[inline(always)]
    pub fn _2_26_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_26_CLKS)
    }
    #[doc = "2^25 WDT clocks"]
    #[inline(always)]
    pub fn _2_25_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_25_CLKS)
    }
    #[doc = "2^24 WDT clocks"]
    #[inline(always)]
    pub fn _2_24_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_24_CLKS)
    }
    #[doc = "2^23 WDT clocks"]
    #[inline(always)]
    pub fn _2_23_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_23_CLKS)
    }
    #[doc = "2^22 WDT clocks"]
    #[inline(always)]
    pub fn _2_22_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_22_CLKS)
    }
    #[doc = "2^21 WDT clocks"]
    #[inline(always)]
    pub fn _2_21_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_21_CLKS)
    }
    #[doc = "2^20 WDT clocks"]
    #[inline(always)]
    pub fn _2_20_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_20_CLKS)
    }
    #[doc = "2^19 WDT clocks"]
    #[inline(always)]
    pub fn _2_19_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_19_CLKS)
    }
    #[doc = "2^18 WDT clocks"]
    #[inline(always)]
    pub fn _2_18_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_18_CLKS)
    }
    #[doc = "2^17 WDT clocks"]
    #[inline(always)]
    pub fn _2_17_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_17_CLKS)
    }
    #[doc = "2^16 WDT clocks"]
    #[inline(always)]
    pub fn _2_16_clks(self) -> &'a mut W {
        self.variant(INT_PERIOD_A::_2_16_CLKS)
    }
}
#[doc = "Period from WDT Clear to Reset Flag Set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RST_PERIOD_A {
    #[doc = "0: 2^31 WDT clocks."]
    _2_31_CLKS = 0,
    #[doc = "1: 2^30 WDT clocks."]
    _2_30_CLKS = 1,
    #[doc = "2: 2^29 WDT clocks."]
    _2_29_CLKS = 2,
    #[doc = "3: 2^28 WDT clocks."]
    _2_28_CLKS = 3,
    #[doc = "4: 2^27 WDT clocks."]
    _2_27_CLKS = 4,
    #[doc = "5: 2^26 WDT clocks."]
    _2_26_CLKS = 5,
    #[doc = "6: 2^25 WDT clocks."]
    _2_25_CLKS = 6,
    #[doc = "7: 2^24 WDT clocks."]
    _2_24_CLKS = 7,
    #[doc = "8: 2^23 WDT clocks."]
    _2_23_CLKS = 8,
    #[doc = "9: 2^22 WDT clocks."]
    _2_22_CLKS = 9,
    #[doc = "10: 2^21 WDT clocks."]
    _2_21_CLKS = 10,
    #[doc = "11: 2^20 WDT clocks."]
    _2_20_CLKS = 11,
    #[doc = "12: 2^19 WDT clocks."]
    _2_19_CLKS = 12,
    #[doc = "13: 2^18 WDT clocks."]
    _2_18_CLKS = 13,
    #[doc = "14: 2^17 WDT clocks."]
    _2_17_CLKS = 14,
    #[doc = "15: 2^16 WDT clocks."]
    _2_16_CLKS = 15,
}
impl From<RST_PERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: RST_PERIOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `rst_period` reader - Period from WDT Clear to Reset Flag Set"]
pub type RST_PERIOD_R = crate::FieldReader<u8, RST_PERIOD_A>;
impl RST_PERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_PERIOD_A {
        match self.bits {
            0 => RST_PERIOD_A::_2_31_CLKS,
            1 => RST_PERIOD_A::_2_30_CLKS,
            2 => RST_PERIOD_A::_2_29_CLKS,
            3 => RST_PERIOD_A::_2_28_CLKS,
            4 => RST_PERIOD_A::_2_27_CLKS,
            5 => RST_PERIOD_A::_2_26_CLKS,
            6 => RST_PERIOD_A::_2_25_CLKS,
            7 => RST_PERIOD_A::_2_24_CLKS,
            8 => RST_PERIOD_A::_2_23_CLKS,
            9 => RST_PERIOD_A::_2_22_CLKS,
            10 => RST_PERIOD_A::_2_21_CLKS,
            11 => RST_PERIOD_A::_2_20_CLKS,
            12 => RST_PERIOD_A::_2_19_CLKS,
            13 => RST_PERIOD_A::_2_18_CLKS,
            14 => RST_PERIOD_A::_2_17_CLKS,
            15 => RST_PERIOD_A::_2_16_CLKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2_31_CLKS`"]
    #[inline(always)]
    pub fn is_2_31_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_31_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_30_CLKS`"]
    #[inline(always)]
    pub fn is_2_30_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_30_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_29_CLKS`"]
    #[inline(always)]
    pub fn is_2_29_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_29_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_28_CLKS`"]
    #[inline(always)]
    pub fn is_2_28_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_28_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_27_CLKS`"]
    #[inline(always)]
    pub fn is_2_27_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_27_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_26_CLKS`"]
    #[inline(always)]
    pub fn is_2_26_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_26_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_25_CLKS`"]
    #[inline(always)]
    pub fn is_2_25_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_25_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_24_CLKS`"]
    #[inline(always)]
    pub fn is_2_24_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_24_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_23_CLKS`"]
    #[inline(always)]
    pub fn is_2_23_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_23_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_22_CLKS`"]
    #[inline(always)]
    pub fn is_2_22_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_22_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_21_CLKS`"]
    #[inline(always)]
    pub fn is_2_21_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_21_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_20_CLKS`"]
    #[inline(always)]
    pub fn is_2_20_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_20_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_19_CLKS`"]
    #[inline(always)]
    pub fn is_2_19_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_19_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_18_CLKS`"]
    #[inline(always)]
    pub fn is_2_18_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_18_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_17_CLKS`"]
    #[inline(always)]
    pub fn is_2_17_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_17_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_16_CLKS`"]
    #[inline(always)]
    pub fn is_2_16_clks(&self) -> bool {
        *self == RST_PERIOD_A::_2_16_CLKS
    }
}
#[doc = "Field `rst_period` writer - Period from WDT Clear to Reset Flag Set"]
pub type RST_PERIOD_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, RST_PERIOD_A, 4, 4>;
impl<'a> RST_PERIOD_W<'a> {
    #[doc = "2^31 WDT clocks."]
    #[inline(always)]
    pub fn _2_31_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_31_CLKS)
    }
    #[doc = "2^30 WDT clocks."]
    #[inline(always)]
    pub fn _2_30_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_30_CLKS)
    }
    #[doc = "2^29 WDT clocks."]
    #[inline(always)]
    pub fn _2_29_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_29_CLKS)
    }
    #[doc = "2^28 WDT clocks."]
    #[inline(always)]
    pub fn _2_28_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_28_CLKS)
    }
    #[doc = "2^27 WDT clocks."]
    #[inline(always)]
    pub fn _2_27_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_27_CLKS)
    }
    #[doc = "2^26 WDT clocks."]
    #[inline(always)]
    pub fn _2_26_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_26_CLKS)
    }
    #[doc = "2^25 WDT clocks."]
    #[inline(always)]
    pub fn _2_25_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_25_CLKS)
    }
    #[doc = "2^24 WDT clocks."]
    #[inline(always)]
    pub fn _2_24_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_24_CLKS)
    }
    #[doc = "2^23 WDT clocks."]
    #[inline(always)]
    pub fn _2_23_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_23_CLKS)
    }
    #[doc = "2^22 WDT clocks."]
    #[inline(always)]
    pub fn _2_22_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_22_CLKS)
    }
    #[doc = "2^21 WDT clocks."]
    #[inline(always)]
    pub fn _2_21_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_21_CLKS)
    }
    #[doc = "2^20 WDT clocks."]
    #[inline(always)]
    pub fn _2_20_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_20_CLKS)
    }
    #[doc = "2^19 WDT clocks."]
    #[inline(always)]
    pub fn _2_19_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_19_CLKS)
    }
    #[doc = "2^18 WDT clocks."]
    #[inline(always)]
    pub fn _2_18_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_18_CLKS)
    }
    #[doc = "2^17 WDT clocks."]
    #[inline(always)]
    pub fn _2_17_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_17_CLKS)
    }
    #[doc = "2^16 WDT clocks."]
    #[inline(always)]
    pub fn _2_16_clks(self) -> &'a mut W {
        self.variant(RST_PERIOD_A::_2_16_CLKS)
    }
}
#[doc = "Field `en_timer` reader - Watchdg Timer Enable"]
pub type EN_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `en_timer` writer - Watchdg Timer Enable"]
pub type EN_TIMER_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 8>;
#[doc = "Watchdog Clock Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_CLOCK_A {
    #[doc = "0: WDT Clock Gate Control Disable"]
    DISABLE = 0,
    #[doc = "1: WDT Clock Gate Control Enable"]
    ENABLE = 1,
}
impl From<EN_CLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: EN_CLOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `en_clock` reader - Watchdog Clock Gate"]
pub type EN_CLOCK_R = crate::BitReader<EN_CLOCK_A>;
impl EN_CLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_CLOCK_A {
        match self.bits {
            false => EN_CLOCK_A::DISABLE,
            true => EN_CLOCK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_CLOCK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_CLOCK_A::ENABLE
    }
}
#[doc = "Field `en_clock` writer - Watchdog Clock Gate"]
pub type EN_CLOCK_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, EN_CLOCK_A, 9>;
impl<'a> EN_CLOCK_W<'a> {
    #[doc = "WDT Clock Gate Control Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_CLOCK_A::DISABLE)
    }
    #[doc = "WDT Clock Gate Control Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_CLOCK_A::ENABLE)
    }
}
#[doc = "Period from WDT Clear to Clear Window Begin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAIT_PERIOD_A {
    #[doc = "0: 2^31 WDT clocks."]
    _2_31_CLKS = 0,
    #[doc = "1: 2^30 WDT clocks."]
    _2_30_CLKS = 1,
    #[doc = "2: 2^29 WDT clocks."]
    _2_29_CLKS = 2,
    #[doc = "3: 2^28 WDT clocks."]
    _2_28_CLKS = 3,
    #[doc = "4: 2^27 WDT clocks"]
    _2_27_CLKS = 4,
    #[doc = "5: 2^26 WDT clocks"]
    _2_26_CLKS = 5,
    #[doc = "6: 2^25 WDT clocks"]
    _2_25_CLKS = 6,
    #[doc = "7: 2^24 WDT clocks"]
    _2_24_CLKS = 7,
    #[doc = "8: 2^23 WDT clocks"]
    _2_23_CLKS = 8,
    #[doc = "9: 2^22 WDT clocks."]
    _2_22_CLKS = 9,
    #[doc = "10: 2^21 WDT clocks."]
    _2_21_CLKS = 10,
    #[doc = "11: 2^20 WDT clocks."]
    _2_20_CLKS = 11,
    #[doc = "12: 2^19 WDT clocks."]
    _2_19_CLKS = 12,
    #[doc = "13: 2^18 WDT clocks."]
    _2_18_CLKS = 13,
    #[doc = "14: 2^17 WDT clocks."]
    _2_17_CLKS = 14,
    #[doc = "15: 2^16 WDT clocks."]
    _2_16_CLKS = 15,
}
impl From<WAIT_PERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: WAIT_PERIOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `wait_period` reader - Period from WDT Clear to Clear Window Begin"]
pub type WAIT_PERIOD_R = crate::FieldReader<u8, WAIT_PERIOD_A>;
impl WAIT_PERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_PERIOD_A {
        match self.bits {
            0 => WAIT_PERIOD_A::_2_31_CLKS,
            1 => WAIT_PERIOD_A::_2_30_CLKS,
            2 => WAIT_PERIOD_A::_2_29_CLKS,
            3 => WAIT_PERIOD_A::_2_28_CLKS,
            4 => WAIT_PERIOD_A::_2_27_CLKS,
            5 => WAIT_PERIOD_A::_2_26_CLKS,
            6 => WAIT_PERIOD_A::_2_25_CLKS,
            7 => WAIT_PERIOD_A::_2_24_CLKS,
            8 => WAIT_PERIOD_A::_2_23_CLKS,
            9 => WAIT_PERIOD_A::_2_22_CLKS,
            10 => WAIT_PERIOD_A::_2_21_CLKS,
            11 => WAIT_PERIOD_A::_2_20_CLKS,
            12 => WAIT_PERIOD_A::_2_19_CLKS,
            13 => WAIT_PERIOD_A::_2_18_CLKS,
            14 => WAIT_PERIOD_A::_2_17_CLKS,
            15 => WAIT_PERIOD_A::_2_16_CLKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2_31_CLKS`"]
    #[inline(always)]
    pub fn is_2_31_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_31_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_30_CLKS`"]
    #[inline(always)]
    pub fn is_2_30_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_30_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_29_CLKS`"]
    #[inline(always)]
    pub fn is_2_29_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_29_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_28_CLKS`"]
    #[inline(always)]
    pub fn is_2_28_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_28_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_27_CLKS`"]
    #[inline(always)]
    pub fn is_2_27_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_27_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_26_CLKS`"]
    #[inline(always)]
    pub fn is_2_26_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_26_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_25_CLKS`"]
    #[inline(always)]
    pub fn is_2_25_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_25_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_24_CLKS`"]
    #[inline(always)]
    pub fn is_2_24_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_24_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_23_CLKS`"]
    #[inline(always)]
    pub fn is_2_23_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_23_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_22_CLKS`"]
    #[inline(always)]
    pub fn is_2_22_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_22_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_21_CLKS`"]
    #[inline(always)]
    pub fn is_2_21_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_21_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_20_CLKS`"]
    #[inline(always)]
    pub fn is_2_20_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_20_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_19_CLKS`"]
    #[inline(always)]
    pub fn is_2_19_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_19_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_18_CLKS`"]
    #[inline(always)]
    pub fn is_2_18_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_18_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_17_CLKS`"]
    #[inline(always)]
    pub fn is_2_17_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_17_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_16_CLKS`"]
    #[inline(always)]
    pub fn is_2_16_clks(&self) -> bool {
        *self == WAIT_PERIOD_A::_2_16_CLKS
    }
}
#[doc = "Field `wait_period` writer - Period from WDT Clear to Clear Window Begin"]
pub type WAIT_PERIOD_W<'a> = crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, WAIT_PERIOD_A, 4, 12>;
impl<'a> WAIT_PERIOD_W<'a> {
    #[doc = "2^31 WDT clocks."]
    #[inline(always)]
    pub fn _2_31_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_31_CLKS)
    }
    #[doc = "2^30 WDT clocks."]
    #[inline(always)]
    pub fn _2_30_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_30_CLKS)
    }
    #[doc = "2^29 WDT clocks."]
    #[inline(always)]
    pub fn _2_29_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_29_CLKS)
    }
    #[doc = "2^28 WDT clocks."]
    #[inline(always)]
    pub fn _2_28_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_28_CLKS)
    }
    #[doc = "2^27 WDT clocks"]
    #[inline(always)]
    pub fn _2_27_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_27_CLKS)
    }
    #[doc = "2^26 WDT clocks"]
    #[inline(always)]
    pub fn _2_26_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_26_CLKS)
    }
    #[doc = "2^25 WDT clocks"]
    #[inline(always)]
    pub fn _2_25_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_25_CLKS)
    }
    #[doc = "2^24 WDT clocks"]
    #[inline(always)]
    pub fn _2_24_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_24_CLKS)
    }
    #[doc = "2^23 WDT clocks"]
    #[inline(always)]
    pub fn _2_23_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_23_CLKS)
    }
    #[doc = "2^22 WDT clocks."]
    #[inline(always)]
    pub fn _2_22_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_22_CLKS)
    }
    #[doc = "2^21 WDT clocks."]
    #[inline(always)]
    pub fn _2_21_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_21_CLKS)
    }
    #[doc = "2^20 WDT clocks."]
    #[inline(always)]
    pub fn _2_20_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_20_CLKS)
    }
    #[doc = "2^19 WDT clocks."]
    #[inline(always)]
    pub fn _2_19_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_19_CLKS)
    }
    #[doc = "2^18 WDT clocks."]
    #[inline(always)]
    pub fn _2_18_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_18_CLKS)
    }
    #[doc = "2^17 WDT clocks."]
    #[inline(always)]
    pub fn _2_17_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_17_CLKS)
    }
    #[doc = "2^16 WDT clocks."]
    #[inline(always)]
    pub fn _2_16_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIOD_A::_2_16_CLKS)
    }
}
impl R {
    #[doc = "Bits 0:3 - Period from WDT Clear to Interrupt Flag Set"]
    #[inline(always)]
    pub fn int_period(&self) -> INT_PERIOD_R {
        INT_PERIOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Period from WDT Clear to Reset Flag Set"]
    #[inline(always)]
    pub fn rst_period(&self) -> RST_PERIOD_R {
        RST_PERIOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Watchdg Timer Enable"]
    #[inline(always)]
    pub fn en_timer(&self) -> EN_TIMER_R {
        EN_TIMER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Watchdog Clock Gate"]
    #[inline(always)]
    pub fn en_clock(&self) -> EN_CLOCK_R {
        EN_CLOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Period from WDT Clear to Clear Window Begin"]
    #[inline(always)]
    pub fn wait_period(&self) -> WAIT_PERIOD_R {
        WAIT_PERIOD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Period from WDT Clear to Interrupt Flag Set"]
    #[inline(always)]
    pub fn int_period(&mut self) -> INT_PERIOD_W {
        INT_PERIOD_W::new(self)
    }
    #[doc = "Bits 4:7 - Period from WDT Clear to Reset Flag Set"]
    #[inline(always)]
    pub fn rst_period(&mut self) -> RST_PERIOD_W {
        RST_PERIOD_W::new(self)
    }
    #[doc = "Bit 8 - Watchdg Timer Enable"]
    #[inline(always)]
    pub fn en_timer(&mut self) -> EN_TIMER_W {
        EN_TIMER_W::new(self)
    }
    #[doc = "Bit 9 - Watchdog Clock Gate"]
    #[inline(always)]
    pub fn en_clock(&mut self) -> EN_CLOCK_W {
        EN_CLOCK_W::new(self)
    }
    #[doc = "Bits 12:15 - Period from WDT Clear to Clear Window Begin"]
    #[inline(always)]
    pub fn wait_period(&mut self) -> WAIT_PERIOD_W {
        WAIT_PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT0 - Watchdog Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
