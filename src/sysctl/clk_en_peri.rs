#[doc = "Reader of register clk_en_peri"]
pub type R = crate::R<u32, super::CLK_EN_PERI>;
#[doc = "Writer for register clk_en_peri"]
pub type W = crate::W<u32, super::CLK_EN_PERI>;
#[doc = "Register clk_en_peri `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_EN_PERI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rom_clk_en`"]
pub type ROM_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rom_clk_en`"]
pub struct ROM_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_CLK_EN_W<'a> {
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
#[doc = "Reader of field `dma_clk_en`"]
pub type DMA_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dma_clk_en`"]
pub struct DMA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_CLK_EN_W<'a> {
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
#[doc = "Reader of field `ai_clk_en`"]
pub type AI_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ai_clk_en`"]
pub struct AI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_CLK_EN_W<'a> {
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
#[doc = "Reader of field `dvp_clk_en`"]
pub type DVP_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dvp_clk_en`"]
pub struct DVP_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DVP_CLK_EN_W<'a> {
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
#[doc = "Reader of field `fft_clk_en`"]
pub type FFT_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fft_clk_en`"]
pub struct FFT_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FFT_CLK_EN_W<'a> {
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
#[doc = "Reader of field `gpio_clk_en`"]
pub type GPIO_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpio_clk_en`"]
pub struct GPIO_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_CLK_EN_W<'a> {
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
#[doc = "Reader of field `spi0_clk_en`"]
pub type SPI0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi0_clk_en`"]
pub struct SPI0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_CLK_EN_W<'a> {
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
#[doc = "Reader of field `spi1_clk_en`"]
pub type SPI1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi1_clk_en`"]
pub struct SPI1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_CLK_EN_W<'a> {
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
#[doc = "Reader of field `spi2_clk_en`"]
pub type SPI2_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi2_clk_en`"]
pub struct SPI2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_CLK_EN_W<'a> {
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
#[doc = "Reader of field `spi3_clk_en`"]
pub type SPI3_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi3_clk_en`"]
pub struct SPI3_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_CLK_EN_W<'a> {
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
#[doc = "Reader of field `i2s0_clk_en`"]
pub type I2S0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2s0_clk_en`"]
pub struct I2S0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S0_CLK_EN_W<'a> {
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
#[doc = "Reader of field `i2s1_clk_en`"]
pub type I2S1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2s1_clk_en`"]
pub struct I2S1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1_CLK_EN_W<'a> {
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
#[doc = "Reader of field `i2s2_clk_en`"]
pub type I2S2_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2s2_clk_en`"]
pub struct I2S2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2_CLK_EN_W<'a> {
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
#[doc = "Reader of field `i2c0_clk_en`"]
pub type I2C0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c0_clk_en`"]
pub struct I2C0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_CLK_EN_W<'a> {
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
#[doc = "Reader of field `i2c1_clk_en`"]
pub type I2C1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c1_clk_en`"]
pub struct I2C1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_CLK_EN_W<'a> {
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
#[doc = "Reader of field `i2c2_clk_en`"]
pub type I2C2_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c2_clk_en`"]
pub struct I2C2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_CLK_EN_W<'a> {
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
#[doc = "Reader of field `uart1_clk_en`"]
pub type UART1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `uart1_clk_en`"]
pub struct UART1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_CLK_EN_W<'a> {
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
#[doc = "Reader of field `uart2_clk_en`"]
pub type UART2_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `uart2_clk_en`"]
pub struct UART2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_CLK_EN_W<'a> {
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
#[doc = "Reader of field `uart3_clk_en`"]
pub type UART3_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `uart3_clk_en`"]
pub struct UART3_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_CLK_EN_W<'a> {
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
#[doc = "Reader of field `aes_clk_en`"]
pub type AES_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `aes_clk_en`"]
pub struct AES_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_CLK_EN_W<'a> {
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
#[doc = "Reader of field `fpioa_clk_en`"]
pub type FPIOA_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fpioa_clk_en`"]
pub struct FPIOA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIOA_CLK_EN_W<'a> {
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
#[doc = "Reader of field `timer0_clk_en`"]
pub type TIMER0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer0_clk_en`"]
pub struct TIMER0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_CLK_EN_W<'a> {
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
#[doc = "Reader of field `timer1_clk_en`"]
pub type TIMER1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer1_clk_en`"]
pub struct TIMER1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_CLK_EN_W<'a> {
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
#[doc = "Reader of field `timer2_clk_en`"]
pub type TIMER2_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer2_clk_en`"]
pub struct TIMER2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_CLK_EN_W<'a> {
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
#[doc = "Reader of field `wdt0_clk_en`"]
pub type WDT0_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wdt0_clk_en`"]
pub struct WDT0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT0_CLK_EN_W<'a> {
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
#[doc = "Reader of field `wdt1_clk_en`"]
pub type WDT1_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wdt1_clk_en`"]
pub struct WDT1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT1_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `sha_clk_en`"]
pub type SHA_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sha_clk_en`"]
pub struct SHA_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `otp_clk_en`"]
pub type OTP_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `otp_clk_en`"]
pub struct OTP_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `rtc_clk_en`"]
pub type RTC_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rtc_clk_en`"]
pub struct RTC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rom_clk_en(&self) -> ROM_CLK_EN_R {
        ROM_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ai_clk_en(&self) -> AI_CLK_EN_R {
        AI_CLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dvp_clk_en(&self) -> DVP_CLK_EN_R {
        DVP_CLK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fft_clk_en(&self) -> FFT_CLK_EN_R {
        FFT_CLK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_clk_en(&self) -> GPIO_CLK_EN_R {
        GPIO_CLK_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi0_clk_en(&self) -> SPI0_CLK_EN_R {
        SPI0_CLK_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi1_clk_en(&self) -> SPI1_CLK_EN_R {
        SPI1_CLK_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi2_clk_en(&self) -> SPI2_CLK_EN_R {
        SPI2_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi3_clk_en(&self) -> SPI3_CLK_EN_R {
        SPI3_CLK_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s0_clk_en(&self) -> I2S0_CLK_EN_R {
        I2S0_CLK_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s1_clk_en(&self) -> I2S1_CLK_EN_R {
        I2S1_CLK_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s2_clk_en(&self) -> I2S2_CLK_EN_R {
        I2S2_CLK_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c0_clk_en(&self) -> I2C0_CLK_EN_R {
        I2C0_CLK_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c1_clk_en(&self) -> I2C1_CLK_EN_R {
        I2C1_CLK_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2c2_clk_en(&self) -> I2C2_CLK_EN_R {
        I2C2_CLK_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart3_clk_en(&self) -> UART3_CLK_EN_R {
        UART3_CLK_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn aes_clk_en(&self) -> AES_CLK_EN_R {
        AES_CLK_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fpioa_clk_en(&self) -> FPIOA_CLK_EN_R {
        FPIOA_CLK_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn timer0_clk_en(&self) -> TIMER0_CLK_EN_R {
        TIMER0_CLK_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn timer1_clk_en(&self) -> TIMER1_CLK_EN_R {
        TIMER1_CLK_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn timer2_clk_en(&self) -> TIMER2_CLK_EN_R {
        TIMER2_CLK_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdt0_clk_en(&self) -> WDT0_CLK_EN_R {
        WDT0_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wdt1_clk_en(&self) -> WDT1_CLK_EN_R {
        WDT1_CLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sha_clk_en(&self) -> SHA_CLK_EN_R {
        SHA_CLK_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn otp_clk_en(&self) -> OTP_CLK_EN_R {
        OTP_CLK_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_clk_en(&self) -> RTC_CLK_EN_R {
        RTC_CLK_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rom_clk_en(&mut self) -> ROM_CLK_EN_W {
        ROM_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W {
        DMA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ai_clk_en(&mut self) -> AI_CLK_EN_W {
        AI_CLK_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dvp_clk_en(&mut self) -> DVP_CLK_EN_W {
        DVP_CLK_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fft_clk_en(&mut self) -> FFT_CLK_EN_W {
        FFT_CLK_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_clk_en(&mut self) -> GPIO_CLK_EN_W {
        GPIO_CLK_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi0_clk_en(&mut self) -> SPI0_CLK_EN_W {
        SPI0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi1_clk_en(&mut self) -> SPI1_CLK_EN_W {
        SPI1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi2_clk_en(&mut self) -> SPI2_CLK_EN_W {
        SPI2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi3_clk_en(&mut self) -> SPI3_CLK_EN_W {
        SPI3_CLK_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s0_clk_en(&mut self) -> I2S0_CLK_EN_W {
        I2S0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s1_clk_en(&mut self) -> I2S1_CLK_EN_W {
        I2S1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s2_clk_en(&mut self) -> I2S2_CLK_EN_W {
        I2S2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c0_clk_en(&mut self) -> I2C0_CLK_EN_W {
        I2C0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c1_clk_en(&mut self) -> I2C1_CLK_EN_W {
        I2C1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2c2_clk_en(&mut self) -> I2C2_CLK_EN_W {
        I2C2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W {
        UART1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W {
        UART2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart3_clk_en(&mut self) -> UART3_CLK_EN_W {
        UART3_CLK_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn aes_clk_en(&mut self) -> AES_CLK_EN_W {
        AES_CLK_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fpioa_clk_en(&mut self) -> FPIOA_CLK_EN_W {
        FPIOA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn timer0_clk_en(&mut self) -> TIMER0_CLK_EN_W {
        TIMER0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn timer1_clk_en(&mut self) -> TIMER1_CLK_EN_W {
        TIMER1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn timer2_clk_en(&mut self) -> TIMER2_CLK_EN_W {
        TIMER2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdt0_clk_en(&mut self) -> WDT0_CLK_EN_W {
        WDT0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wdt1_clk_en(&mut self) -> WDT1_CLK_EN_W {
        WDT1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sha_clk_en(&mut self) -> SHA_CLK_EN_W {
        SHA_CLK_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn otp_clk_en(&mut self) -> OTP_CLK_EN_W {
        OTP_CLK_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_clk_en(&mut self) -> RTC_CLK_EN_W {
        RTC_CLK_EN_W { w: self }
    }
}
