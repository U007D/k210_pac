#[doc = "SAR Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sar](sar) module"]
pub type SAR = crate::Reg<u64, _SAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAR;
#[doc = "`read()` method returns [sar::R](sar::R) reader structure"]
impl crate::Readable for SAR {}
#[doc = "`write(|w| ..)` method takes [sar::W](sar::W) writer structure"]
impl crate::Writable for SAR {}
#[doc = "SAR Address Register"]
pub mod sar;
#[doc = "DAR Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dar](dar) module"]
pub type DAR = crate::Reg<u64, _DAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAR;
#[doc = "`read()` method returns [dar::R](dar::R) reader structure"]
impl crate::Readable for DAR {}
#[doc = "`write(|w| ..)` method takes [dar::W](dar::W) writer structure"]
impl crate::Writable for DAR {}
#[doc = "DAR Address Register"]
pub mod dar;
#[doc = "Block Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [block_ts](block_ts) module"]
pub type BLOCK_TS = crate::Reg<u64, _BLOCK_TS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLOCK_TS;
#[doc = "`read()` method returns [block_ts::R](block_ts::R) reader structure"]
impl crate::Readable for BLOCK_TS {}
#[doc = "`write(|w| ..)` method takes [block_ts::W](block_ts::W) writer structure"]
impl crate::Writable for BLOCK_TS {}
#[doc = "Block Transfer Size Register"]
pub mod block_ts;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u64, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control Register"]
pub mod ctl;
#[doc = "Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u64, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configure Register"]
pub mod cfg;
#[doc = "Linked List Pointer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [llp](llp) module"]
pub type LLP = crate::Reg<u64, _LLP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LLP;
#[doc = "`read()` method returns [llp::R](llp::R) reader structure"]
impl crate::Readable for LLP {}
#[doc = "`write(|w| ..)` method takes [llp::W](llp::W) writer structure"]
impl crate::Writable for LLP {}
#[doc = "Linked List Pointer register"]
pub mod llp;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u64, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Channel Status Register"]
pub mod status;
#[doc = "Channel Software handshake Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [swhssrc](swhssrc) module"]
pub type SWHSSRC = crate::Reg<u64, _SWHSSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWHSSRC;
#[doc = "`read()` method returns [swhssrc::R](swhssrc::R) reader structure"]
impl crate::Readable for SWHSSRC {}
#[doc = "`write(|w| ..)` method takes [swhssrc::W](swhssrc::W) writer structure"]
impl crate::Writable for SWHSSRC {}
#[doc = "Channel Software handshake Source Register"]
pub mod swhssrc;
#[doc = "Channel Software handshake Destination Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [swhsdst](swhsdst) module"]
pub type SWHSDST = crate::Reg<u64, _SWHSDST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWHSDST;
#[doc = "`read()` method returns [swhsdst::R](swhsdst::R) reader structure"]
impl crate::Readable for SWHSDST {}
#[doc = "`write(|w| ..)` method takes [swhsdst::W](swhsdst::W) writer structure"]
impl crate::Writable for SWHSDST {}
#[doc = "Channel Software handshake Destination Register"]
pub mod swhsdst;
#[doc = "Channel Block Transfer Resume Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk_tfr](blk_tfr) module"]
pub type BLK_TFR = crate::Reg<u64, _BLK_TFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK_TFR;
#[doc = "`read()` method returns [blk_tfr::R](blk_tfr::R) reader structure"]
impl crate::Readable for BLK_TFR {}
#[doc = "`write(|w| ..)` method takes [blk_tfr::W](blk_tfr::W) writer structure"]
impl crate::Writable for BLK_TFR {}
#[doc = "Channel Block Transfer Resume Request Register"]
pub mod blk_tfr;
#[doc = "Channel AXI ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [axi_id](axi_id) module"]
pub type AXI_ID = crate::Reg<u64, _AXI_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXI_ID;
#[doc = "`read()` method returns [axi_id::R](axi_id::R) reader structure"]
impl crate::Readable for AXI_ID {}
#[doc = "`write(|w| ..)` method takes [axi_id::W](axi_id::W) writer structure"]
impl crate::Writable for AXI_ID {}
#[doc = "Channel AXI ID Register"]
pub mod axi_id;
#[doc = "AXI QOS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [axi_qos](axi_qos) module"]
pub type AXI_QOS = crate::Reg<u64, _AXI_QOS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXI_QOS;
#[doc = "`read()` method returns [axi_qos::R](axi_qos::R) reader structure"]
impl crate::Readable for AXI_QOS {}
#[doc = "`write(|w| ..)` method takes [axi_qos::W](axi_qos::W) writer structure"]
impl crate::Writable for AXI_QOS {}
#[doc = "AXI QOS Register"]
pub mod axi_qos;
#[doc = "Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstatus_en](intstatus_en) module"]
pub type INTSTATUS_EN = crate::Reg<u64, _INTSTATUS_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS_EN;
#[doc = "`read()` method returns [intstatus_en::R](intstatus_en::R) reader structure"]
impl crate::Readable for INTSTATUS_EN {}
#[doc = "`write(|w| ..)` method takes [intstatus_en::W](intstatus_en::W) writer structure"]
impl crate::Writable for INTSTATUS_EN {}
#[doc = "Interrupt Status Enable Register"]
pub mod intstatus_en;
#[doc = "Channel Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u64, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "`write(|w| ..)` method takes [intstatus::W](intstatus::W) writer structure"]
impl crate::Writable for INTSTATUS {}
#[doc = "Channel Interrupt Status Register"]
pub mod intstatus;
#[doc = "Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intsignal_en](intsignal_en) module"]
pub type INTSIGNAL_EN = crate::Reg<u64, _INTSIGNAL_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSIGNAL_EN;
#[doc = "`read()` method returns [intsignal_en::R](intsignal_en::R) reader structure"]
impl crate::Readable for INTSIGNAL_EN {}
#[doc = "`write(|w| ..)` method takes [intsignal_en::W](intsignal_en::W) writer structure"]
impl crate::Writable for INTSIGNAL_EN {}
#[doc = "Interrupt Signal Enable Register"]
pub mod intsignal_en;
#[doc = "Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intclear](intclear) module"]
pub type INTCLEAR = crate::Reg<u64, _INTCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLEAR;
#[doc = "`read()` method returns [intclear::R](intclear::R) reader structure"]
impl crate::Readable for INTCLEAR {}
#[doc = "`write(|w| ..)` method takes [intclear::W](intclear::W) writer structure"]
impl crate::Writable for INTCLEAR {}
#[doc = "Interrupt Clear Register"]
pub mod intclear;
#[doc = "Padding to make structure size 256 bytes so that channels\\[\\] is an array\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_reserved](_reserved) module"]
pub type _RESERVED = crate::Reg<u64, __RESERVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __RESERVED;
#[doc = "`read()` method returns [_reserved::R](_reserved::R) reader structure"]
impl crate::Readable for _RESERVED {}
#[doc = "`write(|w| ..)` method takes [_reserved::W](_reserved::W) writer structure"]
impl crate::Writable for _RESERVED {}
#[doc = "Padding to make structure size 256 bytes so that channels\\[\\] is an array"]
pub mod _reserved;
