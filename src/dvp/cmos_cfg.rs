#[doc = "Reader of register cmos_cfg"]
pub type R = crate::R<u32, super::CMOS_CFG>;
#[doc = "Writer for register cmos_cfg"]
pub type W = crate::W<u32, super::CMOS_CFG>;
#[doc = "Register cmos_cfg `reset()`'s with value 0"]
impl crate::ResetValue for super::CMOS_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `clk_div`"]
pub type CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clk_div`"]
pub struct CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `clk_enable`"]
pub type CLK_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clk_enable`"]
pub struct CLK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `reset`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reset`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `power_down`"]
pub type POWER_DOWN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `power_down`"]
pub struct POWER_DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_DOWN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CLK_DIV"]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - CLK_ENABLE"]
    #[inline(always)]
    pub fn clk_enable(&self) -> CLK_ENABLE_R {
        CLK_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - POWER_DOWN"]
    #[inline(always)]
    pub fn power_down(&self) -> POWER_DOWN_R {
        POWER_DOWN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLK_DIV"]
    #[inline(always)]
    pub fn clk_div(&mut self) -> CLK_DIV_W {
        CLK_DIV_W { w: self }
    }
    #[doc = "Bit 8 - CLK_ENABLE"]
    #[inline(always)]
    pub fn clk_enable(&mut self) -> CLK_ENABLE_W {
        CLK_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 24 - POWER_DOWN"]
    #[inline(always)]
    pub fn power_down(&mut self) -> POWER_DOWN_W {
        POWER_DOWN_W { w: self }
    }
}
