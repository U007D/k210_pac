#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 0"]
    pub ctrlr0: CTRLR0,
    #[doc = "0x04 - Control Register 1"]
    pub ctrlr1: CTRLR1,
    #[doc = "0x08 - Enable Register"]
    pub ssienr: SSIENR,
    #[doc = "0x0c - Microwire Control Register"]
    pub mwcr: MWCR,
    #[doc = "0x10 - Slave Enable Register"]
    pub ser: SER,
    #[doc = "0x14 - Baud Rate Select"]
    pub baudr: BAUDR,
    #[doc = "0x18 - Transmit FIFO Threshold Level"]
    pub txftlr: TXFTLR,
    #[doc = "0x1c - Receive FIFO Threshold Level"]
    pub rxftlr: RXFTLR,
    #[doc = "0x20 - Transmit FIFO Level Register"]
    pub txflr: TXFLR,
    #[doc = "0x24 - Receive FIFO Level Register"]
    pub rxflr: RXFLR,
    #[doc = "0x28 - Status Register"]
    pub sr: SR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x34 - Raw Interrupt Status Register"]
    pub risr: RISR,
    #[doc = "0x38 - Transmit FIFO Overflow Interrupt Clear Register"]
    pub txoicr: TXOICR,
    #[doc = "0x3c - Receive FIFO Overflow Interrupt Clear Register"]
    pub rxoicr: RXOICR,
    #[doc = "0x40 - Receive FIFO Underflow Interrupt Clear Register"]
    pub rxuicr: RXUICR,
    #[doc = "0x44 - Multi-Master Interrupt Clear Register"]
    pub msticr: MSTICR,
    #[doc = "0x48 - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x4c - DMA Control Register"]
    pub dmacr: DMACR,
    #[doc = "0x50 - DMA Transmit Data Level"]
    pub dmatdlr: DMATDLR,
    #[doc = "0x54 - DMA Receive Data Level"]
    pub dmardlr: DMARDLR,
    #[doc = "0x58 - Identification Register"]
    pub idr: IDR,
    #[doc = "0x5c - DWC_ssi component version"]
    pub ssic_version_id: SSIC_VERSION_ID,
    #[doc = "0x60 - Data Register"]
    pub dr: [DR; 36],
    #[doc = "0xf0 - RX Sample Delay Register"]
    pub rx_sample_delay: RX_SAMPLE_DELAY,
    #[doc = "0xf4 - SPI Control Register"]
    pub spi_ctrlr0: SPI_CTRLR0,
    _reserved27: [u8; 4usize],
    #[doc = "0xfc - XIP Mode bits"]
    pub xip_mode_bits: XIP_MODE_BITS,
    #[doc = "0x100 - XIP INCR transfer opcode"]
    pub xip_incr_inst: XIP_INCR_INST,
    #[doc = "0x104 - XIP WRAP transfer opcode"]
    pub xip_wrap_inst: XIP_WRAP_INST,
    #[doc = "0x108 - XIP Control Register"]
    pub xip_ctrl: XIP_CTRL,
    #[doc = "0x10c - XIP Slave Enable Register"]
    pub xip_ser: XIP_SER,
    #[doc = "0x110 - XIP Receive FIFO Overflow Interrupt Clear Register"]
    pub xrxoicr: XRXOICR,
    #[doc = "0x114 - XIP time out register for continuous transfers"]
    pub xip_cnt_time_out: XIP_CNT_TIME_OUT,
    #[doc = "0x118 - ENDIAN"]
    pub endian: ENDIAN,
}
#[doc = "Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlr0](ctrlr0) module"]
pub type CTRLR0 = crate::Reg<u32, _CTRLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLR0;
#[doc = "`read()` method returns [ctrlr0::R](ctrlr0::R) reader structure"]
impl crate::Readable for CTRLR0 {}
#[doc = "`write(|w| ..)` method takes [ctrlr0::W](ctrlr0::W) writer structure"]
impl crate::Writable for CTRLR0 {}
#[doc = "Control Register 0"]
pub mod ctrlr0;
#[doc = "Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlr1](ctrlr1) module"]
pub type CTRLR1 = crate::Reg<u32, _CTRLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLR1;
#[doc = "`read()` method returns [ctrlr1::R](ctrlr1::R) reader structure"]
impl crate::Readable for CTRLR1 {}
#[doc = "`write(|w| ..)` method takes [ctrlr1::W](ctrlr1::W) writer structure"]
impl crate::Writable for CTRLR1 {}
#[doc = "Control Register 1"]
pub mod ctrlr1;
#[doc = "Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssienr](ssienr) module"]
pub type SSIENR = crate::Reg<u32, _SSIENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIENR;
#[doc = "`read()` method returns [ssienr::R](ssienr::R) reader structure"]
impl crate::Readable for SSIENR {}
#[doc = "`write(|w| ..)` method takes [ssienr::W](ssienr::W) writer structure"]
impl crate::Writable for SSIENR {}
#[doc = "Enable Register"]
pub mod ssienr;
#[doc = "Microwire Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mwcr](mwcr) module"]
pub type MWCR = crate::Reg<u32, _MWCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MWCR;
#[doc = "`read()` method returns [mwcr::R](mwcr::R) reader structure"]
impl crate::Readable for MWCR {}
#[doc = "`write(|w| ..)` method takes [mwcr::W](mwcr::W) writer structure"]
impl crate::Writable for MWCR {}
#[doc = "Microwire Control Register"]
pub mod mwcr;
#[doc = "Slave Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ser](ser) module"]
pub type SER = crate::Reg<u32, _SER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SER;
#[doc = "`read()` method returns [ser::R](ser::R) reader structure"]
impl crate::Readable for SER {}
#[doc = "`write(|w| ..)` method takes [ser::W](ser::W) writer structure"]
impl crate::Writable for SER {}
#[doc = "Slave Enable Register"]
pub mod ser;
#[doc = "Baud Rate Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [baudr](baudr) module"]
pub type BAUDR = crate::Reg<u32, _BAUDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BAUDR;
#[doc = "`read()` method returns [baudr::R](baudr::R) reader structure"]
impl crate::Readable for BAUDR {}
#[doc = "`write(|w| ..)` method takes [baudr::W](baudr::W) writer structure"]
impl crate::Writable for BAUDR {}
#[doc = "Baud Rate Select"]
pub mod baudr;
#[doc = "Transmit FIFO Threshold Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txftlr](txftlr) module"]
pub type TXFTLR = crate::Reg<u32, _TXFTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFTLR;
#[doc = "`read()` method returns [txftlr::R](txftlr::R) reader structure"]
impl crate::Readable for TXFTLR {}
#[doc = "`write(|w| ..)` method takes [txftlr::W](txftlr::W) writer structure"]
impl crate::Writable for TXFTLR {}
#[doc = "Transmit FIFO Threshold Level"]
pub mod txftlr;
#[doc = "Receive FIFO Threshold Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxftlr](rxftlr) module"]
pub type RXFTLR = crate::Reg<u32, _RXFTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFTLR;
#[doc = "`read()` method returns [rxftlr::R](rxftlr::R) reader structure"]
impl crate::Readable for RXFTLR {}
#[doc = "`write(|w| ..)` method takes [rxftlr::W](rxftlr::W) writer structure"]
impl crate::Writable for RXFTLR {}
#[doc = "Receive FIFO Threshold Level"]
pub mod rxftlr;
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
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [risr](risr) module"]
pub type RISR = crate::Reg<u32, _RISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RISR;
#[doc = "`read()` method returns [risr::R](risr::R) reader structure"]
impl crate::Readable for RISR {}
#[doc = "`write(|w| ..)` method takes [risr::W](risr::W) writer structure"]
impl crate::Writable for RISR {}
#[doc = "Raw Interrupt Status Register"]
pub mod risr;
#[doc = "Transmit FIFO Overflow Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txoicr](txoicr) module"]
pub type TXOICR = crate::Reg<u32, _TXOICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXOICR;
#[doc = "`read()` method returns [txoicr::R](txoicr::R) reader structure"]
impl crate::Readable for TXOICR {}
#[doc = "`write(|w| ..)` method takes [txoicr::W](txoicr::W) writer structure"]
impl crate::Writable for TXOICR {}
#[doc = "Transmit FIFO Overflow Interrupt Clear Register"]
pub mod txoicr;
#[doc = "Receive FIFO Overflow Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxoicr](rxoicr) module"]
pub type RXOICR = crate::Reg<u32, _RXOICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXOICR;
#[doc = "`read()` method returns [rxoicr::R](rxoicr::R) reader structure"]
impl crate::Readable for RXOICR {}
#[doc = "`write(|w| ..)` method takes [rxoicr::W](rxoicr::W) writer structure"]
impl crate::Writable for RXOICR {}
#[doc = "Receive FIFO Overflow Interrupt Clear Register"]
pub mod rxoicr;
#[doc = "Receive FIFO Underflow Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxuicr](rxuicr) module"]
pub type RXUICR = crate::Reg<u32, _RXUICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXUICR;
#[doc = "`read()` method returns [rxuicr::R](rxuicr::R) reader structure"]
impl crate::Readable for RXUICR {}
#[doc = "`write(|w| ..)` method takes [rxuicr::W](rxuicr::W) writer structure"]
impl crate::Writable for RXUICR {}
#[doc = "Receive FIFO Underflow Interrupt Clear Register"]
pub mod rxuicr;
#[doc = "Multi-Master Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msticr](msticr) module"]
pub type MSTICR = crate::Reg<u32, _MSTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSTICR;
#[doc = "`read()` method returns [msticr::R](msticr::R) reader structure"]
impl crate::Readable for MSTICR {}
#[doc = "`write(|w| ..)` method takes [msticr::W](msticr::W) writer structure"]
impl crate::Writable for MSTICR {}
#[doc = "Multi-Master Interrupt Clear Register"]
pub mod msticr;
#[doc = "Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmacr](dmacr) module"]
pub type DMACR = crate::Reg<u32, _DMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACR;
#[doc = "`read()` method returns [dmacr::R](dmacr::R) reader structure"]
impl crate::Readable for DMACR {}
#[doc = "`write(|w| ..)` method takes [dmacr::W](dmacr::W) writer structure"]
impl crate::Writable for DMACR {}
#[doc = "DMA Control Register"]
pub mod dmacr;
#[doc = "DMA Transmit Data Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmatdlr](dmatdlr) module"]
pub type DMATDLR = crate::Reg<u32, _DMATDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMATDLR;
#[doc = "`read()` method returns [dmatdlr::R](dmatdlr::R) reader structure"]
impl crate::Readable for DMATDLR {}
#[doc = "`write(|w| ..)` method takes [dmatdlr::W](dmatdlr::W) writer structure"]
impl crate::Writable for DMATDLR {}
#[doc = "DMA Transmit Data Level"]
pub mod dmatdlr;
#[doc = "DMA Receive Data Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmardlr](dmardlr) module"]
pub type DMARDLR = crate::Reg<u32, _DMARDLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARDLR;
#[doc = "`read()` method returns [dmardlr::R](dmardlr::R) reader structure"]
impl crate::Readable for DMARDLR {}
#[doc = "`write(|w| ..)` method takes [dmardlr::W](dmardlr::W) writer structure"]
impl crate::Writable for DMARDLR {}
#[doc = "DMA Receive Data Level"]
pub mod dmardlr;
#[doc = "Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
impl crate::Readable for IDR {}
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Identification Register"]
pub mod idr;
#[doc = "DWC_ssi component version\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ssic_version_id](ssic_version_id) module"]
pub type SSIC_VERSION_ID = crate::Reg<u32, _SSIC_VERSION_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIC_VERSION_ID;
#[doc = "`read()` method returns [ssic_version_id::R](ssic_version_id::R) reader structure"]
impl crate::Readable for SSIC_VERSION_ID {}
#[doc = "`write(|w| ..)` method takes [ssic_version_id::W](ssic_version_id::W) writer structure"]
impl crate::Writable for SSIC_VERSION_ID {}
#[doc = "DWC_ssi component version"]
pub mod ssic_version_id;
#[doc = "Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "Data Register"]
pub mod dr;
#[doc = "RX Sample Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rx_sample_delay](rx_sample_delay) module"]
pub type RX_SAMPLE_DELAY = crate::Reg<u32, _RX_SAMPLE_DELAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RX_SAMPLE_DELAY;
#[doc = "`read()` method returns [rx_sample_delay::R](rx_sample_delay::R) reader structure"]
impl crate::Readable for RX_SAMPLE_DELAY {}
#[doc = "`write(|w| ..)` method takes [rx_sample_delay::W](rx_sample_delay::W) writer structure"]
impl crate::Writable for RX_SAMPLE_DELAY {}
#[doc = "RX Sample Delay Register"]
pub mod rx_sample_delay;
#[doc = "SPI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_ctrlr0](spi_ctrlr0) module"]
pub type SPI_CTRLR0 = crate::Reg<u32, _SPI_CTRLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CTRLR0;
#[doc = "`read()` method returns [spi_ctrlr0::R](spi_ctrlr0::R) reader structure"]
impl crate::Readable for SPI_CTRLR0 {}
#[doc = "`write(|w| ..)` method takes [spi_ctrlr0::W](spi_ctrlr0::W) writer structure"]
impl crate::Writable for SPI_CTRLR0 {}
#[doc = "SPI Control Register"]
pub mod spi_ctrlr0;
#[doc = "XIP Mode bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xip_mode_bits](xip_mode_bits) module"]
pub type XIP_MODE_BITS = crate::Reg<u32, _XIP_MODE_BITS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIP_MODE_BITS;
#[doc = "`read()` method returns [xip_mode_bits::R](xip_mode_bits::R) reader structure"]
impl crate::Readable for XIP_MODE_BITS {}
#[doc = "`write(|w| ..)` method takes [xip_mode_bits::W](xip_mode_bits::W) writer structure"]
impl crate::Writable for XIP_MODE_BITS {}
#[doc = "XIP Mode bits"]
pub mod xip_mode_bits;
#[doc = "XIP INCR transfer opcode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xip_incr_inst](xip_incr_inst) module"]
pub type XIP_INCR_INST = crate::Reg<u32, _XIP_INCR_INST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIP_INCR_INST;
#[doc = "`read()` method returns [xip_incr_inst::R](xip_incr_inst::R) reader structure"]
impl crate::Readable for XIP_INCR_INST {}
#[doc = "`write(|w| ..)` method takes [xip_incr_inst::W](xip_incr_inst::W) writer structure"]
impl crate::Writable for XIP_INCR_INST {}
#[doc = "XIP INCR transfer opcode"]
pub mod xip_incr_inst;
#[doc = "XIP WRAP transfer opcode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xip_wrap_inst](xip_wrap_inst) module"]
pub type XIP_WRAP_INST = crate::Reg<u32, _XIP_WRAP_INST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIP_WRAP_INST;
#[doc = "`read()` method returns [xip_wrap_inst::R](xip_wrap_inst::R) reader structure"]
impl crate::Readable for XIP_WRAP_INST {}
#[doc = "`write(|w| ..)` method takes [xip_wrap_inst::W](xip_wrap_inst::W) writer structure"]
impl crate::Writable for XIP_WRAP_INST {}
#[doc = "XIP WRAP transfer opcode"]
pub mod xip_wrap_inst;
#[doc = "XIP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xip_ctrl](xip_ctrl) module"]
pub type XIP_CTRL = crate::Reg<u32, _XIP_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIP_CTRL;
#[doc = "`read()` method returns [xip_ctrl::R](xip_ctrl::R) reader structure"]
impl crate::Readable for XIP_CTRL {}
#[doc = "`write(|w| ..)` method takes [xip_ctrl::W](xip_ctrl::W) writer structure"]
impl crate::Writable for XIP_CTRL {}
#[doc = "XIP Control Register"]
pub mod xip_ctrl;
#[doc = "XIP Slave Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xip_ser](xip_ser) module"]
pub type XIP_SER = crate::Reg<u32, _XIP_SER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIP_SER;
#[doc = "`read()` method returns [xip_ser::R](xip_ser::R) reader structure"]
impl crate::Readable for XIP_SER {}
#[doc = "`write(|w| ..)` method takes [xip_ser::W](xip_ser::W) writer structure"]
impl crate::Writable for XIP_SER {}
#[doc = "XIP Slave Enable Register"]
pub mod xip_ser;
#[doc = "XIP Receive FIFO Overflow Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xrxoicr](xrxoicr) module"]
pub type XRXOICR = crate::Reg<u32, _XRXOICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XRXOICR;
#[doc = "`read()` method returns [xrxoicr::R](xrxoicr::R) reader structure"]
impl crate::Readable for XRXOICR {}
#[doc = "`write(|w| ..)` method takes [xrxoicr::W](xrxoicr::W) writer structure"]
impl crate::Writable for XRXOICR {}
#[doc = "XIP Receive FIFO Overflow Interrupt Clear Register"]
pub mod xrxoicr;
#[doc = "XIP time out register for continuous transfers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xip_cnt_time_out](xip_cnt_time_out) module"]
pub type XIP_CNT_TIME_OUT = crate::Reg<u32, _XIP_CNT_TIME_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XIP_CNT_TIME_OUT;
#[doc = "`read()` method returns [xip_cnt_time_out::R](xip_cnt_time_out::R) reader structure"]
impl crate::Readable for XIP_CNT_TIME_OUT {}
#[doc = "`write(|w| ..)` method takes [xip_cnt_time_out::W](xip_cnt_time_out::W) writer structure"]
impl crate::Writable for XIP_CNT_TIME_OUT {}
#[doc = "XIP time out register for continuous transfers"]
pub mod xip_cnt_time_out;
#[doc = "ENDIAN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [endian](endian) module"]
pub type ENDIAN = crate::Reg<u32, _ENDIAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENDIAN;
#[doc = "`read()` method returns [endian::R](endian::R) reader structure"]
impl crate::Readable for ENDIAN {}
#[doc = "`write(|w| ..)` method takes [endian::W](endian::W) writer structure"]
impl crate::Writable for ENDIAN {}
#[doc = "ENDIAN"]
pub mod endian;
