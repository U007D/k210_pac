#[doc = "Reader of register imr"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Writer for register imr"]
pub type W = crate::W<u32, super::IMR>;
#[doc = "Register imr `reset()`'s with value 0"]
impl crate::ResetValue for super::IMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rxdam`"]
pub type RXDAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxdam`"]
pub struct RXDAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDAM_W<'a> {
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
#[doc = "Reader of field `rxfom`"]
pub type RXFOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rxfom`"]
pub struct RXFOM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFOM_W<'a> {
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
#[doc = "Reader of field `txfem`"]
pub type TXFEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `txfem`"]
pub struct TXFEM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `txfom`"]
pub type TXFOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `txfom`"]
pub struct TXFOM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFOM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Mask RX FIFO data avaliable interrupt"]
    #[inline(always)]
    pub fn rxdam(&self) -> RXDAM_R {
        RXDAM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask RX FIFO overrun interrupt"]
    #[inline(always)]
    pub fn rxfom(&self) -> RXFOM_R {
        RXFOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask TX FIFO empty interrupt"]
    #[inline(always)]
    pub fn txfem(&self) -> TXFEM_R {
        TXFEM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask TX FIFO overrun interrupt"]
    #[inline(always)]
    pub fn txfom(&self) -> TXFOM_R {
        TXFOM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask RX FIFO data avaliable interrupt"]
    #[inline(always)]
    pub fn rxdam(&mut self) -> RXDAM_W {
        RXDAM_W { w: self }
    }
    #[doc = "Bit 1 - Mask RX FIFO overrun interrupt"]
    #[inline(always)]
    pub fn rxfom(&mut self) -> RXFOM_W {
        RXFOM_W { w: self }
    }
    #[doc = "Bit 4 - Mask TX FIFO empty interrupt"]
    #[inline(always)]
    pub fn txfem(&mut self) -> TXFEM_W {
        TXFEM_W { w: self }
    }
    #[doc = "Bit 5 - Mask TX FIFO overrun interrupt"]
    #[inline(always)]
    pub fn txfom(&mut self) -> TXFOM_W {
        TXFOM_W { w: self }
    }
}
