#[doc = "Reader of register comp_param_1"]
pub type R = crate::R<u32, super::COMP_PARAM_1>;
#[doc = "Writer for register comp_param_1"]
pub type W = crate::W<u32, super::COMP_PARAM_1>;
#[doc = "Register comp_param_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP_PARAM_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `always_en`"]
pub type ALWAYS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `always_en`"]
pub struct ALWAYS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWAYS_EN_W<'a> {
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
#[doc = "Reader of field `dflt_rmod`"]
pub type DFLT_RMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dflt_rmod`"]
pub struct DFLT_RMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLT_RMOD_W<'a> {
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
#[doc = "Reader of field `dual_top`"]
pub type DUAL_TOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dual_top`"]
pub struct DUAL_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL_TOP_W<'a> {
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
#[doc = "Reader of field `hc_rmod`"]
pub type HC_RMOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hc_rmod`"]
pub struct HC_RMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> HC_RMOD_W<'a> {
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
#[doc = "Reader of field `hc_rpl`"]
pub type HC_RPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hc_rpl`"]
pub struct HC_RPL_W<'a> {
    w: &'a mut W,
}
impl<'a> HC_RPL_W<'a> {
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
#[doc = "Reader of field `hc_top`"]
pub type HC_TOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hc_top`"]
pub struct HC_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> HC_TOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `use_fix_top`"]
pub type USE_FIX_TOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `use_fix_top`"]
pub struct USE_FIX_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> USE_FIX_TOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `pause`"]
pub type PAUSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pause`"]
pub struct PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `apb_data_width`"]
pub type APB_DATA_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `apb_data_width`"]
pub struct APB_DATA_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_DATA_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `dflt_rpl`"]
pub type DFLT_RPL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dflt_rpl`"]
pub struct DFLT_RPL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLT_RPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `dflt_top`"]
pub type DFLT_TOP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dflt_top`"]
pub struct DFLT_TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLT_TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `dflt_top_init`"]
pub type DFLT_TOP_INIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dflt_top_init`"]
pub struct DFLT_TOP_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLT_TOP_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `cnt_width`"]
pub type CNT_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `cnt_width`"]
pub struct CNT_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - always_en"]
    #[inline(always)]
    pub fn always_en(&self) -> ALWAYS_EN_R {
        ALWAYS_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - dflt_rmod"]
    #[inline(always)]
    pub fn dflt_rmod(&self) -> DFLT_RMOD_R {
        DFLT_RMOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - dual_top"]
    #[inline(always)]
    pub fn dual_top(&self) -> DUAL_TOP_R {
        DUAL_TOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - hc_rmod"]
    #[inline(always)]
    pub fn hc_rmod(&self) -> HC_RMOD_R {
        HC_RMOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - hc_rpl"]
    #[inline(always)]
    pub fn hc_rpl(&self) -> HC_RPL_R {
        HC_RPL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - hc_top"]
    #[inline(always)]
    pub fn hc_top(&self) -> HC_TOP_R {
        HC_TOP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - use_fix_top"]
    #[inline(always)]
    pub fn use_fix_top(&self) -> USE_FIX_TOP_R {
        USE_FIX_TOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - pause"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - apb_data_width"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> APB_DATA_WIDTH_R {
        APB_DATA_WIDTH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12 - dflt_rpl"]
    #[inline(always)]
    pub fn dflt_rpl(&self) -> DFLT_RPL_R {
        DFLT_RPL_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - dflt_top"]
    #[inline(always)]
    pub fn dflt_top(&self) -> DFLT_TOP_R {
        DFLT_TOP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - dflt_top_init"]
    #[inline(always)]
    pub fn dflt_top_init(&self) -> DFLT_TOP_INIT_R {
        DFLT_TOP_INIT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - cnt_width"]
    #[inline(always)]
    pub fn cnt_width(&self) -> CNT_WIDTH_R {
        CNT_WIDTH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - always_en"]
    #[inline(always)]
    pub fn always_en(&mut self) -> ALWAYS_EN_W {
        ALWAYS_EN_W { w: self }
    }
    #[doc = "Bit 1 - dflt_rmod"]
    #[inline(always)]
    pub fn dflt_rmod(&mut self) -> DFLT_RMOD_W {
        DFLT_RMOD_W { w: self }
    }
    #[doc = "Bit 2 - dual_top"]
    #[inline(always)]
    pub fn dual_top(&mut self) -> DUAL_TOP_W {
        DUAL_TOP_W { w: self }
    }
    #[doc = "Bit 3 - hc_rmod"]
    #[inline(always)]
    pub fn hc_rmod(&mut self) -> HC_RMOD_W {
        HC_RMOD_W { w: self }
    }
    #[doc = "Bit 4 - hc_rpl"]
    #[inline(always)]
    pub fn hc_rpl(&mut self) -> HC_RPL_W {
        HC_RPL_W { w: self }
    }
    #[doc = "Bit 5 - hc_top"]
    #[inline(always)]
    pub fn hc_top(&mut self) -> HC_TOP_W {
        HC_TOP_W { w: self }
    }
    #[doc = "Bit 6 - use_fix_top"]
    #[inline(always)]
    pub fn use_fix_top(&mut self) -> USE_FIX_TOP_W {
        USE_FIX_TOP_W { w: self }
    }
    #[doc = "Bit 7 - pause"]
    #[inline(always)]
    pub fn pause(&mut self) -> PAUSE_W {
        PAUSE_W { w: self }
    }
    #[doc = "Bits 8:9 - apb_data_width"]
    #[inline(always)]
    pub fn apb_data_width(&mut self) -> APB_DATA_WIDTH_W {
        APB_DATA_WIDTH_W { w: self }
    }
    #[doc = "Bits 10:12 - dflt_rpl"]
    #[inline(always)]
    pub fn dflt_rpl(&mut self) -> DFLT_RPL_W {
        DFLT_RPL_W { w: self }
    }
    #[doc = "Bits 16:19 - dflt_top"]
    #[inline(always)]
    pub fn dflt_top(&mut self) -> DFLT_TOP_W {
        DFLT_TOP_W { w: self }
    }
    #[doc = "Bits 20:23 - dflt_top_init"]
    #[inline(always)]
    pub fn dflt_top_init(&mut self) -> DFLT_TOP_INIT_W {
        DFLT_TOP_INIT_W { w: self }
    }
    #[doc = "Bits 24:28 - cnt_width"]
    #[inline(always)]
    pub fn cnt_width(&mut self) -> CNT_WIDTH_W {
        CNT_WIDTH_W { w: self }
    }
}
