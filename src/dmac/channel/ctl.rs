#[doc = "Reader of register ctl"]
pub type R = crate::R<u64, super::CTL>;
#[doc = "Writer for register ctl"]
pub type W = crate::W<u64, super::CTL>;
#[doc = "Register ctl `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Source master select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMS_A {
    #[doc = "0: AXI master 1"]
    AXI_MASTER_1,
    #[doc = "1: AXI master 2"]
    AXI_MASTER_2,
}
impl From<SMS_A> for bool {
    #[inline(always)]
    fn from(variant: SMS_A) -> Self {
        match variant {
            SMS_A::AXI_MASTER_1 => false,
            SMS_A::AXI_MASTER_2 => true,
        }
    }
}
#[doc = "Reader of field `sms`"]
pub type SMS_R = crate::R<bool, SMS_A>;
impl SMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMS_A {
        match self.bits {
            false => SMS_A::AXI_MASTER_1,
            true => SMS_A::AXI_MASTER_2,
        }
    }
    #[doc = "Checks if the value of the field is `AXI_MASTER_1`"]
    #[inline(always)]
    pub fn is_axi_master_1(&self) -> bool {
        *self == SMS_A::AXI_MASTER_1
    }
    #[doc = "Checks if the value of the field is `AXI_MASTER_2`"]
    #[inline(always)]
    pub fn is_axi_master_2(&self) -> bool {
        *self == SMS_A::AXI_MASTER_2
    }
}
#[doc = "Write proxy for field `sms`"]
pub struct SMS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AXI master 1"]
    #[inline(always)]
    pub fn axi_master_1(self) -> &'a mut W {
        self.variant(SMS_A::AXI_MASTER_1)
    }
    #[doc = "AXI master 2"]
    #[inline(always)]
    pub fn axi_master_2(self) -> &'a mut W {
        self.variant(SMS_A::AXI_MASTER_2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u64) & 0x01);
        self.w
    }
}
#[doc = "Destination master select"]
pub type DMS_A = SMS_A;
#[doc = "Reader of field `dms`"]
pub type DMS_R = crate::R<bool, SMS_A>;
#[doc = "Write proxy for field `dms`"]
pub struct DMS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AXI master 1"]
    #[inline(always)]
    pub fn axi_master_1(self) -> &'a mut W {
        self.variant(SMS_A::AXI_MASTER_1)
    }
    #[doc = "AXI master 2"]
    #[inline(always)]
    pub fn axi_master_2(self) -> &'a mut W {
        self.variant(SMS_A::AXI_MASTER_2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u64) & 0x01) << 2);
        self.w
    }
}
#[doc = "Source address increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINC_A {
    #[doc = "0: Increment address"]
    INCREMENT,
    #[doc = "1: Don't increment address"]
    NOCHANGE,
}
impl From<SINC_A> for bool {
    #[inline(always)]
    fn from(variant: SINC_A) -> Self {
        match variant {
            SINC_A::INCREMENT => false,
            SINC_A::NOCHANGE => true,
        }
    }
}
#[doc = "Reader of field `sinc`"]
pub type SINC_R = crate::R<bool, SINC_A>;
impl SINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINC_A {
        match self.bits {
            false => SINC_A::INCREMENT,
            true => SINC_A::NOCHANGE,
        }
    }
    #[doc = "Checks if the value of the field is `INCREMENT`"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == SINC_A::INCREMENT
    }
    #[doc = "Checks if the value of the field is `NOCHANGE`"]
    #[inline(always)]
    pub fn is_nochange(&self) -> bool {
        *self == SINC_A::NOCHANGE
    }
}
#[doc = "Write proxy for field `sinc`"]
pub struct SINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Increment address"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut W {
        self.variant(SINC_A::INCREMENT)
    }
    #[doc = "Don't increment address"]
    #[inline(always)]
    pub fn nochange(self) -> &'a mut W {
        self.variant(SINC_A::NOCHANGE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u64) & 0x01) << 4);
        self.w
    }
}
#[doc = "Destination address increment"]
pub type DINC_A = SINC_A;
#[doc = "Reader of field `dinc`"]
pub type DINC_R = crate::R<bool, SINC_A>;
#[doc = "Write proxy for field `dinc`"]
pub struct DINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Increment address"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut W {
        self.variant(SINC_A::INCREMENT)
    }
    #[doc = "Don't increment address"]
    #[inline(always)]
    pub fn nochange(self) -> &'a mut W {
        self.variant(SINC_A::NOCHANGE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u64) & 0x01) << 6);
        self.w
    }
}
#[doc = "Source transfer width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_TR_WIDTH_A {
    #[doc = "0: 8 bits"]
    WIDTH_8,
    #[doc = "1: 16 bits"]
    WIDTH_16,
    #[doc = "2: 32 bits"]
    WIDTH_32,
    #[doc = "3: 64 bits"]
    WIDTH_64,
    #[doc = "4: 128 bits"]
    WIDTH_128,
    #[doc = "5: 256 bits"]
    WIDTH_256,
    #[doc = "6: 512 bits"]
    WIDTH_512,
}
impl From<SRC_TR_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_TR_WIDTH_A) -> Self {
        match variant {
            SRC_TR_WIDTH_A::WIDTH_8 => 0,
            SRC_TR_WIDTH_A::WIDTH_16 => 1,
            SRC_TR_WIDTH_A::WIDTH_32 => 2,
            SRC_TR_WIDTH_A::WIDTH_64 => 3,
            SRC_TR_WIDTH_A::WIDTH_128 => 4,
            SRC_TR_WIDTH_A::WIDTH_256 => 5,
            SRC_TR_WIDTH_A::WIDTH_512 => 6,
        }
    }
}
#[doc = "Reader of field `src_tr_width`"]
pub type SRC_TR_WIDTH_R = crate::R<u8, SRC_TR_WIDTH_A>;
impl SRC_TR_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_TR_WIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_TR_WIDTH_A::WIDTH_8),
            1 => Val(SRC_TR_WIDTH_A::WIDTH_16),
            2 => Val(SRC_TR_WIDTH_A::WIDTH_32),
            3 => Val(SRC_TR_WIDTH_A::WIDTH_64),
            4 => Val(SRC_TR_WIDTH_A::WIDTH_128),
            5 => Val(SRC_TR_WIDTH_A::WIDTH_256),
            6 => Val(SRC_TR_WIDTH_A::WIDTH_512),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WIDTH_8`"]
    #[inline(always)]
    pub fn is_width_8(&self) -> bool {
        *self == SRC_TR_WIDTH_A::WIDTH_8
    }
    #[doc = "Checks if the value of the field is `WIDTH_16`"]
    #[inline(always)]
    pub fn is_width_16(&self) -> bool {
        *self == SRC_TR_WIDTH_A::WIDTH_16
    }
    #[doc = "Checks if the value of the field is `WIDTH_32`"]
    #[inline(always)]
    pub fn is_width_32(&self) -> bool {
        *self == SRC_TR_WIDTH_A::WIDTH_32
    }
    #[doc = "Checks if the value of the field is `WIDTH_64`"]
    #[inline(always)]
    pub fn is_width_64(&self) -> bool {
        *self == SRC_TR_WIDTH_A::WIDTH_64
    }
    #[doc = "Checks if the value of the field is `WIDTH_128`"]
    #[inline(always)]
    pub fn is_width_128(&self) -> bool {
        *self == SRC_TR_WIDTH_A::WIDTH_128
    }
    #[doc = "Checks if the value of the field is `WIDTH_256`"]
    #[inline(always)]
    pub fn is_width_256(&self) -> bool {
        *self == SRC_TR_WIDTH_A::WIDTH_256
    }
    #[doc = "Checks if the value of the field is `WIDTH_512`"]
    #[inline(always)]
    pub fn is_width_512(&self) -> bool {
        *self == SRC_TR_WIDTH_A::WIDTH_512
    }
}
#[doc = "Write proxy for field `src_tr_width`"]
pub struct SRC_TR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_TR_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_TR_WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn width_8(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_8)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn width_16(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_16)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn width_32(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_32)
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn width_64(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_64)
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn width_128(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_128)
    }
    #[doc = "256 bits"]
    #[inline(always)]
    pub fn width_256(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_256)
    }
    #[doc = "512 bits"]
    #[inline(always)]
    pub fn width_512(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u64) & 0x07) << 8);
        self.w
    }
}
#[doc = "Destination transfer width"]
pub type DST_TR_WIDTH_A = SRC_TR_WIDTH_A;
#[doc = "Reader of field `dst_tr_width`"]
pub type DST_TR_WIDTH_R = crate::R<u8, SRC_TR_WIDTH_A>;
#[doc = "Write proxy for field `dst_tr_width`"]
pub struct DST_TR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_TR_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_TR_WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn width_8(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_8)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn width_16(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_16)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn width_32(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_32)
    }
    #[doc = "64 bits"]
    #[inline(always)]
    pub fn width_64(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_64)
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn width_128(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_128)
    }
    #[doc = "256 bits"]
    #[inline(always)]
    pub fn width_256(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_256)
    }
    #[doc = "512 bits"]
    #[inline(always)]
    pub fn width_512(self) -> &'a mut W {
        self.variant(SRC_TR_WIDTH_A::WIDTH_512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u64) & 0x07) << 11);
        self.w
    }
}
#[doc = "Source burst transaction length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_MSIZE_A {
    #[doc = "0: 1 data item"]
    LENGTH_1,
    #[doc = "1: 4 data items"]
    LENGTH_4,
    #[doc = "2: 8 data items"]
    LENGTH_8,
    #[doc = "3: 16 data items"]
    LENGTH_16,
    #[doc = "4: 32 data items"]
    LENGTH_32,
    #[doc = "5: 64 data items"]
    LENGTH_64,
    #[doc = "6: 128 data items"]
    LENGTH_128,
    #[doc = "7: 256 data items"]
    LENGTH_256,
    #[doc = "8: 512 data items"]
    LENGTH_512,
    #[doc = "9: 1024 data items"]
    LENGTH_1024,
}
impl From<SRC_MSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_MSIZE_A) -> Self {
        match variant {
            SRC_MSIZE_A::LENGTH_1 => 0,
            SRC_MSIZE_A::LENGTH_4 => 1,
            SRC_MSIZE_A::LENGTH_8 => 2,
            SRC_MSIZE_A::LENGTH_16 => 3,
            SRC_MSIZE_A::LENGTH_32 => 4,
            SRC_MSIZE_A::LENGTH_64 => 5,
            SRC_MSIZE_A::LENGTH_128 => 6,
            SRC_MSIZE_A::LENGTH_256 => 7,
            SRC_MSIZE_A::LENGTH_512 => 8,
            SRC_MSIZE_A::LENGTH_1024 => 9,
        }
    }
}
#[doc = "Reader of field `src_msize`"]
pub type SRC_MSIZE_R = crate::R<u8, SRC_MSIZE_A>;
impl SRC_MSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_MSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_MSIZE_A::LENGTH_1),
            1 => Val(SRC_MSIZE_A::LENGTH_4),
            2 => Val(SRC_MSIZE_A::LENGTH_8),
            3 => Val(SRC_MSIZE_A::LENGTH_16),
            4 => Val(SRC_MSIZE_A::LENGTH_32),
            5 => Val(SRC_MSIZE_A::LENGTH_64),
            6 => Val(SRC_MSIZE_A::LENGTH_128),
            7 => Val(SRC_MSIZE_A::LENGTH_256),
            8 => Val(SRC_MSIZE_A::LENGTH_512),
            9 => Val(SRC_MSIZE_A::LENGTH_1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LENGTH_1`"]
    #[inline(always)]
    pub fn is_length_1(&self) -> bool {
        *self == SRC_MSIZE_A::LENGTH_1
    }
    #[doc = "Checks if the value of the field is `LENGTH_4`"]
    #[inline(always)]
    pub fn is_length_4(&self) -> bool {
        *self == SRC_MSIZE_A::LENGTH_4
    }
    #[doc = "Checks if the value of the field is `LENGTH_8`"]
    #[inline(always)]
    pub fn is_length_8(&self) -> bool {
        *self == SRC_MSIZE_A::LENGTH_8
    }
    #[doc = "Checks if the value of the field is `LENGTH_16`"]
    #[inline(always)]
    pub fn is_length_16(&self) -> bool {
        *self == SRC_MSIZE_A::LENGTH_16
    }
    #[doc = "Checks if the value of the field is `LENGTH_32`"]
    #[inline(always)]
    pub fn is_length_32(&self) -> bool {
        *self == SRC_MSIZE_A::LENGTH_32
    }
    #[doc = "Checks if the value of the field is `LENGTH_64`"]
    #[inline(always)]
    pub fn is_length_64(&self) -> bool {
        *self == SRC_MSIZE_A::LENGTH_64
    }
    #[doc = "Checks if the value of the field is `LENGTH_128`"]
    #[inline(always)]
    pub fn is_length_128(&self) -> bool {
        *self == SRC_MSIZE_A::LENGTH_128
    }
    #[doc = "Checks if the value of the field is `LENGTH_256`"]
    #[inline(always)]
    pub fn is_length_256(&self) -> bool {
        *self == SRC_MSIZE_A::LENGTH_256
    }
    #[doc = "Checks if the value of the field is `LENGTH_512`"]
    #[inline(always)]
    pub fn is_length_512(&self) -> bool {
        *self == SRC_MSIZE_A::LENGTH_512
    }
    #[doc = "Checks if the value of the field is `LENGTH_1024`"]
    #[inline(always)]
    pub fn is_length_1024(&self) -> bool {
        *self == SRC_MSIZE_A::LENGTH_1024
    }
}
#[doc = "Write proxy for field `src_msize`"]
pub struct SRC_MSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_MSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_MSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data item"]
    #[inline(always)]
    pub fn length_1(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_1)
    }
    #[doc = "4 data items"]
    #[inline(always)]
    pub fn length_4(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_4)
    }
    #[doc = "8 data items"]
    #[inline(always)]
    pub fn length_8(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_8)
    }
    #[doc = "16 data items"]
    #[inline(always)]
    pub fn length_16(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_16)
    }
    #[doc = "32 data items"]
    #[inline(always)]
    pub fn length_32(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_32)
    }
    #[doc = "64 data items"]
    #[inline(always)]
    pub fn length_64(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_64)
    }
    #[doc = "128 data items"]
    #[inline(always)]
    pub fn length_128(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_128)
    }
    #[doc = "256 data items"]
    #[inline(always)]
    pub fn length_256(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_256)
    }
    #[doc = "512 data items"]
    #[inline(always)]
    pub fn length_512(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_512)
    }
    #[doc = "1024 data items"]
    #[inline(always)]
    pub fn length_1024(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | (((value as u64) & 0x0f) << 14);
        self.w
    }
}
#[doc = "Destination burst transaction length"]
pub type DST_MSIZE_A = SRC_MSIZE_A;
#[doc = "Reader of field `dst_msize`"]
pub type DST_MSIZE_R = crate::R<u8, SRC_MSIZE_A>;
#[doc = "Write proxy for field `dst_msize`"]
pub struct DST_MSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_MSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_MSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data item"]
    #[inline(always)]
    pub fn length_1(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_1)
    }
    #[doc = "4 data items"]
    #[inline(always)]
    pub fn length_4(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_4)
    }
    #[doc = "8 data items"]
    #[inline(always)]
    pub fn length_8(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_8)
    }
    #[doc = "16 data items"]
    #[inline(always)]
    pub fn length_16(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_16)
    }
    #[doc = "32 data items"]
    #[inline(always)]
    pub fn length_32(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_32)
    }
    #[doc = "64 data items"]
    #[inline(always)]
    pub fn length_64(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_64)
    }
    #[doc = "128 data items"]
    #[inline(always)]
    pub fn length_128(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_128)
    }
    #[doc = "256 data items"]
    #[inline(always)]
    pub fn length_256(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_256)
    }
    #[doc = "512 data items"]
    #[inline(always)]
    pub fn length_512(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_512)
    }
    #[doc = "1024 data items"]
    #[inline(always)]
    pub fn length_1024(self) -> &'a mut W {
        self.variant(SRC_MSIZE_A::LENGTH_1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u64) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Reader of field `nonposted_lastwrite_en`"]
