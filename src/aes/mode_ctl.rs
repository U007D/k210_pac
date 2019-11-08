#[doc = "Reader of register mode_ctl"]
pub type R = crate::R<u32, super::MODE_CTL>;
#[doc = "Writer for register mode_ctl"]
pub type W = crate::W<u32, super::MODE_CTL>;
#[doc = "Register mode_ctl `reset()`'s with value 0"]
impl crate::ResetValue for super::MODE_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Cipher mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIPHER_MODE_A {
    #[doc = "0: Electronic Codebook"]
    ECB,
    #[doc = "1: Cipher Block Chaining"]
    CBC,
    #[doc = "2: Galois/Counter Mode"]
    GCM,
}
impl From<CIPHER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CIPHER_MODE_A) -> Self {
        match variant {
            CIPHER_MODE_A::ECB => 0,
            CIPHER_MODE_A::CBC => 1,
            CIPHER_MODE_A::GCM => 2,
        }
    }
}
#[doc = "Reader of field `cipher_mode`"]
pub type CIPHER_MODE_R = crate::R<u8, CIPHER_MODE_A>;
impl CIPHER_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CIPHER_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CIPHER_MODE_A::ECB),
            1 => Val(CIPHER_MODE_A::CBC),
            2 => Val(CIPHER_MODE_A::GCM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == CIPHER_MODE_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == CIPHER_MODE_A::CBC
    }
    #[doc = "Checks if the value of the field is `GCM`"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == CIPHER_MODE_A::GCM
    }
}
#[doc = "Write proxy for field `cipher_mode`"]
pub struct CIPHER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CIPHER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CIPHER_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Electronic Codebook"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(CIPHER_MODE_A::ECB)
    }
    #[doc = "Cipher Block Chaining"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(CIPHER_MODE_A::CBC)
    }
    #[doc = "Galois/Counter Mode"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut W {
        self.variant(CIPHER_MODE_A::GCM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Key mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_MODE_A {
    #[doc = "0: AES-128"]
    AES128,
    #[doc = "1: AES-192"]
    AES192,
    #[doc = "2: AES-256"]
    AES256,
}
impl From<KEY_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_MODE_A) -> Self {
        match variant {
            KEY_MODE_A::AES128 => 0,
            KEY_MODE_A::AES192 => 1,
            KEY_MODE_A::AES256 => 2,
        }
    }
}
#[doc = "Reader of field `key_mode`"]
pub type KEY_MODE_R = crate::R<u8, KEY_MODE_A>;
impl KEY_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KEY_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(KEY_MODE_A::AES128),
            1 => Val(KEY_MODE_A::AES192),
            2 => Val(KEY_MODE_A::AES256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KEY_MODE_A::AES128
    }
    #[doc = "Checks if the value of the field is `AES192`"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == KEY_MODE_A::AES192
    }
    #[doc = "Checks if the value of the field is `AES256`"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KEY_MODE_A::AES256
    }
}
#[doc = "Write proxy for field `key_mode`"]
pub struct KEY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "AES-128"]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut W {
        self.variant(KEY_MODE_A::AES128)
    }
    #[doc = "AES-192"]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut W {
        self.variant(KEY_MODE_A::AES192)
    }
    #[doc = "AES-256"]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut W {
        self.variant(KEY_MODE_A::AES256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Input key order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_ORDER_A {
    #[doc = "0: Big Endian"]
    BE,
    #[doc = "1: Little Endian"]
    LE,
}
impl From<KEY_ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: KEY_ORDER_A) -> Self {
        match variant {
            KEY_ORDER_A::BE => false,
            KEY_ORDER_A::LE => true,
        }
    }
}
#[doc = "Reader of field `key_order`"]
pub type KEY_ORDER_R = crate::R<bool, KEY_ORDER_A>;
impl KEY_ORDER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEY_ORDER_A {
        match self.bits {
            false => KEY_ORDER_A::BE,
            true => KEY_ORDER_A::LE,
        }
    }
    #[doc = "Checks if the value of the field is `BE`"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == KEY_ORDER_A::BE
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == KEY_ORDER_A::LE
    }
}
#[doc = "Write proxy for field `key_order`"]
pub struct KEY_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_ORDER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut W {
        self.variant(KEY_ORDER_A::BE)
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut W {
        self.variant(KEY_ORDER_A::LE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Input data order"]
pub type INPUT_ORDER_A = KEY_ORDER_A;
#[doc = "Reader of field `input_order`"]
pub type INPUT_ORDER_R = crate::R<bool, KEY_ORDER_A>;
#[doc = "Write proxy for field `input_order`"]
pub struct INPUT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT_ORDER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut W {
        self.variant(KEY_ORDER_A::BE)
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut W {
        self.variant(KEY_ORDER_A::LE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Output data order"]
pub type OUTPUT_ORDER_A = KEY_ORDER_A;
#[doc = "Reader of field `output_order`"]
pub type OUTPUT_ORDER_R = crate::R<bool, KEY_ORDER_A>;
#[doc = "Write proxy for field `output_order`"]
pub struct OUTPUT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTPUT_ORDER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut W {
        self.variant(KEY_ORDER_A::BE)
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut W {
        self.variant(KEY_ORDER_A::LE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Cipher mode"]
    #[inline(always)]
    pub fn cipher_mode(&self) -> CIPHER_MODE_R {
        CIPHER_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Key mode"]
    #[inline(always)]
    pub fn key_mode(&self) -> KEY_MODE_R {
        KEY_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Input key order"]
    #[inline(always)]
    pub fn key_order(&self) -> KEY_ORDER_R {
        KEY_ORDER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Input data order"]
    #[inline(always)]
    pub fn input_order(&self) -> INPUT_ORDER_R {
        INPUT_ORDER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Output data order"]
    #[inline(always)]
    pub fn output_order(&self) -> OUTPUT_ORDER_R {
        OUTPUT_ORDER_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cipher mode"]
    #[inline(always)]
    pub fn cipher_mode(&mut self) -> CIPHER_MODE_W {
        CIPHER_MODE_W { w: self }
    }
    #[doc = "Bits 3:4 - Key mode"]
    #[inline(always)]
    pub fn key_mode(&mut self) -> KEY_MODE_W {
        KEY_MODE_W { w: self }
    }
    #[doc = "Bit 5 - Input key order"]
    #[inline(always)]
    pub fn key_order(&mut self) -> KEY_ORDER_W {
        KEY_ORDER_W { w: self }
    }
    #[doc = "Bit 7 - Input data order"]
    #[inline(always)]
    pub fn input_order(&mut self) -> INPUT_ORDER_W {
        INPUT_ORDER_W { w: self }
    }
    #[doc = "Bit 9 - Output data order"]
    #[inline(always)]
    pub fn output_order(&mut self) -> OUTPUT_ORDER_W {
        OUTPUT_ORDER_W { w: self }
    }
}
