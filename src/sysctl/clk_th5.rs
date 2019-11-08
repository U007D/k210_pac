#[doc = "Reader of register clk_th5"]
pub type R = crate::R<u32, super::CLK_TH5>;
#[doc = "Writer for register clk_th5"]
pub type W = crate::W<u32, super::CLK_TH5>;
#[doc = "Register clk_th5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_TH5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `i2s2_mclk`"]
pub type I2S2_MCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `i2s2_mclk`"]
pub struct I2S2_MCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S2_MCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `i2c0_clk`"]
pub type I2C0_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `i2c0_clk`"]
pub struct I2C0_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `i2c1_clk`"]
pub type I2C1_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `i2c1_clk`"]
pub struct I2C1_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `i2c2_clk`"]
pub type I2C2_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `i2c2_clk`"]
pub struct I2C2_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s2_mclk(&self) -> I2S2_MCLK_R {
        I2S2_MCLK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn i2c0_clk(&self) -> I2C0_CLK_R {
        I2C0_CLK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c1_clk(&self) -> I2C1_CLK_R {
        I2C1_CLK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn i2c2_clk(&self) -> I2C2_CLK_R {
        I2C2_CLK_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s2_mclk(&mut self) -> I2S2_MCLK_W {
        I2S2_MCLK_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn i2c0_clk(&mut self) -> I2C0_CLK_W {
        I2C0_CLK_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c1_clk(&mut self) -> I2C1_CLK_W {
        I2C1_CLK_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn i2c2_clk(&mut self) -> I2C2_CLK_W {
        I2C2_CLK_W { w: self }
    }
}
