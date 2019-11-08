#[doc = "Reader of register rff"]
pub type R = crate::R<u32, super::RFF>;
#[doc = "Writer for register rff"]
pub type W = crate::W<u32, super::RFF>;
#[doc = "Register rff `reset()`'s with value 0"]
impl crate::ResetValue for super::RFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receiver channel FIFO reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCHFR_A {
    #[doc = "0: Not flush an individual FIFO"]
    NOT_FLUSH,
    #[doc = "1: Flush an indiviadual FIFO"]
    FLUSH,
}
impl From<RXCHFR_A> for bool {
    #[inline(always)]
    fn from(variant: RXCHFR_A) -> Self {
        match variant {
            RXCHFR_A::NOT_FLUSH => false,
            RXCHFR_A::FLUSH => true,
        }
    }
}
#[doc = "Reader of field `rxchfr`"]
pub type RXCHFR_R = crate::R<bool, RXCHFR_A>;
impl RXCHFR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXCHFR_A {
        match self.bits {
            false => RXCHFR_A::NOT_FLUSH,
            true => RXCHFR_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FLUSH`"]
    #[inline(always)]
    pub fn is_not_flush(&self) -> bool {
        *self == RXCHFR_A::NOT_FLUSH
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == RXCHFR_A::FLUSH
    }
}
#[doc = "Write proxy for field `rxchfr`"]
pub struct RXCHFR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCHFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXCHFR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not flush an individual FIFO"]
    #[inline(always)]
    pub fn not_flush(self) -> &'a mut W {
        self.variant(RXCHFR_A::NOT_FLUSH)
    }
    #[doc = "Flush an indiviadual FIFO"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(RXCHFR_A::FLUSH)
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
    #[doc = "Bit 0 - Receiver channel FIFO reset"]
    #[inline(always)]
    pub fn rxchfr(&self) -> RXCHFR_R {
        RXCHFR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver channel FIFO reset"]
    #[inline(always)]
    pub fn rxchfr(&mut self) -> RXCHFR_W {
        RXCHFR_W { w: self }
    }
}
