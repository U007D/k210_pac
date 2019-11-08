#[doc = "Reader of register clk_sel0"]
pub type R = crate::R<u32, super::CLK_SEL0>;
#[doc = "Writer for register clk_sel0"]
pub type W = crate::W<u32, super::CLK_SEL0>;
#[doc = "Register clk_sel0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_SEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `aclk_sel`"]
pub type ACLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `aclk_sel`"]
pub struct ACLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_SEL_W<'a> {
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
#[doc = "Reader of field `aclk_divider_sel`"]
pub type ACLK_DIVIDER_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `aclk_divider_sel`"]
pub struct ACLK_DIVIDER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_DIVIDER_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `apb0_clk_sel`"]
pub type APB0_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `apb0_clk_sel`"]
pub struct APB0_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB0_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `apb1_clk_sel`"]
pub type APB1_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `apb1_clk_sel`"]
pub struct APB1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB1_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `apb2_clk_sel`"]
pub type APB2_CLK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `apb2_clk_sel`"]
pub struct APB2_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `spi3_clk_sel`"]
pub type SPI3_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi3_clk_sel`"]
pub struct SPI3_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `timer0_clk_sel`"]
pub type TIMER0_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer0_clk_sel`"]
pub struct TIMER0_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `timer1_clk_sel`"]
pub type TIMER1_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer1_clk_sel`"]
pub struct TIMER1_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `timer2_clk_sel`"]
pub type TIMER2_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer2_clk_sel`"]
pub struct TIMER2_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_CLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aclk_sel(&self) -> ACLK_SEL_R {
        ACLK_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn aclk_divider_sel(&self) -> ACLK_DIVIDER_SEL_R {
        ACLK_DIVIDER_SEL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn apb0_clk_sel(&self) -> APB0_CLK_SEL_R {
        APB0_CLK_SEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn apb1_clk_sel(&self) -> APB1_CLK_SEL_R {
        APB1_CLK_SEL_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn apb2_clk_sel(&self) -> APB2_CLK_SEL_R {
        APB2_CLK_SEL_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi3_clk_sel(&self) -> SPI3_CLK_SEL_R {
        SPI3_CLK_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timer0_clk_sel(&self) -> TIMER0_CLK_SEL_R {
        TIMER0_CLK_SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn timer1_clk_sel(&self) -> TIMER1_CLK_SEL_R {
        TIMER1_CLK_SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timer2_clk_sel(&self) -> TIMER2_CLK_SEL_R {
        TIMER2_CLK_SEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn aclk_sel(&mut self) -> ACLK_SEL_W {
        ACLK_SEL_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn aclk_divider_sel(&mut self) -> ACLK_DIVIDER_SEL_W {
        ACLK_DIVIDER_SEL_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn apb0_clk_sel(&mut self) -> APB0_CLK_SEL_W {
        APB0_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn apb1_clk_sel(&mut self) -> APB1_CLK_SEL_W {
        APB1_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn apb2_clk_sel(&mut self) -> APB2_CLK_SEL_W {
        APB2_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi3_clk_sel(&mut self) -> SPI3_CLK_SEL_W {
        SPI3_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn timer0_clk_sel(&mut self) -> TIMER0_CLK_SEL_W {
        TIMER0_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn timer1_clk_sel(&mut self) -> TIMER1_CLK_SEL_W {
        TIMER1_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn timer2_clk_sel(&mut self) -> TIMER2_CLK_SEL_W {
        TIMER2_CLK_SEL_W { w: self }
    }
}
