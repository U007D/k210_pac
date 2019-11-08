#[doc = "Reader of register dwsz_cfg"]
pub type R = crate::R<u32, super::DWSZ_CFG>;
#[doc = "Writer for register dwsz_cfg"]
pub type W = crate::W<u32, super::DWSZ_CFG>;
#[doc = "Register dwsz_cfg `reset()`'s with value 0"]
impl crate::ResetValue for super::DWSZ_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dir_dwn_siz_rate`"]
pub type DIR_DWN_SIZ_RATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dir_dwn_siz_rate`"]
pub struct DIR_DWN_SIZ_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_DWN_SIZ_RATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `voc_dwn_siz_rate`"]
pub type VOC_DWN_SIZ_RATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `voc_dwn_siz_rate`"]
pub struct VOC_DWN_SIZ_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> VOC_DWN_SIZ_RATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `smpl_shift_bits`"]
pub type SMPL_SHIFT_BITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `smpl_shift_bits`"]
pub struct SMPL_SHIFT_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPL_SHIFT_BITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Down-sizing ratio used for direction searching"]
    #[inline(always)]
    pub fn dir_dwn_siz_rate(&self) -> DIR_DWN_SIZ_RATE_R {
        DIR_DWN_SIZ_RATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Down-sizing ratio used for voice stream generation"]
    #[inline(always)]
    pub fn voc_dwn_siz_rate(&self) -> VOC_DWN_SIZ_RATE_R {
        VOC_DWN_SIZ_RATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Sample precision reduction when the source sound sample precision is 20/24/32 bits"]
    #[inline(always)]
    pub fn smpl_shift_bits(&self) -> SMPL_SHIFT_BITS_R {
        SMPL_SHIFT_BITS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Down-sizing ratio used for direction searching"]
    #[inline(always)]
    pub fn dir_dwn_siz_rate(&mut self) -> DIR_DWN_SIZ_RATE_W {
        DIR_DWN_SIZ_RATE_W { w: self }
    }
    #[doc = "Bits 4:7 - Down-sizing ratio used for voice stream generation"]
    #[inline(always)]
    pub fn voc_dwn_siz_rate(&mut self) -> VOC_DWN_SIZ_RATE_W {
        VOC_DWN_SIZ_RATE_W { w: self }
    }
    #[doc = "Bits 8:12 - Sample precision reduction when the source sound sample precision is 20/24/32 bits"]
    #[inline(always)]
    pub fn smpl_shift_bits(&mut self) -> SMPL_SHIFT_BITS_W {
        SMPL_SHIFT_BITS_W { w: self }
    }
}
