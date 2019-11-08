#[doc = "Reader of register rxffr"]
pub type R = crate::R<u32, super::RXFFR>;
#[doc = "Writer for register rxffr"]
pub type W = crate::W<u32, super::RXFFR>;
#[doc = "Register rxffr `reset()`'s with value 0"]
impl crate::ResetValue for super::RXFFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receiver FIFO reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFFR_A {
    #[doc = "0: Not flush FIFO"]
    NOT_FLUSH,
    #[doc = "1: Flush FIFO"]
    FLUSH,
}
impl From<RXFFR_A> for bool {
    #[inline(always)]
    fn from(variant: RXFFR_A) -> Self {
        match variant {
            RXFFR_A::NOT_FLUSH => false,
            RXFFR_A::FLUSH => true,
        }
    }
}
#[doc = "Reader of field `rxffr`"]
pub type RXFFR_R = crate::R<bool, RXFFR_A>;
impl RXFFR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFFR_A {
        match self.bits {
            false => RXFFR_A::NOT_FLUSH,
            true => RXFFR_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FLUSH`"]
    #[inline(always)]
    pub fn is_not_flush(&self) -> bool {
        *self == RXFFR_A::NOT_FLUSH
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == RXFFR_A::FLUSH
    }
}
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
        self.variant(RXFFR_A::NOT_FLUSH)
    }
    #[doc = "Flush FIFO"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(RXFFR_A::FLUSH)
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
    #[doc = "Bit 0 - Receiver FIFO reset"]
    #[inline(always)]
    pub fn rxffr(&self) -> RXFFR_R {
        RXFFR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver FIFO reset"]
    #[inline(always)]
    pub fn rxffr(&mut self) -> RXFFR_W {
        RXFFR_W { w: self }
    }
}
