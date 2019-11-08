#[doc = "Reader of register intr_mask"]
pub type R = crate::R<u32, super::INTR_MASK>;
#[doc = "Writer for register intr_mask"]
pub type W = crate::W<u32, super::INTR_MASK>;
#[doc = "Register intr_mask `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rx_under`"]
pub type RX_UNDER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_under`"]
pub struct RX_UNDER_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_UNDER_W<'a> {
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
#[doc = "Reader of field `rx_over`"]
pub type RX_OVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_over`"]
pub struct RX_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_OVER_W<'a> {
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
#[doc = "Reader of field `rx_full`"]
pub type RX_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_full`"]
pub struct RX_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FULL_W<'a> {
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
#[doc = "Reader of field `tx_over`"]
pub type TX_OVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_over`"]
pub struct TX_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_OVER_W<'a> {
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
#[doc = "Reader of field `tx_empty`"]
pub type TX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_empty`"]
pub struct TX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EMPTY_W<'a> {
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
#[doc = "Reader of field `rd_req`"]
pub type RD_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rd_req`"]
pub struct RD_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_REQ_W<'a> {
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
#[doc = "Reader of field `tx_abrt`"]
pub type TX_ABRT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_abrt`"]
pub struct TX_ABRT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ABRT_W<'a> {
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
#[doc = "Reader of field `rx_done`"]
pub type RX_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rx_done`"]
pub struct RX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DONE_W<'a> {
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
#[doc = "Reader of field `activity`"]
pub type ACTIVITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `activity`"]
pub struct ACTIVITY_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTIVITY_W<'a> {
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
#[doc = "Reader of field `stop_det`"]
pub type STOP_DET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `stop_det`"]
pub struct STOP_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_DET_W<'a> {
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
#[doc = "Reader of field `start_det`"]
pub type START_DET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `start_det`"]
pub struct START_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> START_DET_W<'a> {
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
#[doc = "Reader of field `gen_call`"]
pub type GEN_CALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gen_call`"]
pub struct GEN_CALL_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_CALL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RX_UNDER"]
    #[inline(always)]
    pub fn rx_under(&self) -> RX_UNDER_R {
        RX_UNDER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX_OVER"]
    #[inline(always)]
    pub fn rx_over(&self) -> RX_OVER_R {
        RX_OVER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX_FULL"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX_OVER"]
    #[inline(always)]
    pub fn tx_over(&self) -> TX_OVER_R {
        TX_OVER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX_EMPTY"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RD_REQ"]
    #[inline(always)]
    pub fn rd_req(&self) -> RD_REQ_R {
        RD_REQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX_ABRT"]
    #[inline(always)]
    pub fn tx_abrt(&self) -> TX_ABRT_R {
        TX_ABRT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RX_DONE"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ACTIVITY"]
    #[inline(always)]
    pub fn activity(&self) -> ACTIVITY_R {
        ACTIVITY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - STOP_DET"]
    #[inline(always)]
    pub fn stop_det(&self) -> STOP_DET_R {
        STOP_DET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - START_DET"]
    #[inline(always)]
    pub fn start_det(&self) -> START_DET_R {
        START_DET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GEN_CALL"]
    #[inline(always)]
    pub fn gen_call(&self) -> GEN_CALL_R {
        GEN_CALL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX_UNDER"]
    #[inline(always)]
    pub fn rx_under(&mut self) -> RX_UNDER_W {
        RX_UNDER_W { w: self }
    }
    #[doc = "Bit 1 - RX_OVER"]
    #[inline(always)]
    pub fn rx_over(&mut self) -> RX_OVER_W {
        RX_OVER_W { w: self }
    }
    #[doc = "Bit 2 - RX_FULL"]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RX_FULL_W {
        RX_FULL_W { w: self }
    }
    #[doc = "Bit 3 - TX_OVER"]
    #[inline(always)]
    pub fn tx_over(&mut self) -> TX_OVER_W {
        TX_OVER_W { w: self }
    }
    #[doc = "Bit 4 - TX_EMPTY"]
    #[inline(always)]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W {
        TX_EMPTY_W { w: self }
    }
    #[doc = "Bit 5 - RD_REQ"]
    #[inline(always)]
    pub fn rd_req(&mut self) -> RD_REQ_W {
        RD_REQ_W { w: self }
    }
    #[doc = "Bit 6 - TX_ABRT"]
    #[inline(always)]
    pub fn tx_abrt(&mut self) -> TX_ABRT_W {
        TX_ABRT_W { w: self }
    }
    #[doc = "Bit 7 - RX_DONE"]
    #[inline(always)]
    pub fn rx_done(&mut self) -> RX_DONE_W {
        RX_DONE_W { w: self }
    }
    #[doc = "Bit 8 - ACTIVITY"]
    #[inline(always)]
    pub fn activity(&mut self) -> ACTIVITY_W {
        ACTIVITY_W { w: self }
    }
    #[doc = "Bit 9 - STOP_DET"]
    #[inline(always)]
    pub fn stop_det(&mut self) -> STOP_DET_W {
        STOP_DET_W { w: self }
    }
    #[doc = "Bit 10 - START_DET"]
    #[inline(always)]
    pub fn start_det(&mut self) -> START_DET_W {
        START_DET_W { w: self }
    }
    #[doc = "Bit 11 - GEN_CALL"]
    #[inline(always)]
    pub fn gen_call(&mut self) -> GEN_CALL_W {
        GEN_CALL_W { w: self }
    }
}
