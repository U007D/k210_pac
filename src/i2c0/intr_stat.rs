#[doc = "Reader of register intr_stat"]
pub type R = crate::R<u32, super::INTR_STAT>;
#[doc = "Reader of field `rx_under`"]
pub type RX_UNDER_R = crate::R<bool, bool>;
#[doc = "Reader of field `rx_over`"]
pub type RX_OVER_R = crate::R<bool, bool>;
#[doc = "Reader of field `rx_full`"]
pub type RX_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `tx_over`"]
pub type TX_OVER_R = crate::R<bool, bool>;
#[doc = "Reader of field `tx_empty`"]
pub type TX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `rd_req`"]
pub type RD_REQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `tx_abrt`"]
pub type TX_ABRT_R = crate::R<bool, bool>;
#[doc = "Reader of field `rx_done`"]
pub type RX_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `activity`"]
pub type ACTIVITY_R = crate::R<bool, bool>;
#[doc = "Reader of field `stop_det`"]
pub type STOP_DET_R = crate::R<bool, bool>;
#[doc = "Reader of field `start_det`"]
pub type START_DET_R = crate::R<bool, bool>;
#[doc = "Reader of field `gen_call`"]
pub type GEN_CALL_R = crate::R<bool, bool>;
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
