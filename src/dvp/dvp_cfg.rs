#[doc = "Reader of register dvp_cfg"]
pub type R = crate::R<u32, super::DVP_CFG>;
#[doc = "Writer for register dvp_cfg"]
pub type W = crate::W<u32, super::DVP_CFG>;
#[doc = "Register dvp_cfg `reset()`'s with value 0"]
impl crate::ResetValue for super::DVP_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `start_int_enable`"]
pub type START_INT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `start_int_enable`"]
pub struct START_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> START_INT_ENABLE_W<'a> {
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
#[doc = "Reader of field `finish_int_enable`"]
pub type FINISH_INT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `finish_int_enable`"]
pub struct FINISH_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FINISH_INT_ENABLE_W<'a> {
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
#[doc = "Reader of field `ai_output_enable`"]
pub type AI_OUTPUT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ai_output_enable`"]
pub struct AI_OUTPUT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AI_OUTPUT_ENABLE_W<'a> {
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
#[doc = "Reader of field `display_output_enable`"]
pub type DISPLAY_OUTPUT_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `display_output_enable`"]
pub struct DISPLAY_OUTPUT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISPLAY_OUTPUT_ENABLE_W<'a> {
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
#[doc = "Reader of field `auto_enable`"]
pub type AUTO_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `auto_enable`"]
pub struct AUTO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_ENABLE_W<'a> {
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
#[doc = "Reader of field `burst_size_4beats`"]
pub type BURST_SIZE_4BEATS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `burst_size_4beats`"]
pub struct BURST_SIZE_4BEATS_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_SIZE_4BEATS_W<'a> {
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
#[doc = "FORMAT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORMAT_A {
    #[doc = "0: RGB_FORMAT"]
    RGB,
    #[doc = "1: YUV_FORMAT"]
    YUV,
    #[doc = "3: Y_FORMAT"]
    Y,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        match variant {
            FORMAT_A::RGB => 0,
            FORMAT_A::YUV => 1,
            FORMAT_A::Y => 3,
        }
    }
}
#[doc = "Reader of field `format`"]
pub type FORMAT_R = crate::R<u8, FORMAT_A>;
impl FORMAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FORMAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FORMAT_A::RGB),
            1 => Val(FORMAT_A::YUV),
            3 => Val(FORMAT_A::Y),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        *self == FORMAT_A::RGB
    }
    #[doc = "Checks if the value of the field is `YUV`"]
    #[inline(always)]
    pub fn is_yuv(&self) -> bool {
        *self == FORMAT_A::YUV
    }
    #[doc = "Checks if the value of the field is `Y`"]
    #[inline(always)]
    pub fn is_y(&self) -> bool {
        *self == FORMAT_A::Y
    }
}
#[doc = "Write proxy for field `format`"]
pub struct FORMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> FORMAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORMAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RGB_FORMAT"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut W {
        self.variant(FORMAT_A::RGB)
    }
    #[doc = "YUV_FORMAT"]
    #[inline(always)]
    pub fn yuv(self) -> &'a mut W {
        self.variant(FORMAT_A::YUV)
    }
    #[doc = "Y_FORMAT"]
    #[inline(always)]
    pub fn y(self) -> &'a mut W {
        self.variant(FORMAT_A::Y)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `href_burst_num`"]
pub type HREF_BURST_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `href_burst_num`"]
pub struct HREF_BURST_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> HREF_BURST_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `line_num`"]
pub type LINE_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `line_num`"]
pub struct LINE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - START_INT_ENABLE"]
    #[inline(always)]
    pub fn start_int_enable(&self) -> START_INT_ENABLE_R {
        START_INT_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FINISH_INT_ENABLE"]
    #[inline(always)]
    pub fn finish_int_enable(&self) -> FINISH_INT_ENABLE_R {
        FINISH_INT_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AI_OUTPUT_ENABLE"]
    #[inline(always)]
    pub fn ai_output_enable(&self) -> AI_OUTPUT_ENABLE_R {
        AI_OUTPUT_ENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DISPLAY_OUTPUT_ENABLE"]
    #[inline(always)]
    pub fn display_output_enable(&self) -> DISPLAY_OUTPUT_ENABLE_R {
        DISPLAY_OUTPUT_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUTO_ENABLE"]
    #[inline(always)]
    pub fn auto_enable(&self) -> AUTO_ENABLE_R {
        AUTO_ENABLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BURST_SIZE_4BEATS"]
    #[inline(always)]
    pub fn burst_size_4beats(&self) -> BURST_SIZE_4BEATS_R {
        BURST_SIZE_4BEATS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - FORMAT"]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 12:19 - HREF_BURST_NUM"]
    #[inline(always)]
    pub fn href_burst_num(&self) -> HREF_BURST_NUM_R {
        HREF_BURST_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:29 - LINE_NUM"]
    #[inline(always)]
    pub fn line_num(&self) -> LINE_NUM_R {
        LINE_NUM_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - START_INT_ENABLE"]
    #[inline(always)]
    pub fn start_int_enable(&mut self) -> START_INT_ENABLE_W {
        START_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - FINISH_INT_ENABLE"]
    #[inline(always)]
    pub fn finish_int_enable(&mut self) -> FINISH_INT_ENABLE_W {
        FINISH_INT_ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - AI_OUTPUT_ENABLE"]
    #[inline(always)]
    pub fn ai_output_enable(&mut self) -> AI_OUTPUT_ENABLE_W {
        AI_OUTPUT_ENABLE_W { w: self }
    }
    #[doc = "Bit 3 - DISPLAY_OUTPUT_ENABLE"]
    #[inline(always)]
    pub fn display_output_enable(&mut self) -> DISPLAY_OUTPUT_ENABLE_W {
        DISPLAY_OUTPUT_ENABLE_W { w: self }
    }
    #[doc = "Bit 4 - AUTO_ENABLE"]
    #[inline(always)]
    pub fn auto_enable(&mut self) -> AUTO_ENABLE_W {
        AUTO_ENABLE_W { w: self }
    }
    #[doc = "Bit 8 - BURST_SIZE_4BEATS"]
    #[inline(always)]
    pub fn burst_size_4beats(&mut self) -> BURST_SIZE_4BEATS_W {
        BURST_SIZE_4BEATS_W { w: self }
    }
    #[doc = "Bits 9:10 - FORMAT"]
    #[inline(always)]
    pub fn format(&mut self) -> FORMAT_W {
        FORMAT_W { w: self }
    }
    #[doc = "Bits 12:19 - HREF_BURST_NUM"]
    #[inline(always)]
    pub fn href_burst_num(&mut self) -> HREF_BURST_NUM_W {
        HREF_BURST_NUM_W { w: self }
    }
    #[doc = "Bits 20:29 - LINE_NUM"]
    #[inline(always)]
    pub fn line_num(&mut self) -> LINE_NUM_W {
        LINE_NUM_W { w: self }
    }
}
