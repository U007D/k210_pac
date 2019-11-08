#[doc = "Reader of register peri"]
pub type R = crate::R<u32, super::PERI>;
#[doc = "Writer for register peri"]
pub type W = crate::W<u32, super::PERI>;
#[doc = "Register peri `reset()`'s with value 0"]
impl crate::ResetValue for super::PERI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `timer0_pause`"]
pub type TIMER0_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer0_pause`"]
pub struct TIMER0_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_PAUSE_W<'a> {
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
#[doc = "Reader of field `timer1_pause`"]
pub type TIMER1_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer1_pause`"]
pub struct TIMER1_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_PAUSE_W<'a> {
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
#[doc = "Reader of field `timer2_pause`"]
pub type TIMER2_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer2_pause`"]
pub struct TIMER2_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_PAUSE_W<'a> {
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
#[doc = "Reader of field `timer3_pause`"]
pub type TIMER3_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer3_pause`"]
pub struct TIMER3_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_PAUSE_W<'a> {
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
#[doc = "Reader of field `timer4_pause`"]
pub type TIMER4_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer4_pause`"]
pub struct TIMER4_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4_PAUSE_W<'a> {
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
#[doc = "Reader of field `timer5_pause`"]
pub type TIMER5_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer5_pause`"]
pub struct TIMER5_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER5_PAUSE_W<'a> {
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
#[doc = "Reader of field `timer6_pause`"]
pub type TIMER6_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer6_pause`"]
pub struct TIMER6_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER6_PAUSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `timer7_pause`"]
pub type TIMER7_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer7_pause`"]
pub struct TIMER7_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER7_PAUSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `timer8_pause`"]
pub type TIMER8_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer8_pause`"]
pub struct TIMER8_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER8_PAUSE_W<'a> {
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
#[doc = "Reader of field `timer9_pause`"]
pub type TIMER9_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer9_pause`"]
pub struct TIMER9_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER9_PAUSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `timer10_pause`"]
pub type TIMER10_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer10_pause`"]
pub struct TIMER10_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER10_PAUSE_W<'a> {
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
#[doc = "Reader of field `timer11_pause`"]
pub type TIMER11_PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer11_pause`"]
pub struct TIMER11_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER11_PAUSE_W<'a> {
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
#[doc = "Reader of field `spi0_xip_en`"]
pub type SPI0_XIP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi0_xip_en`"]
pub struct SPI0_XIP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_XIP_EN_W<'a> {
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
#[doc = "Reader of field `spi1_xip_en`"]
pub type SPI1_XIP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi1_xip_en`"]
pub struct SPI1_XIP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_XIP_EN_W<'a> {
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
#[doc = "Reader of field `spi2_xip_en`"]
pub type SPI2_XIP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi2_xip_en`"]
pub struct SPI2_XIP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_XIP_EN_W<'a> {
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
#[doc = "Reader of field `spi3_xip_en`"]
pub type SPI3_XIP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi3_xip_en`"]
pub struct SPI3_XIP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_XIP_EN_W<'a> {
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
#[doc = "Reader of field `spi0_clk_bypass`"]
pub type SPI0_CLK_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi0_clk_bypass`"]
pub struct SPI0_CLK_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_CLK_BYPASS_W<'a> {
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
#[doc = "Reader of field `spi1_clk_bypass`"]
pub type SPI1_CLK_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi1_clk_bypass`"]
pub struct SPI1_CLK_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_CLK_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `spi2_clk_bypass`"]
pub type SPI2_CLK_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi2_clk_bypass`"]
pub struct SPI2_CLK_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_CLK_BYPASS_W<'a> {
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
#[doc = "Reader of field `i2s0_clk_bypass`"]
pub type I2S0_CLK_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2s0_clk_bypass`"]
pub struct I2S0_CLK_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S0_CLK_BYPASS_W<'a> {
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
#[doc = "Reader of field `i2s1_clk_bypass`"]
pub type I2S1_CLK_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2s1_clk_bypass`"]
pub struct I2S1_CLK_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1_CLK_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `i2s2_clk_bypass`"]
pub type I2S2_CLK_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2s2_clk_bypass`"]
pub struct I2S2_CLK_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2_CLK_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `jtag_clk_bypass`"]
pub type JTAG_CLK_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `jtag_clk_bypass`"]
pub struct JTAG_CLK_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_CLK_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `dvp_clk_bypass`"]
pub type DVP_CLK_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dvp_clk_bypass`"]
pub struct DVP_CLK_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DVP_CLK_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `debug_clk_bypass`"]
pub type DEBUG_CLK_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `debug_clk_bypass`"]
pub struct DEBUG_CLK_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_CLK_BYPASS_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_pause(&self) -> TIMER0_PAUSE_R {
        TIMER0_PAUSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer1_pause(&self) -> TIMER1_PAUSE_R {
        TIMER1_PAUSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer2_pause(&self) -> TIMER2_PAUSE_R {
        TIMER2_PAUSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer3_pause(&self) -> TIMER3_PAUSE_R {
        TIMER3_PAUSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer4_pause(&self) -> TIMER4_PAUSE_R {
        TIMER4_PAUSE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer5_pause(&self) -> TIMER5_PAUSE_R {
        TIMER5_PAUSE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer6_pause(&self) -> TIMER6_PAUSE_R {
        TIMER6_PAUSE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timer7_pause(&self) -> TIMER7_PAUSE_R {
        TIMER7_PAUSE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timer8_pause(&self) -> TIMER8_PAUSE_R {
        TIMER8_PAUSE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn timer9_pause(&self) -> TIMER9_PAUSE_R {
        TIMER9_PAUSE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn timer10_pause(&self) -> TIMER10_PAUSE_R {
        TIMER10_PAUSE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn timer11_pause(&self) -> TIMER11_PAUSE_R {
        TIMER11_PAUSE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi0_xip_en(&self) -> SPI0_XIP_EN_R {
        SPI0_XIP_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi1_xip_en(&self) -> SPI1_XIP_EN_R {
        SPI1_XIP_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi2_xip_en(&self) -> SPI2_XIP_EN_R {
        SPI2_XIP_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi3_xip_en(&self) -> SPI3_XIP_EN_R {
        SPI3_XIP_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi0_clk_bypass(&self) -> SPI0_CLK_BYPASS_R {
        SPI0_CLK_BYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi1_clk_bypass(&self) -> SPI1_CLK_BYPASS_R {
        SPI1_CLK_BYPASS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi2_clk_bypass(&self) -> SPI2_CLK_BYPASS_R {
        SPI2_CLK_BYPASS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn i2s0_clk_bypass(&self) -> I2S0_CLK_BYPASS_R {
        I2S0_CLK_BYPASS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn i2s1_clk_bypass(&self) -> I2S1_CLK_BYPASS_R {
        I2S1_CLK_BYPASS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s2_clk_bypass(&self) -> I2S2_CLK_BYPASS_R {
        I2S2_CLK_BYPASS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn jtag_clk_bypass(&self) -> JTAG_CLK_BYPASS_R {
        JTAG_CLK_BYPASS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dvp_clk_bypass(&self) -> DVP_CLK_BYPASS_R {
        DVP_CLK_BYPASS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn debug_clk_bypass(&self) -> DEBUG_CLK_BYPASS_R {
        DEBUG_CLK_BYPASS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn timer0_pause(&mut self) -> TIMER0_PAUSE_W {
        TIMER0_PAUSE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer1_pause(&mut self) -> TIMER1_PAUSE_W {
        TIMER1_PAUSE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer2_pause(&mut self) -> TIMER2_PAUSE_W {
        TIMER2_PAUSE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timer3_pause(&mut self) -> TIMER3_PAUSE_W {
        TIMER3_PAUSE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn timer4_pause(&mut self) -> TIMER4_PAUSE_W {
        TIMER4_PAUSE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn timer5_pause(&mut self) -> TIMER5_PAUSE_W {
        TIMER5_PAUSE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn timer6_pause(&mut self) -> TIMER6_PAUSE_W {
        TIMER6_PAUSE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn timer7_pause(&mut self) -> TIMER7_PAUSE_W {
        TIMER7_PAUSE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn timer8_pause(&mut self) -> TIMER8_PAUSE_W {
        TIMER8_PAUSE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn timer9_pause(&mut self) -> TIMER9_PAUSE_W {
        TIMER9_PAUSE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn timer10_pause(&mut self) -> TIMER10_PAUSE_W {
        TIMER10_PAUSE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn timer11_pause(&mut self) -> TIMER11_PAUSE_W {
        TIMER11_PAUSE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi0_xip_en(&mut self) -> SPI0_XIP_EN_W {
        SPI0_XIP_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi1_xip_en(&mut self) -> SPI1_XIP_EN_W {
        SPI1_XIP_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi2_xip_en(&mut self) -> SPI2_XIP_EN_W {
        SPI2_XIP_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spi3_xip_en(&mut self) -> SPI3_XIP_EN_W {
        SPI3_XIP_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spi0_clk_bypass(&mut self) -> SPI0_CLK_BYPASS_W {
        SPI0_CLK_BYPASS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spi1_clk_bypass(&mut self) -> SPI1_CLK_BYPASS_W {
        SPI1_CLK_BYPASS_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi2_clk_bypass(&mut self) -> SPI2_CLK_BYPASS_W {
        SPI2_CLK_BYPASS_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn i2s0_clk_bypass(&mut self) -> I2S0_CLK_BYPASS_W {
        I2S0_CLK_BYPASS_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn i2s1_clk_bypass(&mut self) -> I2S1_CLK_BYPASS_W {
        I2S1_CLK_BYPASS_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn i2s2_clk_bypass(&mut self) -> I2S2_CLK_BYPASS_W {
        I2S2_CLK_BYPASS_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn jtag_clk_bypass(&mut self) -> JTAG_CLK_BYPASS_W {
        JTAG_CLK_BYPASS_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dvp_clk_bypass(&mut self) -> DVP_CLK_BYPASS_W {
        DVP_CLK_BYPASS_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn debug_clk_bypass(&mut self) -> DEBUG_CLK_BYPASS_W {
        DEBUG_CLK_BYPASS_W { w: self }
    }
}
