#[doc = "Reader of register io[%s]"]
pub type R = crate::R<u32, super::IO>;
#[doc = "Writer for register io[%s]"]
pub type W = crate::W<u32, super::IO>;
#[doc = "Register io[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::IO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ch_sel`"]
pub type CH_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ch_sel`"]
pub struct CH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ds`"]
pub type DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ds`"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `oe_en`"]
pub type OE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `oe_en`"]
pub struct OE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_EN_W<'a> {
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
#[doc = "Reader of field `oe_inv`"]
pub type OE_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `oe_inv`"]
pub struct OE_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `do_sel`"]
pub type DO_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `do_sel`"]
pub struct DO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `do_inv`"]
pub type DO_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `do_inv`"]
pub struct DO_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DO_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `pu`"]
pub type PU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pu`"]
pub struct PU_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_W<'a> {
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
#[doc = "Reader of field `pd`"]
pub type PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pd`"]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
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
#[doc = "Reader of field `sl`"]
pub type SL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sl`"]
pub struct SL_W<'a> {
    w: &'a mut W,
}
impl<'a> SL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ie_en`"]
pub type IE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ie_en`"]
pub struct IE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_EN_W<'a> {
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
#[doc = "Reader of field `ie_inv`"]
pub type IE_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ie_inv`"]
pub struct IE_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_INV_W<'a> {
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
#[doc = "Reader of field `di_inv`"]
pub type DI_INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `di_inv`"]
pub struct DI_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DI_INV_W<'a> {
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
#[doc = "Reader of field `st`"]
pub type ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `st`"]
pub struct ST_W<'a> {
    w: &'a mut W,
}
impl<'a> ST_W<'a> {
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
#[doc = "Reader of field `pad_di`"]
pub type PAD_DI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pad_di`"]
pub struct PAD_DI_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_DI_W<'a> {
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
    #[doc = "Bits 0:7 - Channel select from 256 input"]
    #[inline(always)]
    pub fn ch_sel(&self) -> CH_SEL_R {
        CH_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Driving selector"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Static output enable, will AND with OE_INV"]
    #[inline(always)]
    pub fn oe_en(&self) -> OE_EN_R {
        OE_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Invert output enable"]
    #[inline(always)]
    pub fn oe_inv(&self) -> OE_INV_R {
        OE_INV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Data output select: 0 for DO, 1 for OE"]
    #[inline(always)]
    pub fn do_sel(&self) -> DO_SEL_R {
        DO_SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Invert the result of data output select (DO_SEL)"]
    #[inline(always)]
    pub fn do_inv(&self) -> DO_INV_R {
        DO_INV_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pull up enable. 0 for nothing, 1 for pull up"]
    #[inline(always)]
    pub fn pu(&self) -> PU_R {
        PU_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pull down enable. 0 for nothing, 1 for pull down"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Slew rate control enable"]
    #[inline(always)]
    pub fn sl(&self) -> SL_R {
        SL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Static input enable, will AND with IE_INV"]
    #[inline(always)]
    pub fn ie_en(&self) -> IE_EN_R {
        IE_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Invert input enable"]
    #[inline(always)]
    pub fn ie_inv(&self) -> IE_INV_R {
        IE_INV_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Invert Data input"]
    #[inline(always)]
    pub fn di_inv(&self) -> DI_INV_R {
        DI_INV_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Schmitt trigger"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Read current IO's data input"]
    #[inline(always)]
    pub fn pad_di(&self) -> PAD_DI_R {
        PAD_DI_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel select from 256 input"]
    #[inline(always)]
    pub fn ch_sel(&mut self) -> CH_SEL_W {
        CH_SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Driving selector"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Bit 12 - Static output enable, will AND with OE_INV"]
    #[inline(always)]
    pub fn oe_en(&mut self) -> OE_EN_W {
        OE_EN_W { w: self }
    }
    #[doc = "Bit 13 - Invert output enable"]
    #[inline(always)]
    pub fn oe_inv(&mut self) -> OE_INV_W {
        OE_INV_W { w: self }
    }
    #[doc = "Bit 14 - Data output select: 0 for DO, 1 for OE"]
    #[inline(always)]
    pub fn do_sel(&mut self) -> DO_SEL_W {
        DO_SEL_W { w: self }
    }
    #[doc = "Bit 15 - Invert the result of data output select (DO_SEL)"]
    #[inline(always)]
    pub fn do_inv(&mut self) -> DO_INV_W {
        DO_INV_W { w: self }
    }
    #[doc = "Bit 16 - Pull up enable. 0 for nothing, 1 for pull up"]
    #[inline(always)]
    pub fn pu(&mut self) -> PU_W {
        PU_W { w: self }
    }
    #[doc = "Bit 17 - Pull down enable. 0 for nothing, 1 for pull down"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Bit 19 - Slew rate control enable"]
    #[inline(always)]
    pub fn sl(&mut self) -> SL_W {
        SL_W { w: self }
    }
    #[doc = "Bit 20 - Static input enable, will AND with IE_INV"]
    #[inline(always)]
    pub fn ie_en(&mut self) -> IE_EN_W {
        IE_EN_W { w: self }
    }
    #[doc = "Bit 21 - Invert input enable"]
    #[inline(always)]
    pub fn ie_inv(&mut self) -> IE_INV_W {
        IE_INV_W { w: self }
    }
    #[doc = "Bit 22 - Invert Data input"]
    #[inline(always)]
    pub fn di_inv(&mut self) -> DI_INV_W {
        DI_INV_W { w: self }
    }
    #[doc = "Bit 23 - Schmitt trigger"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W {
        ST_W { w: self }
    }
    #[doc = "Bit 31 - Read current IO's data input"]
    #[inline(always)]
    pub fn pad_di(&mut self) -> PAD_DI_W {
        PAD_DI_W { w: self }
    }
}
