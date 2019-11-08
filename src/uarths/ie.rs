#[doc = "Reader of register ie"]
pub type R = crate::R<u32, super::IE>;
#[doc = "Writer for register ie"]
pub type W = crate::W<u32, super::IE>;
#[doc = "Register ie `reset()`'s with value 0"]
impl crate::ResetValue for super::IE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `txwm`"]
pub type TXWM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `txwm`"]
pub struct TXWM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXWM_W<'a> {
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
#[doc = "Reader of field `rxwm`"]
pub type RXWM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxwm`"]
pub struct RXWM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXWM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit watermark interrupt enable"]
    #[inline(always)]
    pub fn txwm(&self) -> TXWM_R {
        TXWM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive watermark interrupt enable"]
    #[inline(always)]
    pub fn rxwm(&self) -> RXWM_R {
        RXWM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit watermark interrupt enable"]
    #[inline(always)]
    pub fn txwm(&mut self) -> TXWM_W {
        TXWM_W { w: self }
    }
    #[doc = "Bit 1 - Receive watermark interrupt enable"]
    #[inline(always)]
    pub fn rxwm(&mut self) -> RXWM_W {
        RXWM_W { w: self }
    }
}
