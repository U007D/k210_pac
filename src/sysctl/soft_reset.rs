#[doc = "Reader of register soft_reset"]
pub type R = crate::R<u32, super::SOFT_RESET>;
#[doc = "Writer for register soft_reset"]
pub type W = crate::W<u32, super::SOFT_RESET>;
#[doc = "Register soft_reset `reset()`'s with value 0"]
impl crate::ResetValue for super::SOFT_RESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `soft_reset`"]
pub type SOFT_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_reset`"]
pub struct SOFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RESET_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W {
        SOFT_RESET_W { w: self }
    }
}
