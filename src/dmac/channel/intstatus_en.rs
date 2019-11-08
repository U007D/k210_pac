#[doc = "Reader of register intstatus_en"]
pub type R = crate::R<u64, super::INTSTATUS_EN>;
#[doc = "Writer for register intstatus_en"]
pub type W = crate::W<u64, super::INTSTATUS_EN>;
#[doc = "Register intstatus_en `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTATUS_EN {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `block_tfr_done`"]
pub type BLOCK_TFR_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `block_tfr_done`"]
pub struct BLOCK_TFR_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_TFR_DONE_W<'a> {
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
#[doc = "Reader of field `tfr_done`"]
pub type TFR_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tfr_done`"]
pub struct TFR_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFR_DONE_W<'a> {
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
#[doc = "Reader of field `src_transcomp`"]
pub type SRC_TRANSCOMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `src_transcomp`"]
pub struct SRC_TRANSCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_TRANSCOMP_W<'a> {
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
#[doc = "Reader of field `dst_transcomp`"]
pub type DST_TRANSCOMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dst_transcomp`"]
pub struct DST_TRANSCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_TRANSCOMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u64) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `src_dec_err`"]
pub type SRC_DEC_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `src_dec_err`"]
pub struct SRC_DEC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_DEC_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u64) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `dst_dec_err`"]
pub type DST_DEC_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dst_dec_err`"]
pub struct DST_DEC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_DEC_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u64) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `src_slv_err`"]
pub type SRC_SLV_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `src_slv_err`"]
pub struct SRC_SLV_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_SLV_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u64) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `dst_slv_err`"]
pub type DST_SLV_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dst_slv_err`"]
pub struct DST_SLV_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_SLV_ERR_W<'a> {
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
#[doc = "Reader of field `lli_rd_dec_err`"]
pub type LLI_RD_DEC_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lli_rd_dec_err`"]
pub struct LLI_RD_DEC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LLI_RD_DEC_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u64) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `lli_wr_dec_err`"]
pub type LLI_WR_DEC_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lli_wr_dec_err`"]
pub struct LLI_WR_DEC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LLI_WR_DEC_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u64) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `lli_rd_slv_err`"]
pub type LLI_RD_SLV_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lli_rd_slv_err`"]
pub struct LLI_RD_SLV_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LLI_RD_SLV_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u64) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `lli_wr_slv_err`"]
pub type LLI_WR_SLV_ERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lli_wr_slv_err`"]
pub struct LLI_WR_SLV_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LLI_WR_SLV_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u64) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Block transfer done"]
    #[inline(always)]
    pub fn block_tfr_done(&self) -> BLOCK_TFR_DONE_R {
        BLOCK_TFR_DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer done"]
    #[inline(always)]
    pub fn tfr_done(&self) -> TFR_DONE_R {
        TFR_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Source transaction complete"]
    #[inline(always)]
    pub fn src_transcomp(&self) -> SRC_TRANSCOMP_R {
        SRC_TRANSCOMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Destination transaction complete"]
    #[inline(always)]
    pub fn dst_transcomp(&self) -> DST_TRANSCOMP_R {
        DST_TRANSCOMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Source Decode Error"]
    #[inline(always)]
    pub fn src_dec_err(&self) -> SRC_DEC_ERR_R {
        SRC_DEC_ERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Destination Decode Error"]
    #[inline(always)]
    pub fn dst_dec_err(&self) -> DST_DEC_ERR_R {
        DST_DEC_ERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Source Slave Error"]
    #[inline(always)]
    pub fn src_slv_err(&self) -> SRC_SLV_ERR_R {
        SRC_SLV_ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Destination Slave Error"]
    #[inline(always)]
    pub fn dst_slv_err(&self) -> DST_SLV_ERR_R {
        DST_SLV_ERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LLI Read Decode Error Status Enable"]
    #[inline(always)]
    pub fn lli_rd_dec_err(&self) -> LLI_RD_DEC_ERR_R {
        LLI_RD_DEC_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LLI WRITE Decode Error"]
    #[inline(always)]
    pub fn lli_wr_dec_err(&self) -> LLI_WR_DEC_ERR_R {
        LLI_WR_DEC_ERR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LLI Read Slave Error"]
    #[inline(always)]
    pub fn lli_rd_slv_err(&self) -> LLI_RD_SLV_ERR_R {
        LLI_RD_SLV_ERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LLI WRITE Slave Error"]
    #[inline(always)]
    pub fn lli_wr_slv_err(&self) -> LLI_WR_SLV_ERR_R {
        LLI_WR_SLV_ERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block transfer done"]
    #[inline(always)]
    pub fn block_tfr_done(&mut self) -> BLOCK_TFR_DONE_W {
        BLOCK_TFR_DONE_W { w: self }
    }
    #[doc = "Bit 1 - Transfer done"]
    #[inline(always)]
    pub fn tfr_done(&mut self) -> TFR_DONE_W {
        TFR_DONE_W { w: self }
    }
    #[doc = "Bit 3 - Source transaction complete"]
    #[inline(always)]
    pub fn src_transcomp(&mut self) -> SRC_TRANSCOMP_W {
        SRC_TRANSCOMP_W { w: self }
    }
    #[doc = "Bit 4 - Destination transaction complete"]
    #[inline(always)]
    pub fn dst_transcomp(&mut self) -> DST_TRANSCOMP_W {
        DST_TRANSCOMP_W { w: self }
    }
    #[doc = "Bit 5 - Source Decode Error"]
    #[inline(always)]
    pub fn src_dec_err(&mut self) -> SRC_DEC_ERR_W {
        SRC_DEC_ERR_W { w: self }
    }
    #[doc = "Bit 6 - Destination Decode Error"]
    #[inline(always)]
    pub fn dst_dec_err(&mut self) -> DST_DEC_ERR_W {
        DST_DEC_ERR_W { w: self }
    }
    #[doc = "Bit 7 - Source Slave Error"]
    #[inline(always)]
    pub fn src_slv_err(&mut self) -> SRC_SLV_ERR_W {
        SRC_SLV_ERR_W { w: self }
    }
    #[doc = "Bit 8 - Destination Slave Error"]
    #[inline(always)]
    pub fn dst_slv_err(&mut self) -> DST_SLV_ERR_W {
        DST_SLV_ERR_W { w: self }
    }
    #[doc = "Bit 9 - LLI Read Decode Error Status Enable"]
    #[inline(always)]
    pub fn lli_rd_dec_err(&mut self) -> LLI_RD_DEC_ERR_W {
        LLI_RD_DEC_ERR_W { w: self }
    }
    #[doc = "Bit 10 - LLI WRITE Decode Error"]
    #[inline(always)]
    pub fn lli_wr_dec_err(&mut self) -> LLI_WR_DEC_ERR_W {
        LLI_WR_DEC_ERR_W { w: self }
    }
    #[doc = "Bit 11 - LLI Read Slave Error"]
    #[inline(always)]
    pub fn lli_rd_slv_err(&mut self) -> LLI_RD_SLV_ERR_W {
        LLI_RD_SLV_ERR_W { w: self }
    }
    #[doc = "Bit 12 - LLI WRITE Slave Error"]
    #[inline(always)]
    pub fn lli_wr_slv_err(&mut self) -> LLI_WR_SLV_ERR_W {
        LLI_WR_SLV_ERR_W { w: self }
    }
}
