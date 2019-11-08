#[doc = "Reader of register clk_th2"]
pub type R = crate::R<u32, super::CLK_TH2>;
#[doc = "Writer for register clk_th2"]
pub type W = crate::W<u32, super::CLK_TH2>;
#[doc = "Register clk_th2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_TH2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `timer0_clk`"]
pub type TIMER0_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `timer0_clk`"]
pub struct TIMER0_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `timer1_clk`"]
pub type TIMER1_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `timer1_clk`"]
pub struct TIMER1_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `timer2_clk`"]
pub type TIMER2_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `timer2_clk`"]
pub struct TIMER2_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer0_clk(&self) -> TIMER0_CLK_R {
        TIMER0_CLK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn timer1_clk(&self) -> TIMER1_CLK_R {
        TIMER1_CLK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn timer2_clk(&self) -> TIMER2_CLK_R {
        TIMER2_CLK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn timer0_clk(&mut self) -> TIMER0_CLK_W {
        TIMER0_CLK_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn timer1_clk(&mut self) -> TIMER1_CLK_W {
        TIMER1_CLK_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn timer2_clk(&mut self) -> TIMER2_CLK_W {
        TIMER2_CLK_W { w: self }
    }
}
