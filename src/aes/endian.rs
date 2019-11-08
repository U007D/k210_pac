#[doc = "Reader of register endian"]
pub type R = crate::R<u32, super::ENDIAN>;
#[doc = "Writer for register endian"]
pub type W = crate::W<u32, super::ENDIAN>;
#[doc = "Register endian `reset()`'s with value 0"]
impl crate::ResetValue for super::ENDIAN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Input data endian"]
pub type ENDIAN_A = super::mode_ctl::KEY_ORDER_A;
#[doc = "Reader of field `endian`"]
pub type ENDIAN_R = crate::R<bool, super::mode_ctl::KEY_ORDER_A>;
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
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn be(self) -> &'a mut W {
        self.variant(super::mode_ctl::KEY_ORDER_A::BE)
    }
    #[doc = "Little Endian"]
    #[inline(always)]
    pub fn le(self) -> &'a mut W {
        self.variant(super::mode_ctl::KEY_ORDER_A::LE)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Input data endian"]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input data endian"]
    #[inline(always)]
    pub fn endian(&mut self) -> ENDIAN_W {
        ENDIAN_W { w: self }
    }
}
