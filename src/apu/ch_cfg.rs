#[doc = "Reader of register ch_cfg"]
pub type R = crate::R<u32, super::CH_CFG>;
#[doc = "Writer for register ch_cfg"]
pub type W = crate::W<u32, super::CH_CFG>;
#[doc = "Register ch_cfg `reset()`'s with value 0"]
impl crate::ResetValue for super::CH_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `sound_ch_en`"]
pub type SOUND_CH_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sound_ch_en`"]
pub struct SOUND_CH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOUND_CH_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `target_dir`"]
pub type TARGET_DIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `target_dir`"]
pub struct TARGET_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> TARGET_DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `audio_gain`"]
pub type AUDIO_GAIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `audio_gain`"]
pub struct AUDIO_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDIO_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 12)) | (((value as u32) & 0x07ff) << 12);
        self.w
    }
}
#[doc = "Reader of field `data_src_mode`"]
pub type DATA_SRC_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `data_src_mode`"]
pub struct DATA_SRC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_SRC_MODE_W<'a> {
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
#[doc = "Write proxy for field `we_sound_ch_en`"]
pub struct WE_SOUND_CH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_SOUND_CH_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `we_target_dir`"]
pub struct WE_TARGET_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_TARGET_DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Write proxy for field `we_audio_gain`"]
pub struct WE_AUDIO_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_AUDIO_GAIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `we_data_src_mode`"]
pub struct WE_DATA_SRC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_DATA_SRC_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - BF unit sound channel enable control bits"]
    #[inline(always)]
    pub fn sound_ch_en(&self) -> SOUND_CH_EN_R {
        SOUND_CH_EN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Target direction select for valid voice output"]
    #[inline(always)]
    pub fn target_dir(&self) -> TARGET_DIR_R {
        TARGET_DIR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:22 - Audio sample gain factor"]
    #[inline(always)]
    pub fn audio_gain(&self) -> AUDIO_GAIN_R {
        AUDIO_GAIN_R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
    #[doc = "Bit 24 - Audio data source configure parameter"]
    #[inline(always)]
    pub fn data_src_mode(&self) -> DATA_SRC_MODE_R {
        DATA_SRC_MODE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - BF unit sound channel enable control bits"]
    #[inline(always)]
    pub fn sound_ch_en(&mut self) -> SOUND_CH_EN_W {
        SOUND_CH_EN_W { w: self }
    }
    #[doc = "Bits 8:11 - Target direction select for valid voice output"]
    #[inline(always)]
    pub fn target_dir(&mut self) -> TARGET_DIR_W {
        TARGET_DIR_W { w: self }
    }
    #[doc = "Bits 12:22 - Audio sample gain factor"]
    #[inline(always)]
    pub fn audio_gain(&mut self) -> AUDIO_GAIN_W {
        AUDIO_GAIN_W { w: self }
    }
    #[doc = "Bit 24 - Audio data source configure parameter"]
    #[inline(always)]
    pub fn data_src_mode(&mut self) -> DATA_SRC_MODE_W {
        DATA_SRC_MODE_W { w: self }
    }
    #[doc = "Bit 28 - Write enable for sound_ch_en parameter"]
    #[inline(always)]
    pub fn we_sound_ch_en(&mut self) -> WE_SOUND_CH_EN_W {
        WE_SOUND_CH_EN_W { w: self }
    }
    #[doc = "Bit 29 - Write enable for target_dir parameter"]
    #[inline(always)]
    pub fn we_target_dir(&mut self) -> WE_TARGET_DIR_W {
        WE_TARGET_DIR_W { w: self }
    }
    #[doc = "Bit 30 - Write enable for audio_gain parameter"]
    #[inline(always)]
    pub fn we_audio_gain(&mut self) -> WE_AUDIO_GAIN_W {
        WE_AUDIO_GAIN_W { w: self }
    }
    #[doc = "Bit 31 - Write enable for data_out_mode parameter"]
    #[inline(always)]
    pub fn we_data_src_mode(&mut self) -> WE_DATA_SRC_MODE_W {
        WE_DATA_SRC_MODE_W { w: self }
    }
}
