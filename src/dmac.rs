#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ID Register"]
    pub id: ID,
    #[doc = "0x08 - COMPVER Register"]
    pub compver: COMPVER,
    #[doc = "0x10 - Configure Register"]
    pub cfg: CFG,
    #[doc = "0x18 - Channel Enable Register"]
    pub chen: CHEN,
    _reserved4: [u8; 16usize],
    #[doc = "0x30 - Interrupt Status Register"]
    pub intstatus: INTSTATUS,
    #[doc = "0x38 - Common Interrupt Clear Register"]
    pub com_intclear: COM_INTCLEAR,
    #[doc = "0x40 - Common Interrupt Status Enable Register"]
    pub com_intstatus_en: COM_INTSTATUS_EN,
    #[doc = "0x48 - Common Interrupt Signal Enable Register"]
    pub com_intsignal_en: COM_INTSIGNAL_EN,
    #[doc = "0x50 - Common Interrupt Status"]
    pub com_intstatus: COM_INTSTATUS,
    #[doc = "0x58 - Reset register"]
    pub reset: RESET,
    _reserved10: [u8; 160usize],
    #[doc = "0x100 - Channel configuration"]
    pub channel: [CHANNEL; 6],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - SAR Address Register"]
    pub sar: self::channel::SAR,
    #[doc = "0x08 - DAR Address Register"]
    pub dar: self::channel::DAR,
    #[doc = "0x10 - Block Transfer Size Register"]
    pub block_ts: self::channel::BLOCK_TS,
    #[doc = "0x18 - Control Register"]
    pub ctl: self::channel::CTL,
    #[doc = "0x20 - Configure Register"]
    pub cfg: self::channel::CFG,
    #[doc = "0x28 - Linked List Pointer register"]
    pub llp: self::channel::LLP,
    #[doc = "0x30 - Channel Status Register"]
    pub status: self::channel::STATUS,
    #[doc = "0x38 - Channel Software handshake Source Register"]
    pub swhssrc: self::channel::SWHSSRC,
    #[doc = "0x40 - Channel Software handshake Destination Register"]
    pub swhsdst: self::channel::SWHSDST,
    #[doc = "0x48 - Channel Block Transfer Resume Request Register"]
    pub blk_tfr: self::channel::BLK_TFR,
    #[doc = "0x50 - Channel AXI ID Register"]
    pub axi_id: self::channel::AXI_ID,
    #[doc = "0x58 - AXI QOS Register"]
    pub axi_qos: self::channel::AXI_QOS,
    _reserved12: [u8; 32usize],
    #[doc = "0x80 - Interrupt Status Enable Register"]
    pub intstatus_en: self::channel::INTSTATUS_EN,
    #[doc = "0x88 - Channel Interrupt Status Register"]
    pub intstatus: self::channel::INTSTATUS,
    #[doc = "0x90 - Interrupt Signal Enable Register"]
    pub intsignal_en: self::channel::INTSIGNAL_EN,
    #[doc = "0x98 - Interrupt Clear Register"]
    pub intclear: self::channel::INTCLEAR,
    _reserved16: [u8; 88usize],
    #[doc = "0xf8 - Padding to make structure size 256 bytes so that channels\\[\\] is an array"]
    pub _reserved: self::channel::_RESERVED,
}
#[doc = r"Register block"]
#[doc = "Channel configuration"]
pub mod channel;
#[doc = "ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id](id) module"]
pub type ID = crate::Reg<u64, _ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID;
#[doc = "`read()` method returns [id::R](id::R) reader structure"]
impl crate::Readable for ID {}
#[doc = "`write(|w| ..)` method takes [id::W](id::W) writer structure"]
impl crate::Writable for ID {}
#[doc = "ID Register"]
pub mod id;
#[doc = "COMPVER Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [compver](compver) module"]
pub type COMPVER = crate::Reg<u64, _COMPVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPVER;
#[doc = "`read()` method returns [compver::R](compver::R) reader structure"]
impl crate::Readable for COMPVER {}
#[doc = "`write(|w| ..)` method takes [compver::W](compver::W) writer structure"]
impl crate::Writable for COMPVER {}
#[doc = "COMPVER Register"]
pub mod compver;
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
#[doc = "Channel Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chen](chen) module"]
pub type CHEN = crate::Reg<u64, _CHEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEN;
#[doc = "`read()` method returns [chen::R](chen::R) reader structure"]
impl crate::Readable for CHEN {}
#[doc = "`write(|w| ..)` method takes [chen::W](chen::W) writer structure"]
impl crate::Writable for CHEN {}
#[doc = "Channel Enable Register"]
pub mod chen;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u64, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "`write(|w| ..)` method takes [intstatus::W](intstatus::W) writer structure"]
impl crate::Writable for INTSTATUS {}
#[doc = "Interrupt Status Register"]
pub mod intstatus;
#[doc = "Common Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [com_intclear](com_intclear) module"]
pub type COM_INTCLEAR = crate::Reg<u64, _COM_INTCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COM_INTCLEAR;
#[doc = "`read()` method returns [com_intclear::R](com_intclear::R) reader structure"]
impl crate::Readable for COM_INTCLEAR {}
#[doc = "`write(|w| ..)` method takes [com_intclear::W](com_intclear::W) writer structure"]
impl crate::Writable for COM_INTCLEAR {}
#[doc = "Common Interrupt Clear Register"]
pub mod com_intclear;
#[doc = "Common Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [com_intstatus_en](com_intstatus_en) module"]
pub type COM_INTSTATUS_EN = crate::Reg<u64, _COM_INTSTATUS_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COM_INTSTATUS_EN;
#[doc = "`read()` method returns [com_intstatus_en::R](com_intstatus_en::R) reader structure"]
impl crate::Readable for COM_INTSTATUS_EN {}
#[doc = "`write(|w| ..)` method takes [com_intstatus_en::W](com_intstatus_en::W) writer structure"]
impl crate::Writable for COM_INTSTATUS_EN {}
#[doc = "Common Interrupt Status Enable Register"]
pub mod com_intstatus_en;
#[doc = "Common Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [com_intsignal_en](com_intsignal_en) module"]
pub type COM_INTSIGNAL_EN = crate::Reg<u64, _COM_INTSIGNAL_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COM_INTSIGNAL_EN;
#[doc = "`read()` method returns [com_intsignal_en::R](com_intsignal_en::R) reader structure"]
impl crate::Readable for COM_INTSIGNAL_EN {}
#[doc = "`write(|w| ..)` method takes [com_intsignal_en::W](com_intsignal_en::W) writer structure"]
impl crate::Writable for COM_INTSIGNAL_EN {}
#[doc = "Common Interrupt Signal Enable Register"]
pub mod com_intsignal_en;
#[doc = "Common Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [com_intstatus](com_intstatus) module"]
pub type COM_INTSTATUS = crate::Reg<u64, _COM_INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COM_INTSTATUS;
#[doc = "`read()` method returns [com_intstatus::R](com_intstatus::R) reader structure"]
impl crate::Readable for COM_INTSTATUS {}
#[doc = "`write(|w| ..)` method takes [com_intstatus::W](com_intstatus::W) writer structure"]
impl crate::Writable for COM_INTSTATUS {}
#[doc = "Common Interrupt Status"]
pub mod com_intstatus;
#[doc = "Reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reset](reset) module"]
pub type RESET = crate::Reg<u64, _RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET;
#[doc = "`read()` method returns [reset::R](reset::R) reader structure"]
impl crate::Readable for RESET {}
#[doc = "`write(|w| ..)` method takes [reset::W](reset::W) writer structure"]
impl crate::Writable for RESET {}
#[doc = "Reset register"]
pub mod reset;
