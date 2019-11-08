#[doc = "Reader of register ctl"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register ctl"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register ctl `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dir_search_en`"]
pub type DIR_SEARCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dir_search_en`"]
pub struct DIR_SEARCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_SEARCH_EN_W<'a> {
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
#[doc = "Reader of field `search_path_reset`"]
pub type SEARCH_PATH_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `search_path_reset`"]
pub struct SEARCH_PATH_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SEARCH_PATH_RESET_W<'a> {
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
#[doc = "Reader of field `stream_gen_en`"]
pub type STREAM_GEN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `stream_gen_en`"]
pub struct STREAM_GEN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> STREAM_GEN_EN_W<'a> {
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
#[doc = "Reader of field `voice_gen_path_reset`"]
pub type VOICE_GEN_PATH_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `voice_gen_path_reset`"]
pub struct VOICE_GEN_PATH_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> VOICE_GEN_PATH_RESET_W<'a> {
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
#[doc = "Reader of field `update_voice_dir`"]
pub type UPDATE_VOICE_DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `update_voice_dir`"]
pub struct UPDATE_VOICE_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATE_VOICE_DIR_W<'a> {
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
#[doc = "Write proxy for field `we_dir_search_en`"]
pub struct WE_DIR_SEARCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_DIR_SEARCH_EN_W<'a> {
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
#[doc = "Write proxy for field `we_search_path_rst`"]
pub struct WE_SEARCH_PATH_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_SEARCH_PATH_RST_W<'a> {
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
#[doc = "Write proxy for field `we_stream_gen`"]
pub struct WE_STREAM_GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_STREAM_GEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `we_voice_gen_path_rst`"]
pub struct WE_VOICE_GEN_PATH_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_VOICE_GEN_PATH_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `we_update_voice_dir`"]
pub struct WE_UPDATE_VOICE_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_UPDATE_VOICE_DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sound direction searching enable bit"]
    #[inline(always)]
    pub fn dir_search_en(&self) -> DIR_SEARCH_EN_R {
        DIR_SEARCH_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset all control logic on direction search processing path"]
    #[inline(always)]
    pub fn search_path_reset(&self) -> SEARCH_PATH_RESET_R {
        SEARCH_PATH_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Valid voice sample stream generation enable bit"]
    #[inline(always)]
    pub fn stream_gen_en(&self) -> STREAM_GEN_EN_R {
        STREAM_GEN_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset all control logic on voice stream generating path"]
    #[inline(always)]
    pub fn voice_gen_path_reset(&self) -> VOICE_GEN_PATH_RESET_R {
        VOICE_GEN_PATH_RESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Switch to a new voice source direction"]
    #[inline(always)]
    pub fn update_voice_dir(&self) -> UPDATE_VOICE_DIR_R {
        UPDATE_VOICE_DIR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sound direction searching enable bit"]
    #[inline(always)]
    pub fn dir_search_en(&mut self) -> DIR_SEARCH_EN_W {
        DIR_SEARCH_EN_W { w: self }
    }
    #[doc = "Bit 1 - Reset all control logic on direction search processing path"]
    #[inline(always)]
    pub fn search_path_reset(&mut self) -> SEARCH_PATH_RESET_W {
        SEARCH_PATH_RESET_W { w: self }
    }
    #[doc = "Bit 4 - Valid voice sample stream generation enable bit"]
    #[inline(always)]
    pub fn stream_gen_en(&mut self) -> STREAM_GEN_EN_W {
        STREAM_GEN_EN_W { w: self }
    }
    #[doc = "Bit 5 - Reset all control logic on voice stream generating path"]
    #[inline(always)]
    pub fn voice_gen_path_reset(&mut self) -> VOICE_GEN_PATH_RESET_W {
        VOICE_GEN_PATH_RESET_W { w: self }
    }
    #[doc = "Bit 6 - Switch to a new voice source direction"]
    #[inline(always)]
    pub fn update_voice_dir(&mut self) -> UPDATE_VOICE_DIR_W {
        UPDATE_VOICE_DIR_W { w: self }
    }
    #[doc = "Bit 8 - Write enable for we_dir_search_en parameter"]
    #[inline(always)]
    pub fn we_dir_search_en(&mut self) -> WE_DIR_SEARCH_EN_W {
        WE_DIR_SEARCH_EN_W { w: self }
    }
    #[doc = "Bit 9 - Write enable for we_search_path_rst parameter"]
    #[inline(always)]
    pub fn we_search_path_rst(&mut self) -> WE_SEARCH_PATH_RST_W {
        WE_SEARCH_PATH_RST_W { w: self }
    }
    #[doc = "Bit 10 - Write enable for we_stream_gen parameter"]
    #[inline(always)]
    pub fn we_stream_gen(&mut self) -> WE_STREAM_GEN_W {
        WE_STREAM_GEN_W { w: self }
    }
    #[doc = "Bit 11 - Write enable for we_voice_gen_path_rst parameter"]
    #[inline(always)]
    pub fn we_voice_gen_path_rst(&mut self) -> WE_VOICE_GEN_PATH_RST_W {
        WE_VOICE_GEN_PATH_RST_W { w: self }
    }
    #[doc = "Bit 12 - Write enable for we_update_voice_dir parameter"]
    #[inline(always)]
    pub fn we_update_voice_dir(&mut self) -> WE_UPDATE_VOICE_DIR_W {
        WE_UPDATE_VOICE_DIR_W { w: self }
    }
}
