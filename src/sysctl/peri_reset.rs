#[doc = "Reader of register peri_reset"]
pub type R = crate::R<u32, super::PERI_RESET>;
#[doc = "Writer for register peri_reset"]
pub type W = crate::W<u32, super::PERI_RESET>;
#[doc = "Register peri_reset `reset()`'s with value 0"]
impl crate::ResetValue for super::PERI_RESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rom_reset`"]
pub type ROM_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rom_reset`"]
pub struct ROM_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_RESET_W<'a> {
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
#[doc = "Reader of field `dma_reset`"]
pub type DMA_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dma_reset`"]
pub struct DMA_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RESET_W<'a> {
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
#[doc = "Reader of field `ai_reset`"]
pub type AI_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ai_reset`"]
pub struct AI_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_RESET_W<'a> {
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
#[doc = "Reader of field `dvp_reset`"]
pub type DVP_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dvp_reset`"]
pub struct DVP_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DVP_RESET_W<'a> {
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
#[doc = "Reader of field `fft_reset`"]
pub type FFT_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fft_reset`"]
pub struct FFT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> FFT_RESET_W<'a> {
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
#[doc = "Reader of field `gpio_reset`"]
pub type GPIO_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gpio_reset`"]
pub struct GPIO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_RESET_W<'a> {
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
#[doc = "Reader of field `spi0_reset`"]
pub type SPI0_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi0_reset`"]
pub struct SPI0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_RESET_W<'a> {
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
#[doc = "Reader of field `spi1_reset`"]
pub type SPI1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi1_reset`"]
pub struct SPI1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_RESET_W<'a> {
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
#[doc = "Reader of field `spi2_reset`"]
pub type SPI2_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi2_reset`"]
pub struct SPI2_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_RESET_W<'a> {
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
#[doc = "Reader of field `spi3_reset`"]
pub type SPI3_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi3_reset`"]
pub struct SPI3_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_RESET_W<'a> {
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
#[doc = "Reader of field `i2s0_reset`"]
pub type I2S0_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2s0_reset`"]
pub struct I2S0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S0_RESET_W<'a> {
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
#[doc = "Reader of field `i2s1_reset`"]
pub type I2S1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2s1_reset`"]
pub struct I2S1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1_RESET_W<'a> {
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
#[doc = "Reader of field `i2s2_reset`"]
pub type I2S2_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2s2_reset`"]
pub struct I2S2_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2_RESET_W<'a> {
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
#[doc = "Reader of field `i2c0_reset`"]
pub type I2C0_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c0_reset`"]
pub struct I2C0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_RESET_W<'a> {
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
#[doc = "Reader of field `i2c1_reset`"]
pub type I2C1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c1_reset`"]
pub struct I2C1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_RESET_W<'a> {
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
#[doc = "Reader of field `i2c2_reset`"]
pub type I2C2_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `i2c2_reset`"]
pub struct I2C2_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_RESET_W<'a> {
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
#[doc = "Reader of field `uart1_reset`"]
pub type UART1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `uart1_reset`"]
pub struct UART1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_RESET_W<'a> {
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
#[doc = "Reader of field `uart2_reset`"]
pub type UART2_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `uart2_reset`"]
pub struct UART2_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_RESET_W<'a> {
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
#[doc = "Reader of field `uart3_reset`"]
pub type UART3_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `uart3_reset`"]
pub struct UART3_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_RESET_W<'a> {
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
#[doc = "Reader of field `aes_reset`"]
pub type AES_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `aes_reset`"]
pub struct AES_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_RESET_W<'a> {
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
#[doc = "Reader of field `fpioa_reset`"]
pub type FPIOA_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fpioa_reset`"]
pub struct FPIOA_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> FPIOA_RESET_W<'a> {
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
#[doc = "Reader of field `timer0_reset`"]
pub type TIMER0_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer0_reset`"]
pub struct TIMER0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_RESET_W<'a> {
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
#[doc = "Reader of field `timer1_reset`"]
pub type TIMER1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer1_reset`"]
pub struct TIMER1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_RESET_W<'a> {
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
#[doc = "Reader of field `timer2_reset`"]
pub type TIMER2_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `timer2_reset`"]
pub struct TIMER2_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_RESET_W<'a> {
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
#[doc = "Reader of field `wdt0_reset`"]
pub type WDT0_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wdt0_reset`"]
pub struct WDT0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT0_RESET_W<'a> {
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
#[doc = "Reader of field `wdt1_reset`"]
pub type WDT1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wdt1_reset`"]
pub struct WDT1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT1_RESET_W<'a> {
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
#[doc = "Reader of field `sha_reset`"]
pub type SHA_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sha_reset`"]
pub struct SHA_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_RESET_W<'a> {
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
#[doc = "Reader of field `rtc_reset`"]
pub type RTC_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rtc_reset`"]
pub struct RTC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_RESET_W<'a> {
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
    pub fn rom_reset(&self) -> ROM_RESET_R {
        ROM_RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_reset(&self) -> DMA_RESET_R {
        DMA_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ai_reset(&self) -> AI_RESET_R {
        AI_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dvp_reset(&self) -> DVP_RESET_R {
        DVP_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fft_reset(&self) -> FFT_RESET_R {
        FFT_RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_reset(&self) -> GPIO_RESET_R {
        GPIO_RESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi0_reset(&self) -> SPI0_RESET_R {
        SPI0_RESET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi1_reset(&self) -> SPI1_RESET_R {
        SPI1_RESET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi2_reset(&self) -> SPI2_RESET_R {
        SPI2_RESET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi3_reset(&self) -> SPI3_RESET_R {
        SPI3_RESET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s0_reset(&self) -> I2S0_RESET_R {
        I2S0_RESET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s1_reset(&self) -> I2S1_RESET_R {
        I2S1_RESET_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s2_reset(&self) -> I2S2_RESET_R {
        I2S2_RESET_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c0_reset(&self) -> I2C0_RESET_R {
        I2C0_RESET_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c1_reset(&self) -> I2C1_RESET_R {
        I2C1_RESET_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2c2_reset(&self) -> I2C2_RESET_R {
        I2C2_RESET_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart1_reset(&self) -> UART1_RESET_R {
        UART1_RESET_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart2_reset(&self) -> UART2_RESET_R {
        UART2_RESET_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart3_reset(&self) -> UART3_RESET_R {
        UART3_RESET_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn aes_reset(&self) -> AES_RESET_R {
        AES_RESET_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fpioa_reset(&self) -> FPIOA_RESET_R {
        FPIOA_RESET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn timer0_reset(&self) -> TIMER0_RESET_R {
        TIMER0_RESET_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn timer1_reset(&self) -> TIMER1_RESET_R {
        TIMER1_RESET_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn timer2_reset(&self) -> TIMER2_RESET_R {
        TIMER2_RESET_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdt0_reset(&self) -> WDT0_RESET_R {
        WDT0_RESET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wdt1_reset(&self) -> WDT1_RESET_R {
        WDT1_RESET_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sha_reset(&self) -> SHA_RESET_R {
        SHA_RESET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_reset(&self) -> RTC_RESET_R {
        RTC_RESET_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rom_reset(&mut self) -> ROM_RESET_W {
        ROM_RESET_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dma_reset(&mut self) -> DMA_RESET_W {
        DMA_RESET_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ai_reset(&mut self) -> AI_RESET_W {
        AI_RESET_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dvp_reset(&mut self) -> DVP_RESET_W {
        DVP_RESET_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fft_reset(&mut self) -> FFT_RESET_W {
        FFT_RESET_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpio_reset(&mut self) -> GPIO_RESET_W {
        GPIO_RESET_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi0_reset(&mut self) -> SPI0_RESET_W {
        SPI0_RESET_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spi1_reset(&mut self) -> SPI1_RESET_W {
        SPI1_RESET_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi2_reset(&mut self) -> SPI2_RESET_W {
        SPI2_RESET_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi3_reset(&mut self) -> SPI3_RESET_W {
        SPI3_RESET_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s0_reset(&mut self) -> I2S0_RESET_W {
        I2S0_RESET_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s1_reset(&mut self) -> I2S1_RESET_W {
        I2S1_RESET_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s2_reset(&mut self) -> I2S2_RESET_W {
        I2S2_RESET_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2c0_reset(&mut self) -> I2C0_RESET_W {
        I2C0_RESET_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn i2c1_reset(&mut self) -> I2C1_RESET_W {
        I2C1_RESET_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn i2c2_reset(&mut self) -> I2C2_RESET_W {
        I2C2_RESET_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn uart1_reset(&mut self) -> UART1_RESET_W {
        UART1_RESET_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn uart2_reset(&mut self) -> UART2_RESET_W {
        UART2_RESET_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn uart3_reset(&mut self) -> UART3_RESET_W {
        UART3_RESET_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn aes_reset(&mut self) -> AES_RESET_W {
        AES_RESET_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fpioa_reset(&mut self) -> FPIOA_RESET_W {
        FPIOA_RESET_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn timer0_reset(&mut self) -> TIMER0_RESET_W {
        TIMER0_RESET_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn timer1_reset(&mut self) -> TIMER1_RESET_W {
        TIMER1_RESET_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn timer2_reset(&mut self) -> TIMER2_RESET_W {
        TIMER2_RESET_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdt0_reset(&mut self) -> WDT0_RESET_W {
        WDT0_RESET_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wdt1_reset(&mut self) -> WDT1_RESET_W {
        WDT1_RESET_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sha_reset(&mut self) -> SHA_RESET_W {
        SHA_RESET_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rtc_reset(&mut self) -> RTC_RESET_W {
        RTC_RESET_W { w: self }
    }
}
