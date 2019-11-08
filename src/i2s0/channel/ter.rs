#[doc = "Reader of register ter"]
pub type R = crate::R<u32, super::TER>;
#[doc = "Writer for register ter"]
pub type W = crate::W<u32, super::TER>;
#[doc = "Register ter `reset()`'s with value 0"]
impl crate::ResetValue for super::TER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `txchenx`"]
pub type TXCHENX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `txchenx`"]
pub struct TXCHENX_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCHENX_W<'a> {
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
    #[doc = "Bit 0 - Transmit channel enable/disable"]
    #[inline(always)]
    pub fn txchenx(&self) -> TXCHENX_R {
        TXCHENX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit channel enable/disable"]
    #[inline(always)]
    pub fn txchenx(&mut self) -> TXCHENX_W {
        TXCHENX_W { w: self }
    }
}
