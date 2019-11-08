#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub con: CON,
    #[doc = "0x04 - Target Address Register"]
    pub tar: TAR,
    #[doc = "0x08 - Slave Address Register"]
    pub sar: SAR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Data Buffer and Command Register"]
    pub data_cmd: DATA_CMD,
    #[doc = "0x14 - Standard Speed Clock SCL High Count Register"]
    pub ss_scl_hcnt: SS_SCL_HCNT,
    #[doc = "0x18 - Standard Speed Clock SCL Low Count Register"]
    pub ss_scl_lcnt: SS_SCL_LCNT,
    _reserved6: [u8; 16usize],
    #[doc = "0x2c - Interrupt Status Register"]
    pub intr_stat: INTR_STAT,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub intr_mask: INTR_MASK,
    #[doc = "0x34 - Raw Interrupt Status Register"]
    pub raw_intr_stat: RAW_INTR_STAT,
    #[doc = "0x38 - Receive FIFO Threshold Register"]
    pub rx_tl: RX_TL,
    #[doc = "0x3c - Transmit FIFO Threshold Register"]
    pub tx_tl: TX_TL,
    #[doc = "0x40 - Clear Combined and Individual Interrupt Register"]
    pub clr_intr: CLR_INTR,
    #[doc = "0x44 - Clear RX_UNDER Interrupt Register"]
    pub clr_rx_under: CLR_RX_UNDER,
    #[doc = "0x48 - Clear RX_OVER Interrupt Register"]
    pub clr_rx_over: CLR_RX_OVER,
    #[doc = "0x4c - Clear TX_OVER Interrupt Register"]
    pub clr_tx_over: CLR_TX_OVER,
    #[doc = "0x50 - Clear RD_REQ Interrupt Register"]
    pub clr_rd_req: CLR_RD_REQ,
    #[doc = "0x54 - Clear TX_ABRT Interrupt Register"]
    pub clr_tx_abrt: CLR_TX_ABRT,
    #[doc = "0x58 - Clear RX_DONE Interrupt Register"]
    pub clr_rx_done: CLR_RX_DONE,
    #[doc = "0x5c - Clear ACTIVITY Interrupt Register"]
    pub clr_activity: CLR_ACTIVITY,
    #[doc = "0x60 - Clear STOP_DET Interrupt Register"]
    pub clr_stop_det: CLR_STOP_DET,
    #[doc = "0x64 - Clear START_DET Interrupt Register"]
    pub clr_start_det: CLR_START_DET,
    #[doc = "0x68 - I2C Clear GEN_CALL Interrupt Register"]
    pub clr_gen_call: CLR_GEN_CALL,
    #[doc = "0x6c - Enable Register"]
    pub enable: ENABLE,
    #[doc = "0x70 - Status Register"]
    pub status: STATUS,
    #[doc = "0x74 - Transmit FIFO Level Register"]
    pub txflr: TXFLR,
    #[doc = "0x78 - Receive FIFO Level Register"]
    pub rxflr: RXFLR,
    #[doc = "0x7c - SDA Hold Time Length Register"]
    pub sda_hold: SDA_HOLD,
    #[doc = "0x80 - Transmit Abort Source Register"]
    pub tx_abrt_source: TX_ABRT_SOURCE,
    _reserved28: [u8; 4usize],
    #[doc = "0x88 - I2C DMA Control Register"]
    pub dma_cr: DMA_CR,
    #[doc = "0x8c - DMA Transmit Data Level Register"]
    pub dma_tdlr: DMA_TDLR,
    #[doc = "0x90 - DMA Receive Data Level Register"]
    pub dma_rdlr: DMA_RDLR,
    #[doc = "0x94 - SDA Setup Register"]
    pub sda_setup: SDA_SETUP,
    #[doc = "0x98 - ACK General Call Register"]
    pub general_call: GENERAL_CALL,
    #[doc = "0x9c - Enable Status Register"]
    pub enable_status: ENABLE_STATUS,
    #[doc = "0xa0 - SS, FS or FM+ spike suppression limit"]
    pub fs_spklen: FS_SPKLEN,
    _reserved35: [u8; 80usize],
    #[doc = "0xf4 - Component Parameter Register 1"]
    pub comp_param_1: COMP_PARAM_1,
    #[doc = "0xf8 - Component Version Register"]
    pub comp_version: COMP_VERSION,
    #[doc = "0xfc - Component Type Register"]
    pub comp_type: COMP_TYPE,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [con](con) module"]
