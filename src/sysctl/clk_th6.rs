#[doc = "Reader of register clk_th6"]
pub type R = crate::R<u32, super::CLK_TH6>;
#[doc = "Writer for register clk_th6"]
pub type W = crate::W<u32, super::CLK_TH6>;
#[doc = "Register clk_th6 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_TH6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `wdt0_clk`"]
pub type WDT0_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `wdt0_clk`"]
pub struct WDT0_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT0_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `wdt1_clk`"]
pub type WDT1_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `wdt1_clk`"]
pub struct WDT1_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT1_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn wdt0_clk(&self) -> WDT0_CLK_R {
        WDT0_CLK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn wdt1_clk(&self) -> WDT1_CLK_R {
        WDT1_CLK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn wdt0_clk(&mut self) -> WDT0_CLK_W {
        WDT0_CLK_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn wdt1_clk(&mut self) -> WDT1_CLK_W {
        WDT1_CLK_W { w: self }
    }
}
