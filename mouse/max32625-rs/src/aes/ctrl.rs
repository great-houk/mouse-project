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
#[doc = "Field `start` reader - AES Start/Busy"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `start` writer - AES Start/Busy"]
pub type START_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "AES Encrypt/Decrypt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPT_MODE_A {
    #[doc = "0: Perform AES encryption operation."]
    ENCRYPT_MODE = 0,
    #[doc = "1: Perform AES decryption operation."]
    DECRYPT_MODE = 1,
}
impl From<CRYPT_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: CRYPT_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `crypt_mode` reader - AES Encrypt/Decrypt Mode"]
pub type CRYPT_MODE_R = crate::BitReader<CRYPT_MODE_A>;
impl CRYPT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRYPT_MODE_A {
        match self.bits {
            false => CRYPT_MODE_A::ENCRYPT_MODE,
            true => CRYPT_MODE_A::DECRYPT_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `ENCRYPT_MODE`"]
    #[inline(always)]
    pub fn is_encrypt_mode(&self) -> bool {
        *self == CRYPT_MODE_A::ENCRYPT_MODE
    }
    #[doc = "Checks if the value of the field is `DECRYPT_MODE`"]
    #[inline(always)]
    pub fn is_decrypt_mode(&self) -> bool {
        *self == CRYPT_MODE_A::DECRYPT_MODE
    }
}
#[doc = "Field `crypt_mode` writer - AES Encrypt/Decrypt Mode"]
pub type CRYPT_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, CRYPT_MODE_A, 1>;
impl<'a> CRYPT_MODE_W<'a> {
    #[doc = "Perform AES encryption operation."]
    #[inline(always)]
    pub fn encrypt_mode(self) -> &'a mut W {
        self.variant(CRYPT_MODE_A::ENCRYPT_MODE)
    }
    #[doc = "Perform AES decryption operation."]
    #[inline(always)]
    pub fn decrypt_mode(self) -> &'a mut W {
        self.variant(CRYPT_MODE_A::DECRYPT_MODE)
    }
}
#[doc = "AES Expanded Key Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXP_KEY_MODE_A {
    #[doc = "0: Calculate new expanded key for this operation."]
    CALC_NEW_EXP_KEY = 0,
    #[doc = "1: Use expanded key calculated by the last operation."]
    USE_LAST_EXP_KEY = 1,
}
impl From<EXP_KEY_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: EXP_KEY_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `exp_key_mode` reader - AES Expanded Key Mode"]
pub type EXP_KEY_MODE_R = crate::BitReader<EXP_KEY_MODE_A>;
impl EXP_KEY_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXP_KEY_MODE_A {
        match self.bits {
            false => EXP_KEY_MODE_A::CALC_NEW_EXP_KEY,
            true => EXP_KEY_MODE_A::USE_LAST_EXP_KEY,
        }
    }
    #[doc = "Checks if the value of the field is `CALC_NEW_EXP_KEY`"]
    #[inline(always)]
    pub fn is_calc_new_exp_key(&self) -> bool {
        *self == EXP_KEY_MODE_A::CALC_NEW_EXP_KEY
    }
    #[doc = "Checks if the value of the field is `USE_LAST_EXP_KEY`"]
    #[inline(always)]
    pub fn is_use_last_exp_key(&self) -> bool {
        *self == EXP_KEY_MODE_A::USE_LAST_EXP_KEY
    }
}
#[doc = "Field `exp_key_mode` writer - AES Expanded Key Mode"]
pub type EXP_KEY_MODE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, EXP_KEY_MODE_A, 2>;
impl<'a> EXP_KEY_MODE_W<'a> {
    #[doc = "Calculate new expanded key for this operation."]
    #[inline(always)]
    pub fn calc_new_exp_key(self) -> &'a mut W {
        self.variant(EXP_KEY_MODE_A::CALC_NEW_EXP_KEY)
    }
    #[doc = "Use expanded key calculated by the last operation."]
    #[inline(always)]
    pub fn use_last_exp_key(self) -> &'a mut W {
        self.variant(EXP_KEY_MODE_A::USE_LAST_EXP_KEY)
    }
}
#[doc = "AES Key Size Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_SIZE_A {
    #[doc = "0: Use 128-bit AES key size."]
    KEY_SIZE_128 = 0,
    #[doc = "1: Use 192-bit AES key size."]
    KEY_SIZE_192 = 1,
    #[doc = "2: Use 256-bit AES key size."]
    KEY_SIZE_256 = 2,
}
impl From<KEY_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `key_size` reader - AES Key Size Select"]
pub type KEY_SIZE_R = crate::FieldReader<u8, KEY_SIZE_A>;
impl KEY_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_SIZE_A> {
        match self.bits {
            0 => Some(KEY_SIZE_A::KEY_SIZE_128),
            1 => Some(KEY_SIZE_A::KEY_SIZE_192),
            2 => Some(KEY_SIZE_A::KEY_SIZE_256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEY_SIZE_128`"]
    #[inline(always)]
    pub fn is_key_size_128(&self) -> bool {
        *self == KEY_SIZE_A::KEY_SIZE_128
    }
    #[doc = "Checks if the value of the field is `KEY_SIZE_192`"]
    #[inline(always)]
    pub fn is_key_size_192(&self) -> bool {
        *self == KEY_SIZE_A::KEY_SIZE_192
    }
    #[doc = "Checks if the value of the field is `KEY_SIZE_256`"]
    #[inline(always)]
    pub fn is_key_size_256(&self) -> bool {
        *self == KEY_SIZE_A::KEY_SIZE_256
    }
}
#[doc = "Field `key_size` writer - AES Key Size Select"]
pub type KEY_SIZE_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, KEY_SIZE_A, 2, 3>;
impl<'a> KEY_SIZE_W<'a> {
    #[doc = "Use 128-bit AES key size."]
    #[inline(always)]
    pub fn key_size_128(self) -> &'a mut W {
        self.variant(KEY_SIZE_A::KEY_SIZE_128)
    }
    #[doc = "Use 192-bit AES key size."]
    #[inline(always)]
    pub fn key_size_192(self) -> &'a mut W {
        self.variant(KEY_SIZE_A::KEY_SIZE_192)
    }
    #[doc = "Use 256-bit AES key size."]
    #[inline(always)]
    pub fn key_size_256(self) -> &'a mut W {
        self.variant(KEY_SIZE_A::KEY_SIZE_256)
    }
}
#[doc = "Field `inten` reader - AES Interrupt Enable"]
pub type INTEN_R = crate::BitReader<bool>;
#[doc = "Field `inten` writer - AES Interrupt Enable"]
pub type INTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 5>;
#[doc = "Field `intfl` reader - AES Interrupt Flag"]
pub type INTFL_R = crate::BitReader<bool>;
#[doc = "Field `intfl` writer - AES Interrupt Flag"]
pub type INTFL_W<'a> = crate::BitWriter1C<'a, u32, CTRL_SPEC, bool, 6>;
impl R {
    #[doc = "Bit 0 - AES Start/Busy"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AES Encrypt/Decrypt Mode"]
    #[inline(always)]
    pub fn crypt_mode(&self) -> CRYPT_MODE_R {
        CRYPT_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AES Expanded Key Mode"]
    #[inline(always)]
    pub fn exp_key_mode(&self) -> EXP_KEY_MODE_R {
        EXP_KEY_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - AES Key Size Select"]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - AES Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AES Interrupt Flag"]
    #[inline(always)]
    pub fn intfl(&self) -> INTFL_R {
        INTFL_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Start/Busy"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 1 - AES Encrypt/Decrypt Mode"]
    #[inline(always)]
    pub fn crypt_mode(&mut self) -> CRYPT_MODE_W {
        CRYPT_MODE_W::new(self)
    }
    #[doc = "Bit 2 - AES Expanded Key Mode"]
    #[inline(always)]
    pub fn exp_key_mode(&mut self) -> EXP_KEY_MODE_W {
        EXP_KEY_MODE_W::new(self)
    }
    #[doc = "Bits 3:4 - AES Key Size Select"]
    #[inline(always)]
    pub fn key_size(&mut self) -> KEY_SIZE_W {
        KEY_SIZE_W::new(self)
    }
    #[doc = "Bit 5 - AES Interrupt Enable"]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W::new(self)
    }
    #[doc = "Bit 6 - AES Interrupt Flag"]
    #[inline(always)]
    pub fn intfl(&mut self) -> INTFL_W {
        INTFL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
