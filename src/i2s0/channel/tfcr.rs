#[doc = "Reader of register tfcr"]
pub type R = crate::R<u32, super::TFCR>;
#[doc = "Writer for register tfcr"]
pub type W = crate::W<u32, super::TFCR>;
#[doc = "Register tfcr `reset()`'s with value 0"]
impl crate::ResetValue for super::TFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger level in the TX FIFO at which the transmitter data available interrupt generate"]
pub type TXCHET_A = super::rfcr::RXCHDT_A;
#[doc = "Reader of field `txchet`"]
pub type TXCHET_R = crate::R<u8, super::rfcr::RXCHDT_A>;
#[doc = "Write proxy for field `txchet`"]
pub struct TXCHET_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCHET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCHET_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt trigger when FIFO level is 1"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL1)
    }
    #[doc = "Interrupt trigger when FIFO level is 2"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL2)
    }
    #[doc = "Interrupt trigger when FIFO level is 3"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL3)
    }
    #[doc = "Interrupt trigger when FIFO level is 4"]
    #[inline(always)]
    pub fn level4(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL4)
    }
    #[doc = "Interrupt trigger when FIFO level is 5"]
    #[inline(always)]
    pub fn level5(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL5)
    }
    #[doc = "Interrupt trigger when FIFO level is 6"]
    #[inline(always)]
    pub fn level6(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL6)
    }
    #[doc = "Interrupt trigger when FIFO level is 7"]
    #[inline(always)]
    pub fn level7(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL7)
    }
    #[doc = "Interrupt trigger when FIFO level is 8"]
    #[inline(always)]
    pub fn level8(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL8)
    }
    #[doc = "Interrupt trigger when FIFO level is 9"]
    #[inline(always)]
    pub fn level9(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL9)
    }
    #[doc = "Interrupt trigger when FIFO level is 10"]
    #[inline(always)]
    pub fn level10(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL10)
    }
    #[doc = "Interrupt trigger when FIFO level is 11"]
    #[inline(always)]
    pub fn level11(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL11)
    }
    #[doc = "Interrupt trigger when FIFO level is 12"]
    #[inline(always)]
    pub fn level12(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL12)
    }
    #[doc = "Interrupt trigger when FIFO level is 13"]
    #[inline(always)]
    pub fn level13(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL13)
    }
    #[doc = "Interrupt trigger when FIFO level is 14"]
    #[inline(always)]
    pub fn level14(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL14)
    }
    #[doc = "Interrupt trigger when FIFO level is 15"]
    #[inline(always)]
    pub fn level15(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL15)
    }
    #[doc = "Interrupt trigger when FIFO level is 16"]
    #[inline(always)]
    pub fn level16(self) -> &'a mut W {
        self.variant(super::rfcr::RXCHDT_A::LEVEL16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Trigger level in the TX FIFO at which the transmitter data available interrupt generate"]
    #[inline(always)]
    pub fn txchet(&self) -> TXCHET_R {
        TXCHET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trigger level in the TX FIFO at which the transmitter data available interrupt generate"]
    #[inline(always)]
    pub fn txchet(&mut self) -> TXCHET_W {
        TXCHET_W { w: self }
    }
}
