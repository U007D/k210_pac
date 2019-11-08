#[doc = "Reader of register clk_th1"]
pub type R = crate::R<u32, super::CLK_TH1>;
#[doc = "Writer for register clk_th1"]
pub type W = crate::W<u32, super::CLK_TH1>;
#[doc = "Register clk_th1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_TH1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `spi0_clk`"]
pub type SPI0_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `spi0_clk`"]
pub struct SPI0_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `spi1_clk`"]
pub type SPI1_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `spi1_clk`"]
pub struct SPI1_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `spi2_clk`"]
pub type SPI2_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `spi2_clk`"]
pub struct SPI2_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `spi3_clk`"]
pub type SPI3_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `spi3_clk`"]
pub struct SPI3_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_CLK_W<'a> {
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
    pub fn spi0_clk(&self) -> SPI0_CLK_R {
        SPI0_CLK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn spi1_clk(&self) -> SPI1_CLK_R {
        SPI1_CLK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn spi2_clk(&self) -> SPI2_CLK_R {
        SPI2_CLK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn spi3_clk(&self) -> SPI3_CLK_R {
        SPI3_CLK_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn spi0_clk(&mut self) -> SPI0_CLK_W {
        SPI0_CLK_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn spi1_clk(&mut self) -> SPI1_CLK_W {
        SPI1_CLK_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn spi2_clk(&mut self) -> SPI2_CLK_W {
        SPI2_CLK_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn spi3_clk(&mut self) -> SPI3_CLK_W {
        SPI3_CLK_W { w: self }
    }
}
