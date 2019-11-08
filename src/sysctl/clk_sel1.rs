#[doc = "Reader of register clk_sel1"]
pub type R = crate::R<u32, super::CLK_SEL1>;
#[doc = "Writer for register clk_sel1"]
pub type W = crate::W<u32, super::CLK_SEL1>;
#[doc = "Register clk_sel1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_SEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `spi3_sample_clk_sel`"]
pub type SPI3_SAMPLE_CLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi3_sample_clk_sel`"]
pub struct SPI3_SAMPLE_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3_SAMPLE_CLK_SEL_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi3_sample_clk_sel(&self) -> SPI3_SAMPLE_CLK_SEL_R {
        SPI3_SAMPLE_CLK_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi3_sample_clk_sel(&mut self) -> SPI3_SAMPLE_CLK_SEL_W {
        SPI3_SAMPLE_CLK_SEL_W { w: self }
    }
}
