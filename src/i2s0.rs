#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enable Register"]
    pub ier: IER,
    #[doc = "0x04 - Receiver Block Enable Register"]
    pub irer: IRER,
    #[doc = "0x08 - Transmitter Block Enable Register"]
    pub iter: ITER,
    #[doc = "0x0c - Clock Generation enable"]
    pub cer: CER,
    #[doc = "0x10 - Clock Configuration Register"]
    pub ccr: CCR,
    #[doc = "0x14 - Receiver Block FIFO Reset Register"]
    pub rxffr: RXFFR,
    #[doc = "0x18 - Transmitter Block FIFO Reset Register"]
    pub txffr: TXFFR,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Channel cluster"]
    pub channel: [CHANNEL; 4],
    _reserved8: [u8; 160usize],
    #[doc = "0x1c0 - Receiver Block DMA Register"]
    pub rxdma: RXDMA,
    #[doc = "0x1c4 - Reset Receiver Block DMA Register"]
    pub rrxdma: RRXDMA,
    #[doc = "0x1c8 - Transmitter Block DMA Register"]
    pub txdma: TXDMA,
    #[doc = "0x1cc - Reset Transmitter Block DMA Register"]
    pub rtxdma: RTXDMA,
    _reserved12: [u8; 32usize],
    #[doc = "0x1f0 - Component Parameter Register 2"]
    pub i2s_comp_param_2: I2S_COMP_PARAM_2,
    #[doc = "0x1f4 - Component Parameter Register 1"]
    pub i2s_comp_param_1: I2S_COMP_PARAM_1,
    #[doc = "0x1f8 - Component Version Register"]
    pub i2s_comp_version_1: I2S_COMP_VERSION_1,
    #[doc = "0x1fc - Component Type Register"]
    pub i2s_comp_type: I2S_COMP_TYPE,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Left Receive or Left Transmit Register"]
    pub left_rxtx: self::channel::LEFT_RXTX,
    #[doc = "0x04 - Right Receive or Right Transmit Register"]
    pub right_rxtx: self::channel::RIGHT_RXTX,
    #[doc = "0x08 - Receive Enable Register"]
    pub rer: self::channel::RER,
    #[doc = "0x0c - Transmit Enable Register"]
    pub ter: self::channel::TER,
    #[doc = "0x10 - Receive Configuration Register"]
    pub rcr: self::channel::RCR,
    #[doc = "0x14 - Transmit Configuration Register"]
    pub tcr: self::channel::TCR,
    #[doc = "0x18 - Interrupt Status Register"]
    pub isr: self::channel::ISR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub imr: self::channel::IMR,
    #[doc = "0x20 - Receive Overrun Register"]
    pub ror: self::channel::ROR,
    #[doc = "0x24 - Transmit Overrun Register"]
    pub tor: self::channel::TOR,
    #[doc = "0x28 - Receive FIFO Configuration Register"]
    pub rfcr: self::channel::RFCR,
    #[doc = "0x2c - Transmit FIFO Configuration Register"]
    pub tfcr: self::channel::TFCR,
    #[doc = "0x30 - Receive FIFO Flush Register"]
    pub rff: self::channel::RFF,
    #[doc = "0x34 - Transmit FIFO Flush Register"]
    pub tff: self::channel::TFF,
    #[doc = "0x38 - _RESERVED0"]
    pub _reserved: [self::channel::_RESERVED; 2],
}
#[doc = r"Register block"]
#[doc = "Channel cluster"]
pub mod channel;
#[doc = "Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Enable Register"]
pub mod ier;
#[doc = "Receiver Block Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irer](irer) module"]
pub type IRER = crate::Reg<u32, _IRER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRER;
#[doc = "`read()` method returns [irer::R](irer::R) reader structure"]
impl crate::Readable for IRER {}
#[doc = "`write(|w| ..)` method takes [irer::W](irer::W) writer structure"]
impl crate::Writable for IRER {}
#[doc = "Receiver Block Enable Register"]
pub mod irer;
#[doc = "Transmitter Block Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iter](iter) module"]
pub type ITER = crate::Reg<u32, _ITER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITER;
#[doc = "`read()` method returns [iter::R](iter::R) reader structure"]
impl crate::Readable for ITER {}
#[doc = "`write(|w| ..)` method takes [iter::W](iter::W) writer structure"]
impl crate::Writable for ITER {}
#[doc = "Transmitter Block Enable Register"]
pub mod iter;
#[doc = "Clock Generation enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cer](cer) module"]
pub type CER = crate::Reg<u32, _CER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CER;
#[doc = "`read()` method returns [cer::R](cer::R) reader structure"]
impl crate::Readable for CER {}
#[doc = "`write(|w| ..)` method takes [cer::W](cer::W) writer structure"]
impl crate::Writable for CER {}
#[doc = "Clock Generation enable"]
pub mod cer;
#[doc = "Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Clock Configuration Register"]
pub mod ccr;
#[doc = "Receiver Block FIFO Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxffr](rxffr) module"]
pub type RXFFR = crate::Reg<u32, _RXFFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFFR;
#[doc = "`read()` method returns [rxffr::R](rxffr::R) reader structure"]
impl crate::Readable for RXFFR {}
#[doc = "`write(|w| ..)` method takes [rxffr::W](rxffr::W) writer structure"]
impl crate::Writable for RXFFR {}
#[doc = "Receiver Block FIFO Reset Register"]
pub mod rxffr;
#[doc = "Transmitter Block FIFO Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txffr](txffr) module"]
pub type TXFFR = crate::Reg<u32, _TXFFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFFR;
#[doc = "`read()` method returns [txffr::R](txffr::R) reader structure"]
impl crate::Readable for TXFFR {}
#[doc = "`write(|w| ..)` method takes [txffr::W](txffr::W) writer structure"]
impl crate::Writable for TXFFR {}
#[doc = "Transmitter Block FIFO Reset Register"]
pub mod txffr;
#[doc = "Receiver Block DMA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxdma](rxdma) module"]
pub type RXDMA = crate::Reg<u32, _RXDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDMA;
#[doc = "`read()` method returns [rxdma::R](rxdma::R) reader structure"]
impl crate::Readable for RXDMA {}
#[doc = "`write(|w| ..)` method takes [rxdma::W](rxdma::W) writer structure"]
impl crate::Writable for RXDMA {}
#[doc = "Receiver Block DMA Register"]
pub mod rxdma;
#[doc = "Reset Receiver Block DMA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rrxdma](rrxdma) module"]
pub type RRXDMA = crate::Reg<u32, _RRXDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RRXDMA;
#[doc = "`read()` method returns [rrxdma::R](rrxdma::R) reader structure"]
impl crate::Readable for RRXDMA {}
#[doc = "`write(|w| ..)` method takes [rrxdma::W](rrxdma::W) writer structure"]
impl crate::Writable for RRXDMA {}
#[doc = "Reset Receiver Block DMA Register"]
pub mod rrxdma;
#[doc = "Transmitter Block DMA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txdma](txdma) module"]
pub type TXDMA = crate::Reg<u32, _TXDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDMA;
#[doc = "`read()` method returns [txdma::R](txdma::R) reader structure"]
impl crate::Readable for TXDMA {}
#[doc = "`write(|w| ..)` method takes [txdma::W](txdma::W) writer structure"]
impl crate::Writable for TXDMA {}
#[doc = "Transmitter Block DMA Register"]
pub mod txdma;
#[doc = "Reset Transmitter Block DMA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtxdma](rtxdma) module"]
pub type RTXDMA = crate::Reg<u32, _RTXDMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTXDMA;
#[doc = "`read()` method returns [rtxdma::R](rtxdma::R) reader structure"]
impl crate::Readable for RTXDMA {}
#[doc = "`write(|w| ..)` method takes [rtxdma::W](rtxdma::W) writer structure"]
impl crate::Writable for RTXDMA {}
#[doc = "Reset Transmitter Block DMA Register"]
pub mod rtxdma;
#[doc = "Component Parameter Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_comp_param_2](i2s_comp_param_2) module"]
pub type I2S_COMP_PARAM_2 = crate::Reg<u32, _I2S_COMP_PARAM_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_COMP_PARAM_2;
#[doc = "`read()` method returns [i2s_comp_param_2::R](i2s_comp_param_2::R) reader structure"]
impl crate::Readable for I2S_COMP_PARAM_2 {}
#[doc = "`write(|w| ..)` method takes [i2s_comp_param_2::W](i2s_comp_param_2::W) writer structure"]
impl crate::Writable for I2S_COMP_PARAM_2 {}
#[doc = "Component Parameter Register 2"]
pub mod i2s_comp_param_2;
#[doc = "Component Parameter Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_comp_param_1](i2s_comp_param_1) module"]
pub type I2S_COMP_PARAM_1 = crate::Reg<u32, _I2S_COMP_PARAM_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_COMP_PARAM_1;
#[doc = "`read()` method returns [i2s_comp_param_1::R](i2s_comp_param_1::R) reader structure"]
impl crate::Readable for I2S_COMP_PARAM_1 {}
#[doc = "`write(|w| ..)` method takes [i2s_comp_param_1::W](i2s_comp_param_1::W) writer structure"]
impl crate::Writable for I2S_COMP_PARAM_1 {}
#[doc = "Component Parameter Register 1"]
pub mod i2s_comp_param_1;
#[doc = "Component Version Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_comp_version_1](i2s_comp_version_1) module"]
pub type I2S_COMP_VERSION_1 = crate::Reg<u32, _I2S_COMP_VERSION_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_COMP_VERSION_1;
#[doc = "`read()` method returns [i2s_comp_version_1::R](i2s_comp_version_1::R) reader structure"]
impl crate::Readable for I2S_COMP_VERSION_1 {}
#[doc = "`write(|w| ..)` method takes [i2s_comp_version_1::W](i2s_comp_version_1::W) writer structure"]
impl crate::Writable for I2S_COMP_VERSION_1 {}
#[doc = "Component Version Register"]
pub mod i2s_comp_version_1;
#[doc = "Component Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [i2s_comp_type](i2s_comp_type) module"]
pub type I2S_COMP_TYPE = crate::Reg<u32, _I2S_COMP_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_COMP_TYPE;
#[doc = "`read()` method returns [i2s_comp_type::R](i2s_comp_type::R) reader structure"]
impl crate::Readable for I2S_COMP_TYPE {}
#[doc = "`write(|w| ..)` method takes [i2s_comp_type::W](i2s_comp_type::W) writer structure"]
impl crate::Writable for I2S_COMP_TYPE {}
#[doc = "Component Type Register"]
pub mod i2s_comp_type;
