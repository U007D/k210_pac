#[doc = "Reader of register sts"]
pub type R = crate::R<u32, super::STS>;
#[doc = "Writer for register sts"]
pub type W = crate::W<u32, super::STS>;
#[doc = "Register sts `reset()`'s with value 0"]
impl crate::ResetValue for super::STS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `frame_start`"]
pub type FRAME_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `frame_start`"]
pub struct FRAME_START_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_START_W<'a> {
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
#[doc = "Reader of field `frame_start_we`"]
pub type FRAME_START_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `frame_start_we`"]
pub struct FRAME_START_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_START_WE_W<'a> {
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
#[doc = "Reader of field `frame_finish`"]
pub type FRAME_FINISH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `frame_finish`"]
pub struct FRAME_FINISH_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_FINISH_W<'a> {
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
#[doc = "Reader of field `frame_finish_we`"]
pub type FRAME_FINISH_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `frame_finish_we`"]
pub struct FRAME_FINISH_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_FINISH_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `dvp_en`"]
pub type DVP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dvp_en`"]
pub struct DVP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DVP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `dvp_en_we`"]
pub type DVP_EN_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dvp_en_we`"]
pub struct DVP_EN_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> DVP_EN_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `sccb_en`"]
pub type SCCB_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sccb_en`"]
pub struct SCCB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCCB_EN_W<'a> {
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
#[doc = "Reader of field `sccb_en_we`"]
pub type SCCB_EN_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sccb_en_we`"]
pub struct SCCB_EN_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCCB_EN_WE_W<'a> {
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
    #[doc = "Bit 0 - FRAME_START"]
    #[inline(always)]
    pub fn frame_start(&self) -> FRAME_START_R {
        FRAME_START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FRAME_START_WE"]
    #[inline(always)]
    pub fn frame_start_we(&self) -> FRAME_START_WE_R {
        FRAME_START_WE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FRAME_FINISH"]
    #[inline(always)]
    pub fn frame_finish(&self) -> FRAME_FINISH_R {
        FRAME_FINISH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FRAME_FINISH_WE"]
    #[inline(always)]
    pub fn frame_finish_we(&self) -> FRAME_FINISH_WE_R {
        FRAME_FINISH_WE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DVP_EN"]
    #[inline(always)]
    pub fn dvp_en(&self) -> DVP_EN_R {
        DVP_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DVP_EN_WE"]
    #[inline(always)]
    pub fn dvp_en_we(&self) -> DVP_EN_WE_R {
        DVP_EN_WE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SCCB_EN"]
    #[inline(always)]
    pub fn sccb_en(&self) -> SCCB_EN_R {
        SCCB_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - SCCB_EN_WE"]
    #[inline(always)]
    pub fn sccb_en_we(&self) -> SCCB_EN_WE_R {
        SCCB_EN_WE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FRAME_START"]
    #[inline(always)]
    pub fn frame_start(&mut self) -> FRAME_START_W {
        FRAME_START_W { w: self }
    }
    #[doc = "Bit 1 - FRAME_START_WE"]
    #[inline(always)]
    pub fn frame_start_we(&mut self) -> FRAME_START_WE_W {
        FRAME_START_WE_W { w: self }
    }
    #[doc = "Bit 8 - FRAME_FINISH"]
    #[inline(always)]
    pub fn frame_finish(&mut self) -> FRAME_FINISH_W {
        FRAME_FINISH_W { w: self }
    }
    #[doc = "Bit 9 - FRAME_FINISH_WE"]
    #[inline(always)]
    pub fn frame_finish_we(&mut self) -> FRAME_FINISH_WE_W {
        FRAME_FINISH_WE_W { w: self }
    }
    #[doc = "Bit 16 - DVP_EN"]
    #[inline(always)]
    pub fn dvp_en(&mut self) -> DVP_EN_W {
        DVP_EN_W { w: self }
    }
    #[doc = "Bit 17 - DVP_EN_WE"]
    #[inline(always)]
    pub fn dvp_en_we(&mut self) -> DVP_EN_WE_W {
        DVP_EN_WE_W { w: self }
    }
    #[doc = "Bit 24 - SCCB_EN"]
    #[inline(always)]
    pub fn sccb_en(&mut self) -> SCCB_EN_W {
        SCCB_EN_W { w: self }
    }
    #[doc = "Bit 25 - SCCB_EN_WE"]
    #[inline(always)]
    pub fn sccb_en_we(&mut self) -> SCCB_EN_WE_W {
        SCCB_EN_WE_W { w: self }
    }
}
