#[doc = "Left Receive or Left Transmit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [left_rxtx](left_rxtx) module"]
pub type LEFT_RXTX = crate::Reg<u32, _LEFT_RXTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEFT_RXTX;
#[doc = "`read()` method returns [left_rxtx::R](left_rxtx::R) reader structure"]
impl crate::Readable for LEFT_RXTX {}
#[doc = "`write(|w| ..)` method takes [left_rxtx::W](left_rxtx::W) writer structure"]
impl crate::Writable for LEFT_RXTX {}
#[doc = "Left Receive or Left Transmit Register"]
pub mod left_rxtx;
#[doc = "Right Receive or Right Transmit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [right_rxtx](right_rxtx) module"]
pub type RIGHT_RXTX = crate::Reg<u32, _RIGHT_RXTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIGHT_RXTX;
#[doc = "`read()` method returns [right_rxtx::R](right_rxtx::R) reader structure"]
impl crate::Readable for RIGHT_RXTX {}
#[doc = "`write(|w| ..)` method takes [right_rxtx::W](right_rxtx::W) writer structure"]
impl crate::Writable for RIGHT_RXTX {}
#[doc = "Right Receive or Right Transmit Register"]
pub mod right_rxtx;
#[doc = "Receive Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rer](rer) module"]
pub type RER = crate::Reg<u32, _RER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RER;
#[doc = "`read()` method returns [rer::R](rer::R) reader structure"]
impl crate::Readable for RER {}
#[doc = "`write(|w| ..)` method takes [rer::W](rer::W) writer structure"]
impl crate::Writable for RER {}
#[doc = "Receive Enable Register"]
pub mod rer;
#[doc = "Transmit Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ter](ter) module"]
pub type TER = crate::Reg<u32, _TER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TER;
#[doc = "`read()` method returns [ter::R](ter::R) reader structure"]
impl crate::Readable for TER {}
#[doc = "`write(|w| ..)` method takes [ter::W](ter::W) writer structure"]
impl crate::Writable for TER {}
#[doc = "Transmit Enable Register"]
pub mod ter;
#[doc = "Receive Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcr](rcr) module"]
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
#[doc = "`read()` method returns [rcr::R](rcr::R) reader structure"]
impl crate::Readable for RCR {}
#[doc = "`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure"]
impl crate::Writable for RCR {}
#[doc = "Receive Configuration Register"]
pub mod rcr;
#[doc = "Transmit Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "Transmit Configuration Register"]
pub mod tcr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
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
#[doc = "Receive Overrun Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ror](ror) module"]
pub type ROR = crate::Reg<u32, _ROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROR;
#[doc = "`read()` method returns [ror::R](ror::R) reader structure"]
impl crate::Readable for ROR {}
#[doc = "Receive Overrun Register"]
pub mod ror;
#[doc = "Transmit Overrun Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tor](tor) module"]
pub type TOR = crate::Reg<u32, _TOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TOR;
#[doc = "`read()` method returns [tor::R](tor::R) reader structure"]
impl crate::Readable for TOR {}
#[doc = "Transmit Overrun Register"]
pub mod tor;
#[doc = "Receive FIFO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfcr](rfcr) module"]
pub type RFCR = crate::Reg<u32, _RFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFCR;
#[doc = "`read()` method returns [rfcr::R](rfcr::R) reader structure"]
impl crate::Readable for RFCR {}
#[doc = "`write(|w| ..)` method takes [rfcr::W](rfcr::W) writer structure"]
impl crate::Writable for RFCR {}
#[doc = "Receive FIFO Configuration Register"]
pub mod rfcr;
#[doc = "Transmit FIFO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tfcr](tfcr) module"]
pub type TFCR = crate::Reg<u32, _TFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFCR;
#[doc = "`read()` method returns [tfcr::R](tfcr::R) reader structure"]
impl crate::Readable for TFCR {}
#[doc = "`write(|w| ..)` method takes [tfcr::W](tfcr::W) writer structure"]
impl crate::Writable for TFCR {}
#[doc = "Transmit FIFO Configuration Register"]
pub mod tfcr;
#[doc = "Receive FIFO Flush Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rff](rff) module"]
pub type RFF = crate::Reg<u32, _RFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFF;
#[doc = "`read()` method returns [rff::R](rff::R) reader structure"]
impl crate::Readable for RFF {}
#[doc = "`write(|w| ..)` method takes [rff::W](rff::W) writer structure"]
impl crate::Writable for RFF {}
#[doc = "Receive FIFO Flush Register"]
pub mod rff;
#[doc = "Transmit FIFO Flush Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tff](tff) module"]
pub type TFF = crate::Reg<u32, _TFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFF;
#[doc = "`read()` method returns [tff::R](tff::R) reader structure"]
impl crate::Readable for TFF {}
#[doc = "`write(|w| ..)` method takes [tff::W](tff::W) writer structure"]
impl crate::Writable for TFF {}
#[doc = "Transmit FIFO Flush Register"]
pub mod tff;
#[doc = "_RESERVED0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_reserved](_reserved) module"]
pub type _RESERVED = crate::Reg<u32, __RESERVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __RESERVED;
#[doc = "`read()` method returns [_reserved::R](_reserved::R) reader structure"]
impl crate::Readable for _RESERVED {}
#[doc = "`write(|w| ..)` method takes [_reserved::W](_reserved::W) writer structure"]
impl crate::Writable for _RESERVED {}
#[doc = "_RESERVED0"]
pub mod _reserved;
