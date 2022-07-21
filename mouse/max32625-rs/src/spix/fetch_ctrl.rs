#[doc = "Register `FETCH_CTRL` reader"]
pub struct R(crate::R<FETCH_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FETCH_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FETCH_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FETCH_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FETCH_CTRL` writer"]
pub struct W(crate::W<FETCH_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FETCH_CTRL_SPEC>;
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
impl From<crate::W<FETCH_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FETCH_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cmd_value` reader - Command Value"]
pub type CMD_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cmd_value` writer - Command Value"]
pub type CMD_VALUE_W<'a> = crate::FieldWriter<'a, u32, FETCH_CTRL_SPEC, u8, u8, 8, 0>;
#[doc = "Command Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_WIDTH_A {
    #[doc = "0: Single I/O used for Tx/Rx."]
    SINGLE = 0,
    #[doc = "1: Dual I/O lines used for Tx/Rx."]
    DUAL_IO = 1,
    #[doc = "2: Quad I/O lines used for Tx/Rx."]
    QUAD_IO = 2,
}
impl From<CMD_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `cmd_width` reader - Command Width"]
pub type CMD_WIDTH_R = crate::FieldReader<u8, CMD_WIDTH_A>;
impl CMD_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_WIDTH_A> {
        match self.bits {
            0 => Some(CMD_WIDTH_A::SINGLE),
            1 => Some(CMD_WIDTH_A::DUAL_IO),
            2 => Some(CMD_WIDTH_A::QUAD_IO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CMD_WIDTH_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == CMD_WIDTH_A::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == CMD_WIDTH_A::QUAD_IO
    }
}
#[doc = "Field `cmd_width` writer - Command Width"]
pub type CMD_WIDTH_W<'a> = crate::FieldWriter<'a, u32, FETCH_CTRL_SPEC, u8, CMD_WIDTH_A, 2, 8>;
impl<'a> CMD_WIDTH_W<'a> {
    #[doc = "Single I/O used for Tx/Rx."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CMD_WIDTH_A::SINGLE)
    }
    #[doc = "Dual I/O lines used for Tx/Rx."]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(CMD_WIDTH_A::DUAL_IO)
    }
    #[doc = "Quad I/O lines used for Tx/Rx."]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(CMD_WIDTH_A::QUAD_IO)
    }
}
#[doc = "Address Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADDR_WIDTH_A {
    #[doc = "0: Single I/O used for Tx/Rx."]
    SINGLE = 0,
    #[doc = "1: Dual I/O lines used for Tx/Rx."]
    DUAL_IO = 1,
    #[doc = "2: Quad I/O lines used for Tx/Rx."]
    QUAD_IO = 2,
}
impl From<ADDR_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDR_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `addr_width` reader - Address Width"]
pub type ADDR_WIDTH_R = crate::FieldReader<u8, ADDR_WIDTH_A>;
impl ADDR_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDR_WIDTH_A> {
        match self.bits {
            0 => Some(ADDR_WIDTH_A::SINGLE),
            1 => Some(ADDR_WIDTH_A::DUAL_IO),
            2 => Some(ADDR_WIDTH_A::QUAD_IO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ADDR_WIDTH_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == ADDR_WIDTH_A::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == ADDR_WIDTH_A::QUAD_IO
    }
}
#[doc = "Field `addr_width` writer - Address Width"]
pub type ADDR_WIDTH_W<'a> = crate::FieldWriter<'a, u32, FETCH_CTRL_SPEC, u8, ADDR_WIDTH_A, 2, 10>;
impl<'a> ADDR_WIDTH_W<'a> {
    #[doc = "Single I/O used for Tx/Rx."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ADDR_WIDTH_A::SINGLE)
    }
    #[doc = "Dual I/O lines used for Tx/Rx."]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(ADDR_WIDTH_A::DUAL_IO)
    }
    #[doc = "Quad I/O lines used for Tx/Rx."]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(ADDR_WIDTH_A::QUAD_IO)
    }
}
#[doc = "Data Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATA_WIDTH_A {
    #[doc = "0: Single I/O used for Tx/Rx."]
    SINGLE = 0,
    #[doc = "1: Dual I/O lines used for Tx/Rx."]
    DUAL_IO = 1,
    #[doc = "2: Quad I/O lines used for Tx/Rx."]
    QUAD_IO = 2,
}
impl From<DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `data_width` reader - Data Width"]
pub type DATA_WIDTH_R = crate::FieldReader<u8, DATA_WIDTH_A>;
impl DATA_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATA_WIDTH_A> {
        match self.bits {
            0 => Some(DATA_WIDTH_A::SINGLE),
            1 => Some(DATA_WIDTH_A::DUAL_IO),
            2 => Some(DATA_WIDTH_A::QUAD_IO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DATA_WIDTH_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == DATA_WIDTH_A::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == DATA_WIDTH_A::QUAD_IO
    }
}
#[doc = "Field `data_width` writer - Data Width"]
pub type DATA_WIDTH_W<'a> = crate::FieldWriter<'a, u32, FETCH_CTRL_SPEC, u8, DATA_WIDTH_A, 2, 12>;
impl<'a> DATA_WIDTH_W<'a> {
    #[doc = "Single I/O used for Tx/Rx."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::SINGLE)
    }
    #[doc = "Dual I/O lines used for Tx/Rx."]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::DUAL_IO)
    }
    #[doc = "Quad I/O lines used for Tx/Rx."]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::QUAD_IO)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command Value"]
    #[inline(always)]
    pub fn cmd_value(&self) -> CMD_VALUE_R {
        CMD_VALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Command Width"]
    #[inline(always)]
    pub fn cmd_width(&self) -> CMD_WIDTH_R {
        CMD_WIDTH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Address Width"]
    #[inline(always)]
    pub fn addr_width(&self) -> ADDR_WIDTH_R {
        ADDR_WIDTH_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Data Width"]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command Value"]
    #[inline(always)]
    pub fn cmd_value(&mut self) -> CMD_VALUE_W {
        CMD_VALUE_W::new(self)
    }
    #[doc = "Bits 8:9 - Command Width"]
    #[inline(always)]
    pub fn cmd_width(&mut self) -> CMD_WIDTH_W {
        CMD_WIDTH_W::new(self)
    }
    #[doc = "Bits 10:11 - Address Width"]
    #[inline(always)]
    pub fn addr_width(&mut self) -> ADDR_WIDTH_W {
        ADDR_WIDTH_W::new(self)
    }
    #[doc = "Bits 12:13 - Data Width"]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIX Fetch Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetch_ctrl](index.html) module"]
pub struct FETCH_CTRL_SPEC;
impl crate::RegisterSpec for FETCH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fetch_ctrl::R](R) reader structure"]
impl crate::Readable for FETCH_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fetch_ctrl::W](W) writer structure"]
impl crate::Writable for FETCH_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FETCH_CTRL to value 0"]
impl crate::Resettable for FETCH_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
