#[doc = "Reader of register clk_th4"]
pub type R = crate::R<u32, super::CLK_TH4>;
#[doc = "Writer for register clk_th4"]
pub type W = crate::W<u32, super::CLK_TH4>;
#[doc = "Register clk_th4 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_TH4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `i2s2_clk`"]
pub type I2S2_CLK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `i2s2_clk`"]
pub struct I2S2_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `i2s0_mclk`"]
pub type I2S0_MCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `i2s0_mclk`"]
pub struct I2S0_MCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S0_MCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `i2s1_mclk`"]
pub type I2S1_MCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `i2s1_mclk`"]
pub struct I2S1_MCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1_MCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn i2s2_clk(&self) -> I2S2_CLK_R {
        I2S2_CLK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2s0_mclk(&self) -> I2S0_MCLK_R {
        I2S0_MCLK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn i2s1_mclk(&self) -> I2S1_MCLK_R {
        I2S1_MCLK_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn i2s2_clk(&mut self) -> I2S2_CLK_W {
        I2S2_CLK_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2s0_mclk(&mut self) -> I2S0_MCLK_W {
        I2S0_MCLK_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn i2s1_mclk(&mut self) -> I2S1_MCLK_W {
        I2S1_MCLK_W { w: self }
    }
}
