#[doc = "Reader of register pll_lock"]
pub type R = crate::R<u32, super::PLL_LOCK>;
#[doc = "Writer for register pll_lock"]
pub type W = crate::W<u32, super::PLL_LOCK>;
#[doc = "Register pll_lock `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `pll_lock0`"]
pub type PLL_LOCK0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pll_lock0`"]
pub struct PLL_LOCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LOCK0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `pll_slip_clear0`"]
pub type PLL_SLIP_CLEAR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pll_slip_clear0`"]
pub struct PLL_SLIP_CLEAR0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_SLIP_CLEAR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `test_clk_out0`"]
pub type TEST_CLK_OUT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `test_clk_out0`"]
pub struct TEST_CLK_OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_CLK_OUT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `pll_lock1`"]
pub type PLL_LOCK1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pll_lock1`"]
pub struct PLL_LOCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LOCK1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `pll_slip_clear1`"]
pub type PLL_SLIP_CLEAR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pll_slip_clear1`"]
pub struct PLL_SLIP_CLEAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_SLIP_CLEAR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `test_clk_out1`"]
pub type TEST_CLK_OUT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `test_clk_out1`"]
pub struct TEST_CLK_OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_CLK_OUT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `pll_lock2`"]
pub type PLL_LOCK2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pll_lock2`"]
pub struct PLL_LOCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_LOCK2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `pll_slip_clear2`"]
pub type PLL_SLIP_CLEAR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pll_slip_clear2`"]
pub struct PLL_SLIP_CLEAR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_SLIP_CLEAR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `test_clk_out2`"]
pub type TEST_CLK_OUT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `test_clk_out2`"]
pub struct TEST_CLK_OUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_CLK_OUT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pll_lock0(&self) -> PLL_LOCK0_R {
        PLL_LOCK0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pll_slip_clear0(&self) -> PLL_SLIP_CLEAR0_R {
        PLL_SLIP_CLEAR0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn test_clk_out0(&self) -> TEST_CLK_OUT0_R {
        TEST_CLK_OUT0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pll_lock1(&self) -> PLL_LOCK1_R {
        PLL_LOCK1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pll_slip_clear1(&self) -> PLL_SLIP_CLEAR1_R {
        PLL_SLIP_CLEAR1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn test_clk_out1(&self) -> TEST_CLK_OUT1_R {
        TEST_CLK_OUT1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pll_lock2(&self) -> PLL_LOCK2_R {
        PLL_LOCK2_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pll_slip_clear2(&self) -> PLL_SLIP_CLEAR2_R {
        PLL_SLIP_CLEAR2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn test_clk_out2(&self) -> TEST_CLK_OUT2_R {
        TEST_CLK_OUT2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pll_lock0(&mut self) -> PLL_LOCK0_W {
        PLL_LOCK0_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pll_slip_clear0(&mut self) -> PLL_SLIP_CLEAR0_W {
        PLL_SLIP_CLEAR0_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn test_clk_out0(&mut self) -> TEST_CLK_OUT0_W {
        TEST_CLK_OUT0_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pll_lock1(&mut self) -> PLL_LOCK1_W {
        PLL_LOCK1_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pll_slip_clear1(&mut self) -> PLL_SLIP_CLEAR1_W {
        PLL_SLIP_CLEAR1_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn test_clk_out1(&mut self) -> TEST_CLK_OUT1_W {
        TEST_CLK_OUT1_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pll_lock2(&mut self) -> PLL_LOCK2_W {
        PLL_LOCK2_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pll_slip_clear2(&mut self) -> PLL_SLIP_CLEAR2_W {
        PLL_SLIP_CLEAR2_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn test_clk_out2(&mut self) -> TEST_CLK_OUT2_W {
        TEST_CLK_OUT2_W { w: self }
    }
}
