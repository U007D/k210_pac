#[doc = "Reader of register tff"]
pub type R = crate::R<u32, super::TFF>;
#[doc = "Writer for register tff"]
pub type W = crate::W<u32, super::TFF>;
#[doc = "Register tff `reset()`'s with value 0"]
impl crate::ResetValue for super::TFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmit channel FIFO reset"]
pub type RTXCHFR_A = super::rff::RXCHFR_A;
#[doc = "Reader of field `rtxchfr`"]
pub type RTXCHFR_R = crate::R<bool, super::rff::RXCHFR_A>;
#[doc = "Write proxy for field `rtxchfr`"]
pub struct RTXCHFR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTXCHFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTXCHFR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not flush an individual FIFO"]
    #[inline(always)]
    pub fn not_flush(self) -> &'a mut W {
        self.variant(super::rff::RXCHFR_A::NOT_FLUSH)
    }
    #[doc = "Flush an indiviadual FIFO"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(super::rff::RXCHFR_A::FLUSH)
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
    #[doc = "Bit 0 - Transmit channel FIFO reset"]
    #[inline(always)]
    pub fn rtxchfr(&self) -> RTXCHFR_R {
        RTXCHFR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit channel FIFO reset"]
    #[inline(always)]
    pub fn rtxchfr(&mut self) -> RTXCHFR_W {
        RTXCHFR_W { w: self }
    }
}
