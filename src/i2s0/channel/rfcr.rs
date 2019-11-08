#[doc = "Reader of register rfcr"]
pub type R = crate::R<u32, super::RFCR>;
#[doc = "Writer for register rfcr"]
pub type W = crate::W<u32, super::RFCR>;
#[doc = "Register rfcr `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger level in the RX FIFO at which the receiver data available interrupt generate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCHDT_A {
    #[doc = "0: Interrupt trigger when FIFO level is 1"]
    LEVEL1,
    #[doc = "1: Interrupt trigger when FIFO level is 2"]
    LEVEL2,
    #[doc = "2: Interrupt trigger when FIFO level is 3"]
    LEVEL3,
    #[doc = "3: Interrupt trigger when FIFO level is 4"]
    LEVEL4,
    #[doc = "4: Interrupt trigger when FIFO level is 5"]
    LEVEL5,
    #[doc = "5: Interrupt trigger when FIFO level is 6"]
    LEVEL6,
    #[doc = "6: Interrupt trigger when FIFO level is 7"]
    LEVEL7,
    #[doc = "7: Interrupt trigger when FIFO level is 8"]
    LEVEL8,
    #[doc = "8: Interrupt trigger when FIFO level is 9"]
    LEVEL9,
    #[doc = "9: Interrupt trigger when FIFO level is 10"]
    LEVEL10,
    #[doc = "10: Interrupt trigger when FIFO level is 11"]
    LEVEL11,
    #[doc = "11: Interrupt trigger when FIFO level is 12"]
    LEVEL12,
    #[doc = "12: Interrupt trigger when FIFO level is 13"]
    LEVEL13,
    #[doc = "13: Interrupt trigger when FIFO level is 14"]
    LEVEL14,
    #[doc = "14: Interrupt trigger when FIFO level is 15"]
    LEVEL15,
    #[doc = "15: Interrupt trigger when FIFO level is 16"]
    LEVEL16,
}
impl From<RXCHDT_A> for u8 {
    #[inline(always)]
    fn from(variant: RXCHDT_A) -> Self {
        match variant {
            RXCHDT_A::LEVEL1 => 0,
            RXCHDT_A::LEVEL2 => 1,
            RXCHDT_A::LEVEL3 => 2,
            RXCHDT_A::LEVEL4 => 3,
            RXCHDT_A::LEVEL5 => 4,
            RXCHDT_A::LEVEL6 => 5,
            RXCHDT_A::LEVEL7 => 6,
            RXCHDT_A::LEVEL8 => 7,
            RXCHDT_A::LEVEL9 => 8,
            RXCHDT_A::LEVEL10 => 9,
            RXCHDT_A::LEVEL11 => 10,
            RXCHDT_A::LEVEL12 => 11,
            RXCHDT_A::LEVEL13 => 12,
            RXCHDT_A::LEVEL14 => 13,
            RXCHDT_A::LEVEL15 => 14,
            RXCHDT_A::LEVEL16 => 15,
        }
    }
}
#[doc = "Reader of field `rxchdt`"]
pub type RXCHDT_R = crate::R<u8, RXCHDT_A>;
impl RXCHDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXCHDT_A {
        match self.bits {
            0 => RXCHDT_A::LEVEL1,
            1 => RXCHDT_A::LEVEL2,
            2 => RXCHDT_A::LEVEL3,
            3 => RXCHDT_A::LEVEL4,
            4 => RXCHDT_A::LEVEL5,
            5 => RXCHDT_A::LEVEL6,
            6 => RXCHDT_A::LEVEL7,
            7 => RXCHDT_A::LEVEL8,
            8 => RXCHDT_A::LEVEL9,
            9 => RXCHDT_A::LEVEL10,
            10 => RXCHDT_A::LEVEL11,
            11 => RXCHDT_A::LEVEL12,
            12 => RXCHDT_A::LEVEL13,
            13 => RXCHDT_A::LEVEL14,
            14 => RXCHDT_A::LEVEL15,
            15 => RXCHDT_A::LEVEL16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RXCHDT_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RXCHDT_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == RXCHDT_A::LEVEL3
    }
    #[doc = "Checks if the value of the field is `LEVEL4`"]
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        *self == RXCHDT_A::LEVEL4
    }
    #[doc = "Checks if the value of the field is `LEVEL5`"]
    #[inline(always)]
    pub fn is_level5(&self) -> bool {
        *self == RXCHDT_A::LEVEL5
    }
    #[doc = "Checks if the value of the field is `LEVEL6`"]
    #[inline(always)]
    pub fn is_level6(&self) -> bool {
        *self == RXCHDT_A::LEVEL6
    }
    #[doc = "Checks if the value of the field is `LEVEL7`"]
    #[inline(always)]
    pub fn is_level7(&self) -> bool {
        *self == RXCHDT_A::LEVEL7
    }
    #[doc = "Checks if the value of the field is `LEVEL8`"]
    #[inline(always)]
    pub fn is_level8(&self) -> bool {
        *self == RXCHDT_A::LEVEL8
    }
    #[doc = "Checks if the value of the field is `LEVEL9`"]
    #[inline(always)]
    pub fn is_level9(&self) -> bool {
        *self == RXCHDT_A::LEVEL9
    }
    #[doc = "Checks if the value of the field is `LEVEL10`"]
    #[inline(always)]
    pub fn is_level10(&self) -> bool {
        *self == RXCHDT_A::LEVEL10
    }
    #[doc = "Checks if the value of the field is `LEVEL11`"]
    #[inline(always)]
    pub fn is_level11(&self) -> bool {
        *self == RXCHDT_A::LEVEL11
    }
    #[doc = "Checks if the value of the field is `LEVEL12`"]
    #[inline(always)]
    pub fn is_level12(&self) -> bool {
        *self == RXCHDT_A::LEVEL12
    }
    #[doc = "Checks if the value of the field is `LEVEL13`"]
    #[inline(always)]
    pub fn is_level13(&self) -> bool {
        *self == RXCHDT_A::LEVEL13
    }
    #[doc = "Checks if the value of the field is `LEVEL14`"]
    #[inline(always)]
    pub fn is_level14(&self) -> bool {
        *self == RXCHDT_A::LEVEL14
    }
    #[doc = "Checks if the value of the field is `LEVEL15`"]
    #[inline(always)]
    pub fn is_level15(&self) -> bool {
        *self == RXCHDT_A::LEVEL15
    }
    #[doc = "Checks if the value of the field is `LEVEL16`"]
    #[inline(always)]
    pub fn is_level16(&self) -> bool {
        *self == RXCHDT_A::LEVEL16
    }
}
#[doc = "Write proxy for field `rxchdt`"]
pub struct RXCHDT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCHDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXCHDT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Interrupt trigger when FIFO level is 1"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL1)
    }
    #[doc = "Interrupt trigger when FIFO level is 2"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL2)
    }
    #[doc = "Interrupt trigger when FIFO level is 3"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL3)
    }
    #[doc = "Interrupt trigger when FIFO level is 4"]
    #[inline(always)]
    pub fn level4(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL4)
    }
    #[doc = "Interrupt trigger when FIFO level is 5"]
    #[inline(always)]
    pub fn level5(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL5)
    }
    #[doc = "Interrupt trigger when FIFO level is 6"]
    #[inline(always)]
    pub fn level6(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL6)
    }
    #[doc = "Interrupt trigger when FIFO level is 7"]
    #[inline(always)]
    pub fn level7(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL7)
    }
    #[doc = "Interrupt trigger when FIFO level is 8"]
    #[inline(always)]
    pub fn level8(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL8)
    }
    #[doc = "Interrupt trigger when FIFO level is 9"]
    #[inline(always)]
    pub fn level9(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL9)
    }
    #[doc = "Interrupt trigger when FIFO level is 10"]
    #[inline(always)]
    pub fn level10(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL10)
    }
    #[doc = "Interrupt trigger when FIFO level is 11"]
    #[inline(always)]
    pub fn level11(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL11)
    }
    #[doc = "Interrupt trigger when FIFO level is 12"]
    #[inline(always)]
    pub fn level12(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL12)
    }
    #[doc = "Interrupt trigger when FIFO level is 13"]
    #[inline(always)]
    pub fn level13(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL13)
    }
    #[doc = "Interrupt trigger when FIFO level is 14"]
    #[inline(always)]
    pub fn level14(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL14)
    }
    #[doc = "Interrupt trigger when FIFO level is 15"]
    #[inline(always)]
    pub fn level15(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL15)
    }
    #[doc = "Interrupt trigger when FIFO level is 16"]
    #[inline(always)]
    pub fn level16(self) -> &'a mut W {
        self.variant(RXCHDT_A::LEVEL16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Trigger level in the RX FIFO at which the receiver data available interrupt generate"]
    #[inline(always)]
    pub fn rxchdt(&self) -> RXCHDT_R {
        RXCHDT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Trigger level in the RX FIFO at which the receiver data available interrupt generate"]
    #[inline(always)]
    pub fn rxchdt(&mut self) -> RXCHDT_W {
        RXCHDT_W { w: self }
    }
}
