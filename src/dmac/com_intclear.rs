#[doc = "Reader of register com_intclear"]
pub type R = crate::R<u64, super::COM_INTCLEAR>;
#[doc = "Writer for register com_intclear"]
pub type W = crate::W<u64, super::COM_INTCLEAR>;
#[doc = "Register com_intclear `reset()`'s with value 0"]
impl crate::ResetValue for super::COM_INTCLEAR {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `slvif_dec_err`"]
pub type SLVIF_DEC_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `slvif_dec_err`"]
pub struct SLVIF_DEC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVIF_DEC_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u64) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `slvif_wr2ro_err`"]
pub type SLVIF_WR2RO_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `slvif_wr2ro_err`"]
pub struct SLVIF_WR2RO_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVIF_WR2RO_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u64) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `slvif_rd2wo_err`"]
pub type SLVIF_RD2WO_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `slvif_rd2wo_err`"]
pub struct SLVIF_RD2WO_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVIF_RD2WO_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u64) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `slvif_wronhold_err`"]
pub type SLVIF_WRONHOLD_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `slvif_wronhold_err`"]
pub struct SLVIF_WRONHOLD_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVIF_WRONHOLD_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u64) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `slvif_undefinedreg_dec_err`"]
pub type SLVIF_UNDEFINEDREG_DEC_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `slvif_undefinedreg_dec_err`"]
pub struct SLVIF_UNDEFINEDREG_DEC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVIF_UNDEFINEDREG_DEC_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u64) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clear slvif_dec_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_dec_err(&self) -> SLVIF_DEC_ERR_R {
        SLVIF_DEC_ERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear slvif_wr2ro_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_wr2ro_err(&self) -> SLVIF_WR2RO_ERR_R {
        SLVIF_WR2RO_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear slvif_rd2wo_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_rd2wo_err(&self) -> SLVIF_RD2WO_ERR_R {
        SLVIF_RD2WO_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear slvif_wronhold_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_wronhold_err(&self) -> SLVIF_WRONHOLD_ERR_R {
        SLVIF_WRONHOLD_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clear slvif_undefinedreg_dec_err in com_intstatus"]
    #[inline(always)]
    pub fn slvif_undefinedreg_dec_err(&self) -> SLVIF_UNDEFINEDREG_DEC_ERR_R {
        SLVIF_UNDEFINEDREG_DEC_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear slvif_dec_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_dec_err(&mut self) -> SLVIF_DEC_ERR_W {
        SLVIF_DEC_ERR_W { w: self }
    }
    #[doc = "Bit 1 - Clear slvif_wr2ro_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_wr2ro_err(&mut self) -> SLVIF_WR2RO_ERR_W {
        SLVIF_WR2RO_ERR_W { w: self }
    }
    #[doc = "Bit 2 - Clear slvif_rd2wo_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_rd2wo_err(&mut self) -> SLVIF_RD2WO_ERR_W {
        SLVIF_RD2WO_ERR_W { w: self }
    }
    #[doc = "Bit 3 - Clear slvif_wronhold_err interrupt in com_intstatus"]
    #[inline(always)]
    pub fn slvif_wronhold_err(&mut self) -> SLVIF_WRONHOLD_ERR_W {
        SLVIF_WRONHOLD_ERR_W { w: self }
    }
    #[doc = "Bit 8 - Clear slvif_undefinedreg_dec_err in com_intstatus"]
    #[inline(always)]
    pub fn slvif_undefinedreg_dec_err(&mut self) -> SLVIF_UNDEFINEDREG_DEC_ERR_W {
        SLVIF_UNDEFINEDREG_DEC_ERR_W { w: self }
    }
}