pub type NONPOSTED_LASTWRITE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `nonposted_lastwrite_en`"]
pub struct NONPOSTED_LASTWRITE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NONPOSTED_LASTWRITE_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u64) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `arlen_en`"]
pub type ARLEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `arlen_en`"]
pub struct ARLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARLEN_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 38)) | (((value as u64) & 0x01) << 38);
        self.w
    }
}
#[doc = "Reader of field `arlen`"]
pub type ARLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `arlen`"]
pub struct ARLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 39)) | (((value as u64) & 0xff) << 39);
        self.w
    }
}
#[doc = "Reader of field `awlen_en`"]
pub type AWLEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `awlen_en`"]
pub struct AWLEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AWLEN_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 47)) | (((value as u64) & 0x01) << 47);
        self.w
    }
}
#[doc = "Reader of field `awlen`"]
pub type AWLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `awlen`"]
pub struct AWLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AWLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 48)) | (((value as u64) & 0xff) << 48);
        self.w
    }
}
#[doc = "Reader of field `src_stat_en`"]
pub type SRC_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `src_stat_en`"]
pub struct SRC_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_STAT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 56)) | (((value as u64) & 0x01) << 56);
        self.w
    }
}
#[doc = "Reader of field `dst_stat_en`"]
pub type DST_STAT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dst_stat_en`"]
pub struct DST_STAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_STAT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 57)) | (((value as u64) & 0x01) << 57);
        self.w
    }
}
#[doc = "Reader of field `ioc_blktfr`"]
pub type IOC_BLKTFR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ioc_blktfr`"]
pub struct IOC_BLKTFR_W<'a> {
    w: &'a mut W,
}
impl<'a> IOC_BLKTFR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 58)) | (((value as u64) & 0x01) << 58);
        self.w
    }
}
#[doc = "Reader of field `shadowreg_or_lli_last`"]
pub type SHADOWREG_OR_LLI_LAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `shadowreg_or_lli_last`"]
pub struct SHADOWREG_OR_LLI_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> SHADOWREG_OR_LLI_LAST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 62)) | (((value as u64) & 0x01) << 62);
        self.w
    }
}
#[doc = "Reader of field `shadowreg_or_lli_valid`"]
pub type SHADOWREG_OR_LLI_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `shadowreg_or_lli_valid`"]
pub struct SHADOWREG_OR_LLI_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> SHADOWREG_OR_LLI_VALID_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 63)) | (((value as u64) & 0x01) << 63);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Source master select"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Destination master select"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Source address increment"]
    #[inline(always)]
    pub fn sinc(&self) -> SINC_R {
        SINC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Destination address increment"]
    #[inline(always)]
    pub fn dinc(&self) -> DINC_R {
        DINC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Source transfer width"]
    #[inline(always)]
    pub fn src_tr_width(&self) -> SRC_TR_WIDTH_R {
        SRC_TR_WIDTH_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Destination transfer width"]
    #[inline(always)]
    pub fn dst_tr_width(&self) -> DST_TR_WIDTH_R {
        DST_TR_WIDTH_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 14:17 - Source burst transaction length"]
    #[inline(always)]
    pub fn src_msize(&self) -> SRC_MSIZE_R {
        SRC_MSIZE_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Destination burst transaction length"]
    #[inline(always)]
    pub fn dst_msize(&self) -> DST_MSIZE_R {
        DST_MSIZE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Non Posted Last Write Enable (posted writes may be used till the end of the block)"]
    #[inline(always)]
    pub fn nonposted_lastwrite_en(&self) -> NONPOSTED_LASTWRITE_EN_R {
        NONPOSTED_LASTWRITE_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 38 - Source burst length enable"]
    #[inline(always)]
    pub fn arlen_en(&self) -> ARLEN_EN_R {
        ARLEN_EN_R::new(((self.bits >> 38) & 0x01) != 0)
    }
    #[doc = "Bits 39:46 - Source burst length"]
    #[inline(always)]
    pub fn arlen(&self) -> ARLEN_R {
        ARLEN_R::new(((self.bits >> 39) & 0xff) as u8)
    }
    #[doc = "Bit 47 - Destination burst length enable"]
    #[inline(always)]
    pub fn awlen_en(&self) -> AWLEN_EN_R {
        AWLEN_EN_R::new(((self.bits >> 47) & 0x01) != 0)
    }
    #[doc = "Bits 48:55 - Destination burst length"]
    #[inline(always)]
    pub fn awlen(&self) -> AWLEN_R {
        AWLEN_R::new(((self.bits >> 48) & 0xff) as u8)
    }
    #[doc = "Bit 56 - Source status enable"]
    #[inline(always)]
    pub fn src_stat_en(&self) -> SRC_STAT_EN_R {
        SRC_STAT_EN_R::new(((self.bits >> 56) & 0x01) != 0)
    }
    #[doc = "Bit 57 - Destination status enable"]
    #[inline(always)]
    pub fn dst_stat_en(&self) -> DST_STAT_EN_R {
        DST_STAT_EN_R::new(((self.bits >> 57) & 0x01) != 0)
    }
    #[doc = "Bit 58 - Interrupt completion of block transfer"]
    #[inline(always)]
    pub fn ioc_blktfr(&self) -> IOC_BLKTFR_R {
        IOC_BLKTFR_R::new(((self.bits >> 58) & 0x01) != 0)
    }
    #[doc = "Bit 62 - Last shadow linked list item (indicates shadowreg/LLI content is the last one)"]
    #[inline(always)]
    pub fn shadowreg_or_lli_last(&self) -> SHADOWREG_OR_LLI_LAST_R {
        SHADOWREG_OR_LLI_LAST_R::new(((self.bits >> 62) & 0x01) != 0)
    }
    #[doc = "Bit 63 - last shadow linked list item valid (indicate shadowreg/LLI content is valid)"]
    #[inline(always)]
    pub fn shadowreg_or_lli_valid(&self) -> SHADOWREG_OR_LLI_VALID_R {
        SHADOWREG_OR_LLI_VALID_R::new(((self.bits >> 63) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source master select"]
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W {
        SMS_W { w: self }
    }
    #[doc = "Bit 2 - Destination master select"]
    #[inline(always)]
    pub fn dms(&mut self) -> DMS_W {
        DMS_W { w: self }
    }
    #[doc = "Bit 4 - Source address increment"]
    #[inline(always)]
    pub fn sinc(&mut self) -> SINC_W {
        SINC_W { w: self }
    }
    #[doc = "Bit 6 - Destination address increment"]
    #[inline(always)]
    pub fn dinc(&mut self) -> DINC_W {
        DINC_W { w: self }
    }
    #[doc = "Bits 8:10 - Source transfer width"]
    #[inline(always)]
    pub fn src_tr_width(&mut self) -> SRC_TR_WIDTH_W {
        SRC_TR_WIDTH_W { w: self }
    }
    #[doc = "Bits 11:13 - Destination transfer width"]
    #[inline(always)]
    pub fn dst_tr_width(&mut self) -> DST_TR_WIDTH_W {
        DST_TR_WIDTH_W { w: self }
    }
    #[doc = "Bits 14:17 - Source burst transaction length"]
    #[inline(always)]
    pub fn src_msize(&mut self) -> SRC_MSIZE_W {
        SRC_MSIZE_W { w: self }
    }
    #[doc = "Bits 18:21 - Destination burst transaction length"]
    #[inline(always)]
    pub fn dst_msize(&mut self) -> DST_MSIZE_W {
        DST_MSIZE_W { w: self }
    }
    #[doc = "Bit 30 - Non Posted Last Write Enable (posted writes may be used till the end of the block)"]
    #[inline(always)]
    pub fn nonposted_lastwrite_en(&mut self) -> NONPOSTED_LASTWRITE_EN_W {
        NONPOSTED_LASTWRITE_EN_W { w: self }
    }
    #[doc = "Bit 38 - Source burst length enable"]
    #[inline(always)]
    pub fn arlen_en(&mut self) -> ARLEN_EN_W {
        ARLEN_EN_W { w: self }
    }
    #[doc = "Bits 39:46 - Source burst length"]
    #[inline(always)]
    pub fn arlen(&mut self) -> ARLEN_W {
        ARLEN_W { w: self }
    }
    #[doc = "Bit 47 - Destination burst length enable"]
    #[inline(always)]
    pub fn awlen_en(&mut self) -> AWLEN_EN_W {
        AWLEN_EN_W { w: self }
    }
    #[doc = "Bits 48:55 - Destination burst length"]
    #[inline(always)]
    pub fn awlen(&mut self) -> AWLEN_W {
        AWLEN_W { w: self }
    }
    #[doc = "Bit 56 - Source status enable"]
    #[inline(always)]
    pub fn src_stat_en(&mut self) -> SRC_STAT_EN_W {
        SRC_STAT_EN_W { w: self }
    }
    #[doc = "Bit 57 - Destination status enable"]
    #[inline(always)]
    pub fn dst_stat_en(&mut self) -> DST_STAT_EN_W {
        DST_STAT_EN_W { w: self }
    }
    #[doc = "Bit 58 - Interrupt completion of block transfer"]
    #[inline(always)]
    pub fn ioc_blktfr(&mut self) -> IOC_BLKTFR_W {
        IOC_BLKTFR_W { w: self }
    }
    #[doc = "Bit 62 - Last shadow linked list item (indicates shadowreg/LLI content is the last one)"]
    #[inline(always)]
    pub fn shadowreg_or_lli_last(&mut self) -> SHADOWREG_OR_LLI_LAST_W {
        SHADOWREG_OR_LLI_LAST_W { w: self }
    }
    #[doc = "Bit 63 - last shadow linked list item valid (indicate shadowreg/LLI content is valid)"]
    #[inline(always)]
    pub fn shadowreg_or_lli_valid(&mut self) -> SHADOWREG_OR_LLI_VALID_W {
        SHADOWREG_OR_LLI_VALID_W { w: self }
    }
}
