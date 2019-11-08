#[doc = "Reader of register clk_en_cent"]
pub type R = crate::R<u32, super::CLK_EN_CENT>;
#[doc = "Writer for register clk_en_cent"]
pub type W = crate::W<u32, super::CLK_EN_CENT>;
#[doc = "Register clk_en_cent `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_EN_CENT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cpu_clk_en`"]
pub type CPU_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cpu_clk_en`"]
pub struct CPU_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_CLK_EN_W<'a> {
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
#[doc = "Reader of field `sram0_clk_en`"]
pub type SRAM0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sram0_clk_en`"]
pub struct SRAM0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0_CLK_EN_W<'a> {
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
#[doc = "Reader of field `sram1_clk_en`"]
pub type SRAM1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sram1_clk_en`"]
pub struct SRAM1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1_CLK_EN_W<'a> {
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
#[doc = "Reader of field `apb0_clk_en`"]
pub type APB0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `apb0_clk_en`"]
pub struct APB0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB0_CLK_EN_W<'a> {
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
#[doc = "Reader of field `apb1_clk_en`"]
pub type APB1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `apb1_clk_en`"]
pub struct APB1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB1_CLK_EN_W<'a> {
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
#[doc = "Reader of field `apb2_clk_en`"]
pub type APB2_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `apb2_clk_en`"]
pub struct APB2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> APB2_CLK_EN_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpu_clk_en(&self) -> CPU_CLK_EN_R {
        CPU_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sram0_clk_en(&self) -> SRAM0_CLK_EN_R {
        SRAM0_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sram1_clk_en(&self) -> SRAM1_CLK_EN_R {
        SRAM1_CLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apb0_clk_en(&self) -> APB0_CLK_EN_R {
        APB0_CLK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn apb1_clk_en(&self) -> APB1_CLK_EN_R {
        APB1_CLK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn apb2_clk_en(&self) -> APB2_CLK_EN_R {
        APB2_CLK_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cpu_clk_en(&mut self) -> CPU_CLK_EN_W {
        CPU_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sram0_clk_en(&mut self) -> SRAM0_CLK_EN_W {
        SRAM0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sram1_clk_en(&mut self) -> SRAM1_CLK_EN_W {
        SRAM1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn apb0_clk_en(&mut self) -> APB0_CLK_EN_W {
        APB0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn apb1_clk_en(&mut self) -> APB1_CLK_EN_W {
        APB1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn apb2_clk_en(&mut self) -> APB2_CLK_EN_W {
        APB2_CLK_EN_W { w: self }
    }
}
