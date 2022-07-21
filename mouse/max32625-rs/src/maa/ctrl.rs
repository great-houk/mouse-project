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
#[doc = "Field `start` reader - Start MAA Calculation"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `start` writer - Start MAA Calculation"]
pub type START_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Select Operation Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPSEL_A {
    #[doc = "0: Exponentiation."]
    EXP = 0,
    #[doc = "1: Square operation."]
    SQR = 1,
    #[doc = "2: Multiply."]
    MUL = 2,
    #[doc = "3: Square operation followed by multiply."]
    SQRMUL = 3,
    #[doc = "4: Addition."]
    ADD = 4,
    #[doc = "5: Subtraction."]
    SUB = 5,
}
impl From<OPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `opsel` reader - Select Operation Type"]
pub type OPSEL_R = crate::FieldReader<u8, OPSEL_A>;
impl OPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPSEL_A> {
        match self.bits {
            0 => Some(OPSEL_A::EXP),
            1 => Some(OPSEL_A::SQR),
            2 => Some(OPSEL_A::MUL),
            3 => Some(OPSEL_A::SQRMUL),
            4 => Some(OPSEL_A::ADD),
            5 => Some(OPSEL_A::SUB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXP`"]
    #[inline(always)]
    pub fn is_exp(&self) -> bool {
        *self == OPSEL_A::EXP
    }
    #[doc = "Checks if the value of the field is `SQR`"]
    #[inline(always)]
    pub fn is_sqr(&self) -> bool {
        *self == OPSEL_A::SQR
    }
    #[doc = "Checks if the value of the field is `MUL`"]
    #[inline(always)]
    pub fn is_mul(&self) -> bool {
        *self == OPSEL_A::MUL
    }
    #[doc = "Checks if the value of the field is `SQRMUL`"]
    #[inline(always)]
    pub fn is_sqrmul(&self) -> bool {
        *self == OPSEL_A::SQRMUL
    }
    #[doc = "Checks if the value of the field is `ADD`"]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == OPSEL_A::ADD
    }
    #[doc = "Checks if the value of the field is `SUB`"]
    #[inline(always)]
    pub fn is_sub(&self) -> bool {
        *self == OPSEL_A::SUB
    }
}
#[doc = "Field `opsel` writer - Select Operation Type"]
pub type OPSEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, OPSEL_A, 3, 1>;
impl<'a> OPSEL_W<'a> {
    #[doc = "Exponentiation."]
    #[inline(always)]
    pub fn exp(self) -> &'a mut W {
        self.variant(OPSEL_A::EXP)
    }
    #[doc = "Square operation."]
    #[inline(always)]
    pub fn sqr(self) -> &'a mut W {
        self.variant(OPSEL_A::SQR)
    }
    #[doc = "Multiply."]
    #[inline(always)]
    pub fn mul(self) -> &'a mut W {
        self.variant(OPSEL_A::MUL)
    }
    #[doc = "Square operation followed by multiply."]
    #[inline(always)]
    pub fn sqrmul(self) -> &'a mut W {
        self.variant(OPSEL_A::SQRMUL)
    }
    #[doc = "Addition."]
    #[inline(always)]
    pub fn add(self) -> &'a mut W {
        self.variant(OPSEL_A::ADD)
    }
    #[doc = "Subtraction."]
    #[inline(always)]
    pub fn sub(self) -> &'a mut W {
        self.variant(OPSEL_A::SUB)
    }
}
#[doc = "Field `ocalc` reader - Optimized Calculation Control"]
pub type OCALC_R = crate::BitReader<bool>;
#[doc = "Field `ocalc` writer - Optimized Calculation Control"]
pub type OCALC_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 4>;
#[doc = "Field `if_done` reader - Interrupt Flag - Calculation Done"]
pub type IF_DONE_R = crate::BitReader<bool>;
#[doc = "Field `if_done` writer - Interrupt Flag - Calculation Done"]
pub type IF_DONE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 5>;
#[doc = "Field `inten` reader - MAA Interrupt Enable"]
pub type INTEN_R = crate::BitReader<bool>;
#[doc = "Field `inten` writer - MAA Interrupt Enable"]
pub type INTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 6>;
#[doc = "Field `if_error` reader - Interrupt Flag - Error"]
pub type IF_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `if_error` writer - Interrupt Flag - Error"]
pub type IF_ERROR_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
#[doc = "Field `ofs_a` reader - Operand A Memory Offset Select"]
pub type OFS_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ofs_a` writer - Operand A Memory Offset Select"]
pub type OFS_A_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 8>;
#[doc = "Field `ofs_b` reader - Operand B Memory Offset Select"]
pub type OFS_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ofs_b` writer - Operand B Memory Offset Select"]
pub type OFS_B_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 10>;
#[doc = "Field `ofs_exp` reader - Exponent Memory Offset Select"]
pub type OFS_EXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ofs_exp` writer - Exponent Memory Offset Select"]
pub type OFS_EXP_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 12>;
#[doc = "Field `ofs_mod` reader - Modulus Memory Select"]
pub type OFS_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ofs_mod` writer - Modulus Memory Select"]
pub type OFS_MOD_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 14>;
#[doc = "Field `seg_a` reader - Operand A Memory Segment Select"]
pub type SEG_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `seg_a` writer - Operand A Memory Segment Select"]
pub type SEG_A_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 16>;
#[doc = "Field `seg_b` reader - Operand B Memory Segment Select"]
pub type SEG_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `seg_b` writer - Operand B Memory Segment Select"]
pub type SEG_B_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 20>;
#[doc = "Field `seg_res` reader - Result Memory Segment Select"]
pub type SEG_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `seg_res` writer - Result Memory Segment Select"]
pub type SEG_RES_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 24>;
#[doc = "Field `seg_tmp` reader - Temporary Memory Segment Select"]
pub type SEG_TMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `seg_tmp` writer - Temporary Memory Segment Select"]
pub type SEG_TMP_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bit 0 - Start MAA Calculation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Select Operation Type"]
    #[inline(always)]
    pub fn opsel(&self) -> OPSEL_R {
        OPSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Optimized Calculation Control"]
    #[inline(always)]
    pub fn ocalc(&self) -> OCALC_R {
        OCALC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Flag - Calculation Done"]
    #[inline(always)]
    pub fn if_done(&self) -> IF_DONE_R {
        IF_DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MAA Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Flag - Error"]
    #[inline(always)]
    pub fn if_error(&self) -> IF_ERROR_R {
        IF_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Operand A Memory Offset Select"]
    #[inline(always)]
    pub fn ofs_a(&self) -> OFS_A_R {
        OFS_A_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Operand B Memory Offset Select"]
    #[inline(always)]
    pub fn ofs_b(&self) -> OFS_B_R {
        OFS_B_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Exponent Memory Offset Select"]
    #[inline(always)]
    pub fn ofs_exp(&self) -> OFS_EXP_R {
        OFS_EXP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Modulus Memory Select"]
    #[inline(always)]
    pub fn ofs_mod(&self) -> OFS_MOD_R {
        OFS_MOD_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Operand A Memory Segment Select"]
    #[inline(always)]
    pub fn seg_a(&self) -> SEG_A_R {
        SEG_A_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Operand B Memory Segment Select"]
    #[inline(always)]
    pub fn seg_b(&self) -> SEG_B_R {
        SEG_B_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Result Memory Segment Select"]
    #[inline(always)]
    pub fn seg_res(&self) -> SEG_RES_R {
        SEG_RES_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Temporary Memory Segment Select"]
    #[inline(always)]
    pub fn seg_tmp(&self) -> SEG_TMP_R {
        SEG_TMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start MAA Calculation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bits 1:3 - Select Operation Type"]
    #[inline(always)]
    pub fn opsel(&mut self) -> OPSEL_W {
        OPSEL_W::new(self)
    }
    #[doc = "Bit 4 - Optimized Calculation Control"]
    #[inline(always)]
    pub fn ocalc(&mut self) -> OCALC_W {
        OCALC_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Flag - Calculation Done"]
    #[inline(always)]
    pub fn if_done(&mut self) -> IF_DONE_W {
        IF_DONE_W::new(self)
    }
    #[doc = "Bit 6 - MAA Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Flag - Error"]
    #[inline(always)]
    pub fn if_error(&mut self) -> IF_ERROR_W {
        IF_ERROR_W::new(self)
    }
    #[doc = "Bits 8:9 - Operand A Memory Offset Select"]
    #[inline(always)]
    pub fn ofs_a(&mut self) -> OFS_A_W {
        OFS_A_W::new(self)
    }
    #[doc = "Bits 10:11 - Operand B Memory Offset Select"]
    #[inline(always)]
    pub fn ofs_b(&mut self) -> OFS_B_W {
        OFS_B_W::new(self)
    }
    #[doc = "Bits 12:13 - Exponent Memory Offset Select"]
    #[inline(always)]
    pub fn ofs_exp(&mut self) -> OFS_EXP_W {
        OFS_EXP_W::new(self)
    }
    #[doc = "Bits 14:15 - Modulus Memory Select"]
    #[inline(always)]
    pub fn ofs_mod(&mut self) -> OFS_MOD_W {
        OFS_MOD_W::new(self)
    }
    #[doc = "Bits 16:19 - Operand A Memory Segment Select"]
    #[inline(always)]
    pub fn seg_a(&mut self) -> SEG_A_W {
        SEG_A_W::new(self)
    }
    #[doc = "Bits 20:23 - Operand B Memory Segment Select"]
    #[inline(always)]
    pub fn seg_b(&mut self) -> SEG_B_W {
        SEG_B_W::new(self)
    }
    #[doc = "Bits 24:27 - Result Memory Segment Select"]
    #[inline(always)]
    pub fn seg_res(&mut self) -> SEG_RES_W {
        SEG_RES_W::new(self)
    }
    #[doc = "Bits 28:31 - Temporary Memory Segment Select"]
    #[inline(always)]
    pub fn seg_tmp(&mut self) -> SEG_TMP_W {
        SEG_TMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAA Control, Configuration and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
