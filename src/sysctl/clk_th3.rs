#[doc = "Reader of register clk_th3"]
pub type R = crate::R<u32, super::CLK_TH3>;
#[doc = "Writer for register clk_th3"]
pub type W = crate::W<u32, super::CLK_TH3>;
#[doc = "Register clk_th3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_TH3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `i2s0_clk`"]
pub type I2S0_CLK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `i2s0_clk`"]
pub struct I2S0_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S0_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `i2s1_clk`"]
pub type I2S1_CLK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `i2s1_clk`"]
pub struct I2S1_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S1_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn i2s0_clk(&self) -> I2S0_CLK_R {
        I2S0_CLK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn i2s1_clk(&self) -> I2S1_CLK_R {
        I2S1_CLK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn i2s0_clk(&mut self) -> I2S0_CLK_W {
        I2S0_CLK_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn i2s1_clk(&mut self) -> I2S1_CLK_W {
        I2S1_CLK_W { w: self }
    }
}
