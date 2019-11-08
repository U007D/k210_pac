#[doc = "Reader of register function_reg_0"]
pub type R = crate::R<u32, super::FUNCTION_REG_0>;
#[doc = "Writer for register function_reg_0"]
pub type W = crate::W<u32, super::FUNCTION_REG_0>;
#[doc = "Register function_reg_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FUNCTION_REG_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `en`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `en`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `overflow`"]
pub type OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `overflow`"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Endian setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIAN_A {
    #[doc = "0: Little endian"]
    LE,
    #[doc = "1: Big endian"]
    BE,
}
impl From<ENDIAN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIAN_A) -> Self {
        match variant {
            ENDIAN_A::LE => false,
            ENDIAN_A::BE => true,
        }
    }
}
#[doc = "Reader of field `endian`"]
pub type ENDIAN_R = crate::R<bool, ENDIAN_A>;
impl ENDIAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIAN_A {
        match self.bits {
            false => ENDIAN_A::LE,
            true => ENDIAN_A::BE,
        }
    }
    #[doc = "Checks if the value of the field is `LE`"]
    #[inline(always)]
    pub fn is_le(&self) -> bool {
        *self == ENDIAN_A::LE
    }
    #[doc = "Checks if the value of the field is `BE`"]
    #[inline(always)]
    pub fn is_be(&self) -> bool {
        *self == ENDIAN_A::BE
    }
}
#[doc = "Write proxy for field `endian`"]
pub struct ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDIAN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Little endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut W {
        self.variant(ENDIAN_A::LE)
    }
    #[doc = "Big endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut W {
        self.variant(ENDIAN_A::BE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - write:SHA256 enable register. read:Calculation completed flag"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - SHA256 calculation overflow flag"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endian setting"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write:SHA256 enable register. read:Calculation completed flag"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 8 - SHA256 calculation overflow flag"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 16 - Endian setting"]
    #[inline(always)]
    pub fn endian(&mut self) -> ENDIAN_W {
        ENDIAN_W { w: self }
    }
}
