#[doc = "Reader of register pll1"]
pub type R = crate::R<u32, super::PLL1>;
#[doc = "Writer for register pll1"]
pub type W = crate::W<u32, super::PLL1>;
#[doc = "Register pll1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PLL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `clkr`"]
pub type CLKR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkr`"]
pub struct CLKR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `clkf`"]
pub type CLKF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkf`"]
pub struct CLKF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Reader of field `clkod`"]
pub type CLKOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clkod`"]
pub struct CLKOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Reader of field `bwadj`"]
pub type BWADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `bwadj`"]
pub struct BWADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> BWADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | (((value as u32) & 0x3f) << 14);
        self.w
    }
}
#[doc = "Reader of field `reset`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reset`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Reader of field `pwrd`"]
pub type PWRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pwrd`"]
pub struct PWRD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRD_W<'a> {
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
#[doc = "Reader of field `intfb`"]
pub type INTFB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `intfb`"]
pub struct INTFB_W<'a> {
    w: &'a mut W,
}
impl<'a> INTFB_W<'a> {
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
#[doc = "Reader of field `bypass`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bypass`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Reader of field `test`"]
pub type TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `test`"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
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
#[doc = "Reader of field `out_en`"]
pub type OUT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `out_en`"]
pub struct OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_EN_W<'a> {
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
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn clkr(&self) -> CLKR_R {
        CLKR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    pub fn clkf(&self) -> CLKF_R {
        CLKF_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn clkod(&self) -> CLKOD_R {
        CLKOD_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn bwadj(&self) -> BWADJ_R {
        BWADJ_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pwrd(&self) -> PWRD_R {
        PWRD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn intfb(&self) -> INTFB_R {
        INTFB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn clkr(&mut self) -> CLKR_W {
        CLKR_W { w: self }
    }
    #[doc = "Bits 4:9"]
    #[inline(always)]
    pub fn clkf(&mut self) -> CLKF_W {
        CLKF_W { w: self }
    }
    #[doc = "Bits 10:13"]
    #[inline(always)]
    pub fn clkod(&mut self) -> CLKOD_W {
        CLKOD_W { w: self }
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn bwadj(&mut self) -> BWADJ_W {
        BWADJ_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pwrd(&mut self) -> PWRD_W {
        PWRD_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn intfb(&mut self) -> INTFB_W {
        INTFB_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn out_en(&mut self) -> OUT_EN_W {
        OUT_EN_W { w: self }
    }
}
