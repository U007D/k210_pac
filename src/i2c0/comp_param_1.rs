#[doc = "Reader of register comp_param_1"]
pub type R = crate::R<u32, super::COMP_PARAM_1>;
#[doc = "Reader of field `apb_data_width`"]
pub type APB_DATA_WIDTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `max_speed_mode`"]
pub type MAX_SPEED_MODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `hc_count_values`"]
pub type HC_COUNT_VALUES_R = crate::R<bool, bool>;
#[doc = "Reader of field `intr_io`"]
pub type INTR_IO_R = crate::R<bool, bool>;
#[doc = "Reader of field `has_dma`"]
pub type HAS_DMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `encoded_params`"]
pub type ENCODED_PARAMS_R = crate::R<bool, bool>;
#[doc = "Reader of field `rx_buffer_depth`"]
pub type RX_BUFFER_DEPTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `tx_buffer_depth`"]
pub type TX_BUFFER_DEPTH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - APB_DATA_WIDTH"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> APB_DATA_WIDTH_R {
        APB_DATA_WIDTH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - MAX_SPEED_MODE"]
    #[inline(always)]
    pub fn max_speed_mode(&self) -> MAX_SPEED_MODE_R {
        MAX_SPEED_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - HC_COUNT_VALUES"]
    #[inline(always)]
    pub fn hc_count_values(&self) -> HC_COUNT_VALUES_R {
        HC_COUNT_VALUES_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INTR_IO"]
    #[inline(always)]
    pub fn intr_io(&self) -> INTR_IO_R {
        INTR_IO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HAS_DMA"]
    #[inline(always)]
    pub fn has_dma(&self) -> HAS_DMA_R {
        HAS_DMA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ENCODED_PARAMS"]
    #[inline(always)]
    pub fn encoded_params(&self) -> ENCODED_PARAMS_R {
        ENCODED_PARAMS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - RX_BUFFER_DEPTH"]
    #[inline(always)]
    pub fn rx_buffer_depth(&self) -> RX_BUFFER_DEPTH_R {
        RX_BUFFER_DEPTH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TX_BUFFER_DEPTH"]
    #[inline(always)]
    pub fn tx_buffer_depth(&self) -> TX_BUFFER_DEPTH_R {
        TX_BUFFER_DEPTH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
