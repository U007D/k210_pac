#[doc = "Reader of register tx_abrt_source"]
pub type R = crate::R<u32, super::TX_ABRT_SOURCE>;
#[doc = "Writer for register tx_abrt_source"]
pub type W = crate::W<u32, super::TX_ABRT_SOURCE>;
#[doc = "Register tx_abrt_source `reset()`'s with value 0"]
impl crate::ResetValue for super::TX_ABRT_SOURCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `addr7_noack`"]
pub type ADDR7_NOACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `addr7_noack`"]
pub struct ADDR7_NOACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR7_NOACK_W<'a> {
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
#[doc = "Reader of field `addr1_10_noack`"]
pub type ADDR1_10_NOACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `addr1_10_noack`"]
pub struct ADDR1_10_NOACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR1_10_NOACK_W<'a> {
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
#[doc = "Reader of field `addr2_10_noack`"]
pub type ADDR2_10_NOACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `addr2_10_noack`"]
pub struct ADDR2_10_NOACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR2_10_NOACK_W<'a> {
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
#[doc = "Reader of field `txdata_noack`"]
pub type TXDATA_NOACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `txdata_noack`"]
pub struct TXDATA_NOACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA_NOACK_W<'a> {
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
#[doc = "Reader of field `gcall_noack`"]
pub type GCALL_NOACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gcall_noack`"]
pub struct GCALL_NOACK_W<'a> {
    w: &'a mut W,
}
impl<'a> GCALL_NOACK_W<'a> {
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
#[doc = "Reader of field `gcall_read`"]
pub type GCALL_READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gcall_read`"]
pub struct GCALL_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> GCALL_READ_W<'a> {
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
#[doc = "Reader of field `hs_ackdet`"]
pub type HS_ACKDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hs_ackdet`"]
pub struct HS_ACKDET_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_ACKDET_W<'a> {
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
#[doc = "Reader of field `sbyte_ackdet`"]
pub type SBYTE_ACKDET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sbyte_ackdet`"]
pub struct SBYTE_ACKDET_W<'a> {
    w: &'a mut W,
}
impl<'a> SBYTE_ACKDET_W<'a> {
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
#[doc = "Reader of field `hs_norstrt`"]
pub type HS_NORSTRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `hs_norstrt`"]
pub struct HS_NORSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_NORSTRT_W<'a> {
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
#[doc = "Reader of field `sbyte_norstrt`"]
pub type SBYTE_NORSTRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sbyte_norstrt`"]
pub struct SBYTE_NORSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> SBYTE_NORSTRT_W<'a> {
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
#[doc = "Reader of field `rd_10_norstrt`"]
pub type RD_10_NORSTRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_10_norstrt`"]
pub struct RD_10_NORSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_10_NORSTRT_W<'a> {
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
#[doc = "Reader of field `master_dis`"]
pub type MASTER_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `master_dis`"]
pub struct MASTER_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_DIS_W<'a> {
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
#[doc = "Reader of field `mst_arblost`"]
pub type MST_ARBLOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `mst_arblost`"]
pub struct MST_ARBLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> MST_ARBLOST_W<'a> {
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
#[doc = "Reader of field `slvflush_txfifo`"]
pub type SLVFLUSH_TXFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `slvflush_txfifo`"]
pub struct SLVFLUSH_TXFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVFLUSH_TXFIFO_W<'a> {
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
#[doc = "Reader of field `slv_arblost`"]
pub type SLV_ARBLOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `slv_arblost`"]
pub struct SLV_ARBLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_ARBLOST_W<'a> {
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
#[doc = "Reader of field `slvrd_intx`"]
pub type SLVRD_INTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `slvrd_intx`"]
pub struct SLVRD_INTX_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVRD_INTX_W<'a> {
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
#[doc = "Reader of field `user_abrt`"]
pub type USER_ABRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `user_abrt`"]
pub struct USER_ABRT_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_ABRT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 7B_ADDR_NOACK"]
    #[inline(always)]
    pub fn addr7_noack(&self) -> ADDR7_NOACK_R {
        ADDR7_NOACK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 10B_ADDR1_NOACK"]
    #[inline(always)]
    pub fn addr1_10_noack(&self) -> ADDR1_10_NOACK_R {
        ADDR1_10_NOACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 10B_ADDR2_NOACK"]
    #[inline(always)]
    pub fn addr2_10_noack(&self) -> ADDR2_10_NOACK_R {
        ADDR2_10_NOACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TXDATA_NOACK"]
    #[inline(always)]
    pub fn txdata_noack(&self) -> TXDATA_NOACK_R {
        TXDATA_NOACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GCALL_NOACK"]
    #[inline(always)]
    pub fn gcall_noack(&self) -> GCALL_NOACK_R {
        GCALL_NOACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GCALL_READ"]
    #[inline(always)]
    pub fn gcall_read(&self) -> GCALL_READ_R {
        GCALL_READ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HS_ACKDET"]
    #[inline(always)]
    pub fn hs_ackdet(&self) -> HS_ACKDET_R {
        HS_ACKDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SBYTE_ACKDET"]
    #[inline(always)]
    pub fn sbyte_ackdet(&self) -> SBYTE_ACKDET_R {
        SBYTE_ACKDET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HS_NORSTRT"]
    #[inline(always)]
    pub fn hs_norstrt(&self) -> HS_NORSTRT_R {
        HS_NORSTRT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SBYTE_NORSTRT"]
    #[inline(always)]
    pub fn sbyte_norstrt(&self) -> SBYTE_NORSTRT_R {
        SBYTE_NORSTRT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10B_RD_NORSTRT"]
    #[inline(always)]
    pub fn rd_10_norstrt(&self) -> RD_10_NORSTRT_R {
        RD_10_NORSTRT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - MASTER_DIS"]
    #[inline(always)]
    pub fn master_dis(&self) -> MASTER_DIS_R {
        MASTER_DIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MST_ARBLOST"]
    #[inline(always)]
    pub fn mst_arblost(&self) -> MST_ARBLOST_R {
        MST_ARBLOST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SLVFLUSH_TXFIFO"]
    #[inline(always)]
    pub fn slvflush_txfifo(&self) -> SLVFLUSH_TXFIFO_R {
        SLVFLUSH_TXFIFO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SLV_ARBLOST"]
    #[inline(always)]
    pub fn slv_arblost(&self) -> SLV_ARBLOST_R {
        SLV_ARBLOST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SLVRD_INTX"]
    #[inline(always)]
    pub fn slvrd_intx(&self) -> SLVRD_INTX_R {
        SLVRD_INTX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USER_ABRT"]
    #[inline(always)]
    pub fn user_abrt(&self) -> USER_ABRT_R {
        USER_ABRT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 7B_ADDR_NOACK"]
    #[inline(always)]
    pub fn addr7_noack(&mut self) -> ADDR7_NOACK_W {
        ADDR7_NOACK_W { w: self }
    }
    #[doc = "Bit 1 - 10B_ADDR1_NOACK"]
    #[inline(always)]
    pub fn addr1_10_noack(&mut self) -> ADDR1_10_NOACK_W {
        ADDR1_10_NOACK_W { w: self }
    }
    #[doc = "Bit 2 - 10B_ADDR2_NOACK"]
    #[inline(always)]
    pub fn addr2_10_noack(&mut self) -> ADDR2_10_NOACK_W {
        ADDR2_10_NOACK_W { w: self }
    }
    #[doc = "Bit 3 - TXDATA_NOACK"]
    #[inline(always)]
    pub fn txdata_noack(&mut self) -> TXDATA_NOACK_W {
        TXDATA_NOACK_W { w: self }
    }
    #[doc = "Bit 4 - GCALL_NOACK"]
    #[inline(always)]
    pub fn gcall_noack(&mut self) -> GCALL_NOACK_W {
        GCALL_NOACK_W { w: self }
    }
    #[doc = "Bit 5 - GCALL_READ"]
    #[inline(always)]
    pub fn gcall_read(&mut self) -> GCALL_READ_W {
        GCALL_READ_W { w: self }
    }
    #[doc = "Bit 6 - HS_ACKDET"]
    #[inline(always)]
    pub fn hs_ackdet(&mut self) -> HS_ACKDET_W {
        HS_ACKDET_W { w: self }
    }
    #[doc = "Bit 7 - SBYTE_ACKDET"]
    #[inline(always)]
    pub fn sbyte_ackdet(&mut self) -> SBYTE_ACKDET_W {
        SBYTE_ACKDET_W { w: self }
    }
    #[doc = "Bit 8 - HS_NORSTRT"]
    #[inline(always)]
    pub fn hs_norstrt(&mut self) -> HS_NORSTRT_W {
        HS_NORSTRT_W { w: self }
    }
    #[doc = "Bit 9 - SBYTE_NORSTRT"]
    #[inline(always)]
    pub fn sbyte_norstrt(&mut self) -> SBYTE_NORSTRT_W {
        SBYTE_NORSTRT_W { w: self }
    }
    #[doc = "Bit 10 - 10B_RD_NORSTRT"]
    #[inline(always)]
    pub fn rd_10_norstrt(&mut self) -> RD_10_NORSTRT_W {
        RD_10_NORSTRT_W { w: self }
    }
    #[doc = "Bit 11 - MASTER_DIS"]
    #[inline(always)]
    pub fn master_dis(&mut self) -> MASTER_DIS_W {
        MASTER_DIS_W { w: self }
    }
    #[doc = "Bit 12 - MST_ARBLOST"]
    #[inline(always)]
    pub fn mst_arblost(&mut self) -> MST_ARBLOST_W {
        MST_ARBLOST_W { w: self }
    }
    #[doc = "Bit 13 - SLVFLUSH_TXFIFO"]
    #[inline(always)]
    pub fn slvflush_txfifo(&mut self) -> SLVFLUSH_TXFIFO_W {
        SLVFLUSH_TXFIFO_W { w: self }
    }
    #[doc = "Bit 14 - SLV_ARBLOST"]
    #[inline(always)]
    pub fn slv_arblost(&mut self) -> SLV_ARBLOST_W {
        SLV_ARBLOST_W { w: self }
    }
    #[doc = "Bit 15 - SLVRD_INTX"]
    #[inline(always)]
    pub fn slvrd_intx(&mut self) -> SLVRD_INTX_W {
        SLVRD_INTX_W { w: self }
    }
    #[doc = "Bit 16 - USER_ABRT"]
    #[inline(always)]
    pub fn user_abrt(&mut self) -> USER_ABRT_W {
        USER_ABRT_W { w: self }
    }
}
