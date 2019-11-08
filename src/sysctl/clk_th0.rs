#[doc = "Reader of register clk_th0"]
pub type R = crate::R<u32, super::CLK_TH0>;
#[doc = "Writer for register clk_th0"]
pub type W = crate::W<u32, super::CLK_TH0>;
#[doc = "Register clk_th0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_TH0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sram0_gclk`"]
pub type SRAM0_GCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sram0_gclk`"]
pub struct SRAM0_GCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0_GCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `sram1_gclk`"]
pub type SRAM1_GCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sram1_gclk`"]
pub struct SRAM1_GCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1_GCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ai_gclk`"]
pub type AI_GCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ai_gclk`"]
pub struct AI_GCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_GCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `dvp_gclk`"]
pub type DVP_GCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dvp_gclk`"]
pub struct DVP_GCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DVP_GCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `rom_gclk`"]
pub type ROM_GCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rom_gclk`"]
pub struct ROM_GCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_GCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sram0_gclk(&self) -> SRAM0_GCLK_R {
        SRAM0_GCLK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sram1_gclk(&self) -> SRAM1_GCLK_R {
        SRAM1_GCLK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ai_gclk(&self) -> AI_GCLK_R {
        AI_GCLK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dvp_gclk(&self) -> DVP_GCLK_R {
        DVP_GCLK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rom_gclk(&self) -> ROM_GCLK_R {
        ROM_GCLK_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sram0_gclk(&mut self) -> SRAM0_GCLK_W {
        SRAM0_GCLK_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sram1_gclk(&mut self) -> SRAM1_GCLK_W {
        SRAM1_GCLK_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ai_gclk(&mut self) -> AI_GCLK_W {
        AI_GCLK_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn dvp_gclk(&mut self) -> DVP_GCLK_W {
        DVP_GCLK_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rom_gclk(&mut self) -> ROM_GCLK_W {
        ROM_GCLK_W { w: self }
    }
}
