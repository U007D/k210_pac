#[doc = "Reader of register rxctrl"]
pub type R = crate::R<u32, super::RXCTRL>;
#[doc = "Writer for register rxctrl"]
pub type W = crate::W<u32, super::RXCTRL>;
#[doc = "Register rxctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::RXCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rxen`"]
pub type RXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxen`"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
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
#[doc = "Reader of field `rxcnt`"]
pub type RXCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rxcnt`"]
pub struct RXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Receive watermark level"]
    #[inline(always)]
    pub fn rxcnt(&self) -> RXCNT_R {
        RXCNT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Bits 16:18 - Receive watermark level"]
    #[inline(always)]
    pub fn rxcnt(&mut self) -> RXCNT_W {
        RXCNT_W { w: self }
    }
}
