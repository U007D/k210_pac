#[doc = "Reader of register tar"]
pub type R = crate::R<u32, super::TAR>;
#[doc = "Writer for register tar"]
pub type W = crate::W<u32, super::TAR>;
#[doc = "Register tar `reset()`'s with value 0"]
impl crate::ResetValue for super::TAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `address`"]
pub type ADDRESS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `address`"]
pub struct ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `gc`"]
pub type GC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gc`"]
pub struct GC_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `special`"]
pub type SPECIAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `special`"]
pub struct SPECIAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPECIAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Master Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_MASTER_WIDTH_A {
    #[doc = "0: 7-bit address"]
    B7,
    #[doc = "1: 10-bit address"]
    B10,
}
impl From<ADDR_MASTER_WIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_MASTER_WIDTH_A) -> Self {
        match variant {
            ADDR_MASTER_WIDTH_A::B7 => false,
            ADDR_MASTER_WIDTH_A::B10 => true,
        }
    }
}
#[doc = "Reader of field `addr_master_width`"]
pub type ADDR_MASTER_WIDTH_R = crate::R<bool, ADDR_MASTER_WIDTH_A>;
impl ADDR_MASTER_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_MASTER_WIDTH_A {
        match self.bits {
            false => ADDR_MASTER_WIDTH_A::B7,
            true => ADDR_MASTER_WIDTH_A::B10,
        }
    }
    #[doc = "Checks if the value of the field is `B7`"]
    #[inline(always)]
    pub fn is_b7(&self) -> bool {
        *self == ADDR_MASTER_WIDTH_A::B7
    }
    #[doc = "Checks if the value of the field is `B10`"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ADDR_MASTER_WIDTH_A::B10
    }
}
#[doc = "Write proxy for field `addr_master_width`"]
pub struct ADDR_MASTER_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_MASTER_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR_MASTER_WIDTH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "7-bit address"]
    #[inline(always)]
    pub fn b7(self) -> &'a mut W {
        self.variant(ADDR_MASTER_WIDTH_A::B7)
    }
    #[doc = "10-bit address"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut W {
        self.variant(ADDR_MASTER_WIDTH_A::B10)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Target Address"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - GC_OR_START"]
    #[inline(always)]
    pub fn gc(&self) -> GC_R {
        GC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SPECIAL"]
    #[inline(always)]
    pub fn special(&self) -> SPECIAL_R {
        SPECIAL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Master Address"]
    #[inline(always)]
    pub fn addr_master_width(&self) -> ADDR_MASTER_WIDTH_R {
        ADDR_MASTER_WIDTH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Target Address"]
    #[inline(always)]
    pub fn address(&mut self) -> ADDRESS_W {
        ADDRESS_W { w: self }
    }
    #[doc = "Bit 10 - GC_OR_START"]
    #[inline(always)]
    pub fn gc(&mut self) -> GC_W {
        GC_W { w: self }
    }
    #[doc = "Bit 11 - SPECIAL"]
    #[inline(always)]
    pub fn special(&mut self) -> SPECIAL_W {
        SPECIAL_W { w: self }
    }
    #[doc = "Bit 12 - Master Address"]
    #[inline(always)]
    pub fn addr_master_width(&mut self) -> ADDR_MASTER_WIDTH_W {
        ADDR_MASTER_WIDTH_W { w: self }
    }
}
