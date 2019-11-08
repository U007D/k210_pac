#[doc = "Reader of register txffr"]
pub type R = crate::R<u32, super::TXFFR>;
#[doc = "Writer for register txffr"]
pub type W = crate::W<u32, super::TXFFR>;
#[doc = "Register txffr `reset()`'s with value 0"]
impl crate::ResetValue for super::TXFFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmitter FIFO reset"]
pub type RXFFR_A = super::rxffr::RXFFR_A;
#[doc = "Reader of field `rxffr`"]
pub type RXFFR_R = crate::R<bool, super::rxffr::RXFFR_A>;
#[doc = "Write proxy for field `rxffr`"]
pub struct RXFFR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFFR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not flush FIFO"]
    #[inline(always)]
    pub fn not_flush(self) -> &'a mut W {
        self.variant(super::rxffr::RXFFR_A::NOT_FLUSH)
    }
    #[doc = "Flush FIFO"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(super::rxffr::RXFFR_A::FLUSH)
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
    #[doc = "Bit 0 - Transmitter FIFO reset"]
    #[inline(always)]
    pub fn rxffr(&self) -> RXFFR_R {
        RXFFR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter FIFO reset"]
    #[inline(always)]
    pub fn rxffr(&mut self) -> RXFFR_W {
        RXFFR_W { w: self }
    }
}
