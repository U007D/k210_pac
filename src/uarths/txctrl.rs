#[doc = "Reader of register txctrl"]
pub type R = crate::R<u32, super::TXCTRL>;
#[doc = "Writer for register txctrl"]
pub type W = crate::W<u32, super::TXCTRL>;
#[doc = "Register txctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::TXCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `txen`"]
pub type TXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `txen`"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
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
#[doc = "Reader of field `nstop`"]
pub type NSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `nstop`"]
pub struct NSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NSTOP_W<'a> {
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
#[doc = "Reader of field `txcnt`"]
pub type TXCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `txcnt`"]
pub struct TXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Number of stop bits"]
    #[inline(always)]
    pub fn nstop(&self) -> NSTOP_R {
        NSTOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Transmit watermark level"]
    #[inline(always)]
    pub fn txcnt(&self) -> TXCNT_R {
        TXCNT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 1 - Number of stop bits"]
    #[inline(always)]
    pub fn nstop(&mut self) -> NSTOP_W {
        NSTOP_W { w: self }
    }
    #[doc = "Bits 16:18 - Transmit watermark level"]
    #[inline(always)]
    pub fn txcnt(&mut self) -> TXCNT_W {
        TXCNT_W { w: self }
    }
}
