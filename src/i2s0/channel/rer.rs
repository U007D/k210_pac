#[doc = "Reader of register rer"]
pub type R = crate::R<u32, super::RER>;
#[doc = "Writer for register rer"]
pub type W = crate::W<u32, super::RER>;
#[doc = "Register rer `reset()`'s with value 0"]
impl crate::ResetValue for super::RER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rxchenx`"]
pub type RXCHENX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxchenx`"]
pub struct RXCHENX_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCHENX_W<'a> {
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
    #[doc = "Bit 0 - Receive channel enable/disable"]
    #[inline(always)]
    pub fn rxchenx(&self) -> RXCHENX_R {
        RXCHENX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive channel enable/disable"]
    #[inline(always)]
    pub fn rxchenx(&mut self) -> RXCHENX_W {
        RXCHENX_W { w: self }
    }
}
