#[doc = "Reader of register ctrlr0"]
pub type R = crate::R<u32, super::CTRLR0>;
#[doc = "Writer for register ctrlr0"]
pub type W = crate::W<u32, super::CTRLR0>;
#[doc = "Register ctrlr0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WORK_MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORK_MODE_A {
    #[doc = "0: MODE_0"]
    MODE0,
    #[doc = "1: MODE_1"]
    MODE1,
    #[doc = "2: MODE_2"]
    MODE2,
    #[doc = "3: MODE_3"]
    MODE3,
}
impl From<WORK_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WORK_MODE_A) -> Self {
        match variant {
            WORK_MODE_A::MODE0 => 0,
            WORK_MODE_A::MODE1 => 1,
            WORK_MODE_A::MODE2 => 2,
            WORK_MODE_A::MODE3 => 3,
        }
    }
}
#[doc = "Reader of field `work_mode`"]
pub type WORK_MODE_R = crate::R<u8, WORK_MODE_A>;
impl WORK_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORK_MODE_A {
        match self.bits {
            0 => WORK_MODE_A::MODE0,
            1 => WORK_MODE_A::MODE1,
            2 => WORK_MODE_A::MODE2,
            3 => WORK_MODE_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == WORK_MODE_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == WORK_MODE_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == WORK_MODE_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == WORK_MODE_A::MODE3
    }
}
#[doc = "Write proxy for field `work_mode`"]
pub struct WORK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WORK_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORK_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MODE_0"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(WORK_MODE_A::MODE0)
    }
    #[doc = "MODE_1"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(WORK_MODE_A::MODE1)
    }
    #[doc = "MODE_2"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(WORK_MODE_A::MODE2)
    }
    #[doc = "MODE_3"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(WORK_MODE_A::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "TRANSFER_MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMOD_A {
    #[doc = "0: TRANS_RECV"]
    TRANS_RECV,
    #[doc = "1: TRANS"]
    TRANS,
    #[doc = "2: RECV"]
    RECV,
    #[doc = "3: EEROM"]
    EEROM,
}
impl From<TMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TMOD_A) -> Self {
        match variant {
            TMOD_A::TRANS_RECV => 0,
            TMOD_A::TRANS => 1,
            TMOD_A::RECV => 2,
            TMOD_A::EEROM => 3,
        }
    }
}
#[doc = "Reader of field `tmod`"]
pub type TMOD_R = crate::R<u8, TMOD_A>;
impl TMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMOD_A {
        match self.bits {
            0 => TMOD_A::TRANS_RECV,
            1 => TMOD_A::TRANS,
            2 => TMOD_A::RECV,
            3 => TMOD_A::EEROM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRANS_RECV`"]
    #[inline(always)]
    pub fn is_trans_recv(&self) -> bool {
        *self == TMOD_A::TRANS_RECV
    }
    #[doc = "Checks if the value of the field is `TRANS`"]
    #[inline(always)]
    pub fn is_trans(&self) -> bool {
        *self == TMOD_A::TRANS
    }
    #[doc = "Checks if the value of the field is `RECV`"]
    #[inline(always)]
    pub fn is_recv(&self) -> bool {
        *self == TMOD_A::RECV
    }
    #[doc = "Checks if the value of the field is `EEROM`"]
    #[inline(always)]
    pub fn is_eerom(&self) -> bool {
        *self == TMOD_A::EEROM
    }
}
#[doc = "Write proxy for field `tmod`"]
pub struct TMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "TRANS_RECV"]
    #[inline(always)]
    pub fn trans_recv(self) -> &'a mut W {
        self.variant(TMOD_A::TRANS_RECV)
    }
    #[doc = "TRANS"]
    #[inline(always)]
    pub fn trans(self) -> &'a mut W {
        self.variant(TMOD_A::TRANS)
    }
    #[doc = "RECV"]
    #[inline(always)]
    pub fn recv(self) -> &'a mut W {
        self.variant(TMOD_A::RECV)
    }
    #[doc = "EEROM"]
    #[inline(always)]
    pub fn eerom(self) -> &'a mut W {
        self.variant(TMOD_A::EEROM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "FRAME_FORMAT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_FORMAT_A {
    #[doc = "0: STANDARD"]
    STANDARD,
    #[doc = "1: DUAL"]
    DUAL,
    #[doc = "2: QUAD"]
    QUAD,
    #[doc = "3: OCTAL"]
    OCTAL,
}
impl From<FRAME_FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FRAME_FORMAT_A) -> Self {
        match variant {
            FRAME_FORMAT_A::STANDARD => 0,
            FRAME_FORMAT_A::DUAL => 1,
            FRAME_FORMAT_A::QUAD => 2,
            FRAME_FORMAT_A::OCTAL => 3,
        }
    }
}
#[doc = "Reader of field `frame_format`"]
pub type FRAME_FORMAT_R = crate::R<u8, FRAME_FORMAT_A>;
impl FRAME_FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAME_FORMAT_A {
        match self.bits {
            0 => FRAME_FORMAT_A::STANDARD,
            1 => FRAME_FORMAT_A::DUAL,
            2 => FRAME_FORMAT_A::QUAD,
            3 => FRAME_FORMAT_A::OCTAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == FRAME_FORMAT_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == FRAME_FORMAT_A::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == FRAME_FORMAT_A::QUAD
    }
    #[doc = "Checks if the value of the field is `OCTAL`"]
    #[inline(always)]
    pub fn is_octal(&self) -> bool {
        *self == FRAME_FORMAT_A::OCTAL
    }
}
#[doc = "Write proxy for field `frame_format`"]
pub struct FRAME_FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAME_FORMAT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "STANDARD"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(FRAME_FORMAT_A::STANDARD)
    }
    #[doc = "DUAL"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(FRAME_FORMAT_A::DUAL)
    }
    #[doc = "QUAD"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(FRAME_FORMAT_A::QUAD)
    }
    #[doc = "OCTAL"]
    #[inline(always)]
    pub fn octal(self) -> &'a mut W {
        self.variant(FRAME_FORMAT_A::OCTAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `data_length`"]
pub type DATA_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `data_length`"]
pub struct DATA_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - WORK_MODE"]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - TRANSFER_MODE"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 21:22 - FRAME_FORMAT"]
    #[inline(always)]
    pub fn frame_format(&self) -> FRAME_FORMAT_R {
        FRAME_FORMAT_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - DATA_BIT_LENGTH"]
    #[inline(always)]
    pub fn data_length(&self) -> DATA_LENGTH_R {
        DATA_LENGTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - WORK_MODE"]
    #[inline(always)]
    pub fn work_mode(&mut self) -> WORK_MODE_W {
        WORK_MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - TRANSFER_MODE"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W {
        TMOD_W { w: self }
    }
    #[doc = "Bits 21:22 - FRAME_FORMAT"]
    #[inline(always)]
    pub fn frame_format(&mut self) -> FRAME_FORMAT_W {
        FRAME_FORMAT_W { w: self }
    }
    #[doc = "Bits 16:20 - DATA_BIT_LENGTH"]
    #[inline(always)]
    pub fn data_length(&mut self) -> DATA_LENGTH_W {
        DATA_LENGTH_W { w: self }
    }
}