pub type CON = crate::Reg<u32, _CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON;
#[doc = "`read()` method returns [con::R](con::R) reader structure"]
impl crate::Readable for CON {}
#[doc = "`write(|w| ..)` method takes [con::W](con::W) writer structure"]
impl crate::Writable for CON {}
#[doc = "Control Register"]
pub mod con;
#[doc = "Target Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tar](tar) module"]
pub type TAR = crate::Reg<u32, _TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAR;
#[doc = "`read()` method returns [tar::R](tar::R) reader structure"]
impl crate::Readable for TAR {}
#[doc = "`write(|w| ..)` method takes [tar::W](tar::W) writer structure"]
impl crate::Writable for TAR {}
#[doc = "Target Address Register"]
pub mod tar;
#[doc = "Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar](sar) module"]
pub type SAR = crate::Reg<u32, _SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR;
#[doc = "`read()` method returns [sar::R](sar::R) reader structure"]
impl crate::Readable for SAR {}
#[doc = "`write(|w| ..)` method takes [sar::W](sar::W) writer structure"]
impl crate::Writable for SAR {}
#[doc = "Slave Address Register"]
pub mod sar;
#[doc = "Data Buffer and Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_cmd](data_cmd) module"]
pub type DATA_CMD = crate::Reg<u32, _DATA_CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_CMD;
#[doc = "`read()` method returns [data_cmd::R](data_cmd::R) reader structure"]
impl crate::Readable for DATA_CMD {}
#[doc = "`write(|w| ..)` method takes [data_cmd::W](data_cmd::W) writer structure"]
impl crate::Writable for DATA_CMD {}
#[doc = "Data Buffer and Command Register"]
pub mod data_cmd;
#[doc = "Standard Speed Clock SCL High Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ss_scl_hcnt](ss_scl_hcnt) module"]
pub type SS_SCL_HCNT = crate::Reg<u32, _SS_SCL_HCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SS_SCL_HCNT;
#[doc = "`read()` method returns [ss_scl_hcnt::R](ss_scl_hcnt::R) reader structure"]
impl crate::Readable for SS_SCL_HCNT {}
#[doc = "`write(|w| ..)` method takes [ss_scl_hcnt::W](ss_scl_hcnt::W) writer structure"]
impl crate::Writable for SS_SCL_HCNT {}
#[doc = "Standard Speed Clock SCL High Count Register"]
pub mod ss_scl_hcnt;
#[doc = "Standard Speed Clock SCL Low Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ss_scl_lcnt](ss_scl_lcnt) module"]
pub type SS_SCL_LCNT = crate::Reg<u32, _SS_SCL_LCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SS_SCL_LCNT;
#[doc = "`read()` method returns [ss_scl_lcnt::R](ss_scl_lcnt::R) reader structure"]
impl crate::Readable for SS_SCL_LCNT {}
#[doc = "`write(|w| ..)` method takes [ss_scl_lcnt::W](ss_scl_lcnt::W) writer structure"]
impl crate::Writable for SS_SCL_LCNT {}
#[doc = "Standard Speed Clock SCL Low Count Register"]
pub mod ss_scl_lcnt;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_stat](intr_stat) module"]
pub type INTR_STAT = crate::Reg<u32, _INTR_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_STAT;
#[doc = "`read()` method returns [intr_stat::R](intr_stat::R) reader structure"]
impl crate::Readable for INTR_STAT {}
#[doc = "Interrupt Status Register"]
pub mod intr_stat;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_mask](intr_mask) module"]
pub type INTR_MASK = crate::Reg<u32, _INTR_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_MASK;
#[doc = "`read()` method returns [intr_mask::R](intr_mask::R) reader structure"]
impl crate::Readable for INTR_MASK {}
#[doc = "`write(|w| ..)` method takes [intr_mask::W](intr_mask::W) writer structure"]
impl crate::Writable for INTR_MASK {}
#[doc = "Interrupt Mask Register"]
pub mod intr_mask;
#[doc = "Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [raw_intr_stat](raw_intr_stat) module"]
pub type RAW_INTR_STAT = crate::Reg<u32, _RAW_INTR_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAW_INTR_STAT;
#[doc = "`read()` method returns [raw_intr_stat::R](raw_intr_stat::R) reader structure"]
impl crate::Readable for RAW_INTR_STAT {}
#[doc = "`write(|w| ..)` method takes [raw_intr_stat::W](raw_intr_stat::W) writer structure"]
impl crate::Writable for RAW_INTR_STAT {}
#[doc = "Raw Interrupt Status Register"]
pub mod raw_intr_stat;
#[doc = "Receive FIFO Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_tl](rx_tl) module"]
pub type RX_TL = crate::Reg<u32, _RX_TL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_TL;
#[doc = "`read()` method returns [rx_tl::R](rx_tl::R) reader structure"]
impl crate::Readable for RX_TL {}
#[doc = "`write(|w| ..)` method takes [rx_tl::W](rx_tl::W) writer structure"]
impl crate::Writable for RX_TL {}
#[doc = "Receive FIFO Threshold Register"]
pub mod rx_tl;
#[doc = "Transmit FIFO Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_tl](tx_tl) module"]
pub type TX_TL = crate::Reg<u32, _TX_TL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_TL;
#[doc = "`read()` method returns [tx_tl::R](tx_tl::R) reader structure"]
impl crate::Readable for TX_TL {}
#[doc = "`write(|w| ..)` method takes [tx_tl::W](tx_tl::W) writer structure"]
impl crate::Writable for TX_TL {}
#[doc = "Transmit FIFO Threshold Register"]
pub mod tx_tl;
#[doc = "Clear Combined and Individual Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_intr](clr_intr) module"]
pub type CLR_INTR = crate::Reg<u32, _CLR_INTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_INTR;
#[doc = "`read()` method returns [clr_intr::R](clr_intr::R) reader structure"]
impl crate::Readable for CLR_INTR {}
#[doc = "Clear Combined and Individual Interrupt Register"]
pub mod clr_intr;
#[doc = "Clear RX_UNDER Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_rx_under](clr_rx_under) module"]
pub type CLR_RX_UNDER = crate::Reg<u32, _CLR_RX_UNDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_RX_UNDER;
#[doc = "`read()` method returns [clr_rx_under::R](clr_rx_under::R) reader structure"]
impl crate::Readable for CLR_RX_UNDER {}
#[doc = "Clear RX_UNDER Interrupt Register"]
pub mod clr_rx_under;
#[doc = "Clear RX_OVER Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_rx_over](clr_rx_over) module"]
pub type CLR_RX_OVER = crate::Reg<u32, _CLR_RX_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_RX_OVER;
#[doc = "`read()` method returns [clr_rx_over::R](clr_rx_over::R) reader structure"]
impl crate::Readable for CLR_RX_OVER {}
#[doc = "Clear RX_OVER Interrupt Register"]
pub mod clr_rx_over;
#[doc = "Clear TX_OVER Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_tx_over](clr_tx_over) module"]
pub type CLR_TX_OVER = crate::Reg<u32, _CLR_TX_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_TX_OVER;
#[doc = "`read()` method returns [clr_tx_over::R](clr_tx_over::R) reader structure"]
impl crate::Readable for CLR_TX_OVER {}
#[doc = "Clear TX_OVER Interrupt Register"]
pub mod clr_tx_over;
#[doc = "Clear RD_REQ Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_rd_req](clr_rd_req) module"]
pub type CLR_RD_REQ = crate::Reg<u32, _CLR_RD_REQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_RD_REQ;
#[doc = "`read()` method returns [clr_rd_req::R](clr_rd_req::R) reader structure"]
impl crate::Readable for CLR_RD_REQ {}
#[doc = "Clear RD_REQ Interrupt Register"]
pub mod clr_rd_req;
#[doc = "Clear TX_ABRT Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_tx_abrt](clr_tx_abrt) module"]
pub type CLR_TX_ABRT = crate::Reg<u32, _CLR_TX_ABRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_TX_ABRT;
#[doc = "`read()` method returns [clr_tx_abrt::R](clr_tx_abrt::R) reader structure"]
impl crate::Readable for CLR_TX_ABRT {}
#[doc = "Clear TX_ABRT Interrupt Register"]
pub mod clr_tx_abrt;
#[doc = "Clear RX_DONE Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_rx_done](clr_rx_done) module"]
pub type CLR_RX_DONE = crate::Reg<u32, _CLR_RX_DONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_RX_DONE;
#[doc = "`read()` method returns [clr_rx_done::R](clr_rx_done::R) reader structure"]
impl crate::Readable for CLR_RX_DONE {}
#[doc = "Clear RX_DONE Interrupt Register"]
pub mod clr_rx_done;
#[doc = "Clear ACTIVITY Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_activity](clr_activity) module"]
pub type CLR_ACTIVITY = crate::Reg<u32, _CLR_ACTIVITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_ACTIVITY;
#[doc = "`read()` method returns [clr_activity::R](clr_activity::R) reader structure"]
impl crate::Readable for CLR_ACTIVITY {}
#[doc = "Clear ACTIVITY Interrupt Register"]
pub mod clr_activity;
#[doc = "Clear STOP_DET Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_stop_det](clr_stop_det) module"]
pub type CLR_STOP_DET = crate::Reg<u32, _CLR_STOP_DET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_STOP_DET;
#[doc = "`read()` method returns [clr_stop_det::R](clr_stop_det::R) reader structure"]
impl crate::Readable for CLR_STOP_DET {}
#[doc = "Clear STOP_DET Interrupt Register"]
pub mod clr_stop_det;
#[doc = "Clear START_DET Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_start_det](clr_start_det) module"]
pub type CLR_START_DET = crate::Reg<u32, _CLR_START_DET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_START_DET;
#[doc = "`read()` method returns [clr_start_det::R](clr_start_det::R) reader structure"]
impl crate::Readable for CLR_START_DET {}
#[doc = "Clear START_DET Interrupt Register"]
pub mod clr_start_det;
#[doc = "I2C Clear GEN_CALL Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clr_gen_call](clr_gen_call) module"]
pub type CLR_GEN_CALL = crate::Reg<u32, _CLR_GEN_CALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR_GEN_CALL;
#[doc = "`read()` method returns [clr_gen_call::R](clr_gen_call::R) reader structure"]
impl crate::Readable for CLR_GEN_CALL {}
#[doc = "I2C Clear GEN_CALL Interrupt Register"]
pub mod clr_gen_call;
#[doc = "Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable Register"]
pub mod enable;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Transmit FIFO Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txflr](txflr) module"]
pub type TXFLR = crate::Reg<u32, _TXFLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFLR;
#[doc = "`read()` method returns [txflr::R](txflr::R) reader structure"]
impl crate::Readable for TXFLR {}
#[doc = "`write(|w| ..)` method takes [txflr::W](txflr::W) writer structure"]
impl crate::Writable for TXFLR {}
#[doc = "Transmit FIFO Level Register"]
pub mod txflr;
#[doc = "Receive FIFO Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxflr](rxflr) module"]
pub type RXFLR = crate::Reg<u32, _RXFLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFLR;
#[doc = "`read()` method returns [rxflr::R](rxflr::R) reader structure"]
impl crate::Readable for RXFLR {}
#[doc = "`write(|w| ..)` method takes [rxflr::W](rxflr::W) writer structure"]
impl crate::Writable for RXFLR {}
#[doc = "Receive FIFO Level Register"]
pub mod rxflr;
#[doc = "SDA Hold Time Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sda_hold](sda_hold) module"]
pub type SDA_HOLD = crate::Reg<u32, _SDA_HOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA_HOLD;
#[doc = "`read()` method returns [sda_hold::R](sda_hold::R) reader structure"]
impl crate::Readable for SDA_HOLD {}
#[doc = "`write(|w| ..)` method takes [sda_hold::W](sda_hold::W) writer structure"]
impl crate::Writable for SDA_HOLD {}
#[doc = "SDA Hold Time Length Register"]
pub mod sda_hold;
#[doc = "Transmit Abort Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tx_abrt_source](tx_abrt_source) module"]
pub type TX_ABRT_SOURCE = crate::Reg<u32, _TX_ABRT_SOURCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TX_ABRT_SOURCE;
#[doc = "`read()` method returns [tx_abrt_source::R](tx_abrt_source::R) reader structure"]
impl crate::Readable for TX_ABRT_SOURCE {}
#[doc = "`write(|w| ..)` method takes [tx_abrt_source::W](tx_abrt_source::W) writer structure"]
impl crate::Writable for TX_ABRT_SOURCE {}
#[doc = "Transmit Abort Source Register"]
pub mod tx_abrt_source;
#[doc = "I2C DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_cr](dma_cr) module"]
pub type DMA_CR = crate::Reg<u32, _DMA_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_CR;
#[doc = "`read()` method returns [dma_cr::R](dma_cr::R) reader structure"]
impl crate::Readable for DMA_CR {}
#[doc = "`write(|w| ..)` method takes [dma_cr::W](dma_cr::W) writer structure"]
impl crate::Writable for DMA_CR {}
#[doc = "I2C DMA Control Register"]
pub mod dma_cr;
#[doc = "DMA Transmit Data Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_tdlr](dma_tdlr) module"]
pub type DMA_TDLR = crate::Reg<u32, _DMA_TDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_TDLR;
#[doc = "`read()` method returns [dma_tdlr::R](dma_tdlr::R) reader structure"]
impl crate::Readable for DMA_TDLR {}
#[doc = "`write(|w| ..)` method takes [dma_tdlr::W](dma_tdlr::W) writer structure"]
impl crate::Writable for DMA_TDLR {}
#[doc = "DMA Transmit Data Level Register"]
pub mod dma_tdlr;
#[doc = "DMA Receive Data Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_rdlr](dma_rdlr) module"]
pub type DMA_RDLR = crate::Reg<u32, _DMA_RDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_RDLR;
#[doc = "`read()` method returns [dma_rdlr::R](dma_rdlr::R) reader structure"]
impl crate::Readable for DMA_RDLR {}
#[doc = "`write(|w| ..)` method takes [dma_rdlr::W](dma_rdlr::W) writer structure"]
impl crate::Writable for DMA_RDLR {}
#[doc = "DMA Receive Data Level Register"]
pub mod dma_rdlr;
#[doc = "SDA Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sda_setup](sda_setup) module"]
pub type SDA_SETUP = crate::Reg<u32, _SDA_SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDA_SETUP;
#[doc = "`read()` method returns [sda_setup::R](sda_setup::R) reader structure"]
impl crate::Readable for SDA_SETUP {}
#[doc = "`write(|w| ..)` method takes [sda_setup::W](sda_setup::W) writer structure"]
impl crate::Writable for SDA_SETUP {}
#[doc = "SDA Setup Register"]
pub mod sda_setup;
#[doc = "ACK General Call Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [general_call](general_call) module"]
pub type GENERAL_CALL = crate::Reg<u32, _GENERAL_CALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GENERAL_CALL;
#[doc = "`read()` method returns [general_call::R](general_call::R) reader structure"]
impl crate::Readable for GENERAL_CALL {}
#[doc = "`write(|w| ..)` method takes [general_call::W](general_call::W) writer structure"]
impl crate::Writable for GENERAL_CALL {}
#[doc = "ACK General Call Register"]
pub mod general_call;
#[doc = "Enable Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable_status](enable_status) module"]
pub type ENABLE_STATUS = crate::Reg<u32, _ENABLE_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE_STATUS;
#[doc = "`read()` method returns [enable_status::R](enable_status::R) reader structure"]
impl crate::Readable for ENABLE_STATUS {}
#[doc = "Enable Status Register"]
pub mod enable_status;
#[doc = "SS, FS or FM+ spike suppression limit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fs_spklen](fs_spklen) module"]
pub type FS_SPKLEN = crate::Reg<u32, _FS_SPKLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FS_SPKLEN;
#[doc = "`read()` method returns [fs_spklen::R](fs_spklen::R) reader structure"]
impl crate::Readable for FS_SPKLEN {}
#[doc = "`write(|w| ..)` method takes [fs_spklen::W](fs_spklen::W) writer structure"]
impl crate::Writable for FS_SPKLEN {}
#[doc = "SS, FS or FM+ spike suppression limit"]
pub mod fs_spklen;
#[doc = "Component Parameter Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_param_1](comp_param_1) module"]
pub type COMP_PARAM_1 = crate::Reg<u32, _COMP_PARAM_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_PARAM_1;
#[doc = "`read()` method returns [comp_param_1::R](comp_param_1::R) reader structure"]
impl crate::Readable for COMP_PARAM_1 {}
#[doc = "Component Parameter Register 1"]
pub mod comp_param_1;
#[doc = "Component Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_version](comp_version) module"]
pub type COMP_VERSION = crate::Reg<u32, _COMP_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_VERSION;
#[doc = "`read()` method returns [comp_version::R](comp_version::R) reader structure"]
impl crate::Readable for COMP_VERSION {}
#[doc = "Component Version Register"]
pub mod comp_version;
#[doc = "Component Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_type](comp_type) module"]
pub type COMP_TYPE = crate::Reg<u32, _COMP_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_TYPE;
#[doc = "`read()` method returns [comp_type::R](comp_type::R) reader structure"]
impl crate::Readable for COMP_TYPE {}
#[doc = "Component Type Register"]
pub mod comp_type;
