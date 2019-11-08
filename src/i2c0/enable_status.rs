#[doc = "Reader of register enable_status"]
pub type R = crate::R<u32, super::ENABLE_STATUS>;
#[doc = "Reader of field `ic_enable`"]
pub type IC_ENABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `slv_dis_busy`"]
pub type SLV_DIS_BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `slv_rx_data_lost`"]
pub type SLV_RX_DATA_LOST_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - IC_ENABLE"]
    #[inline(always)]
    pub fn ic_enable(&self) -> IC_ENABLE_R {
        IC_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SLV_DIS_BUSY"]
    #[inline(always)]
    pub fn slv_dis_busy(&self) -> SLV_DIS_BUSY_R {
        SLV_DIS_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SLV_RX_DATA_LOST"]
    #[inline(always)]
    pub fn slv_rx_data_lost(&self) -> SLV_RX_DATA_LOST_R {
        SLV_RX_DATA_LOST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
