#[doc = "Reader of register eight_bit_mode"]
pub type R = crate::R<u64, super::EIGHT_BIT_MODE>;
#[doc = "Writer for register eight_bit_mode"]
pub type W = crate::W<u64, super::EIGHT_BIT_MODE>;
#[doc = "Register eight_bit_mode `reset()`'s with value 0"]
impl crate::ResetValue for super::EIGHT_BIT_MODE {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `eight_bit_mode`"]
pub type EIGHT_BIT_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `eight_bit_mode`"]
pub struct EIGHT_BIT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIGHT_BIT_MODE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Use 8-bit instead of 16-bit precision if set"]
    #[inline(always)]
    pub fn eight_bit_mode(&self) -> EIGHT_BIT_MODE_R {
        EIGHT_BIT_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use 8-bit instead of 16-bit precision if set"]
    #[inline(always)]
    pub fn eight_bit_mode(&mut self) -> EIGHT_BIT_MODE_W {
        EIGHT_BIT_MODE_W { w: self }
    }
}
