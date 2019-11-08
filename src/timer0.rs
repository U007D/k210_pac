#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel cluster: load_count, current_value, control, eoi and intr_stat registers"]
    pub channel: [CHANNEL; 4],
    _reserved1: [u8; 80usize],
    #[doc = "0xa0 - Interrupt Status Register"]
    pub intr_stat: INTR_STAT,
    #[doc = "0xa4 - Interrupt Clear Register"]
    pub eoi: EOI,
    #[doc = "0xa8 - Raw Interrupt Status Register"]
    pub raw_intr_stat: RAW_INTR_STAT,
    #[doc = "0xac - Component Version Register"]
    pub comp_version: COMP_VERSION,
    #[doc = "0xb0 - Load Count2 Register"]
    pub load_count2: [LOAD_COUNT2; 4],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Load Count Register"]
    pub load_count: self::channel::LOAD_COUNT,
    #[doc = "0x04 - Current Value Register"]
    pub current_value: self::channel::CURRENT_VALUE,
    #[doc = "0x08 - Control Register"]
    pub control: self::channel::CONTROL,
    #[doc = "0x0c - Interrupt Clear Register"]
    pub eoi: self::channel::EOI,
    #[doc = "0x10 - Interrupt Status Register"]
    pub intr_stat: self::channel::INTR_STAT,
}
#[doc = r"Register block"]
#[doc = "Channel cluster: load_count, current_value, control, eoi and intr_stat registers"]
pub mod channel;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_stat](intr_stat) module"]
pub type INTR_STAT = crate::Reg<u32, _INTR_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_STAT;
#[doc = "`read()` method returns [intr_stat::R](intr_stat::R) reader structure"]
impl crate::Readable for INTR_STAT {}
#[doc = "`write(|w| ..)` method takes [intr_stat::W](intr_stat::W) writer structure"]
impl crate::Writable for INTR_STAT {}
#[doc = "Interrupt Status Register"]
pub mod intr_stat;
#[doc = "Interrupt Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eoi](eoi) module"]
pub type EOI = crate::Reg<u32, _EOI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EOI;
#[doc = "`read()` method returns [eoi::R](eoi::R) reader structure"]
impl crate::Readable for EOI {}
#[doc = "`write(|w| ..)` method takes [eoi::W](eoi::W) writer structure"]
impl crate::Writable for EOI {}
#[doc = "Interrupt Clear Register"]
pub mod eoi;
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
#[doc = "Component Version Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_version](comp_version) module"]
pub type COMP_VERSION = crate::Reg<u32, _COMP_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_VERSION;
#[doc = "`read()` method returns [comp_version::R](comp_version::R) reader structure"]
impl crate::Readable for COMP_VERSION {}
#[doc = "`write(|w| ..)` method takes [comp_version::W](comp_version::W) writer structure"]
impl crate::Writable for COMP_VERSION {}
#[doc = "Component Version Register"]
pub mod comp_version;
#[doc = "Load Count2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [load_count2](load_count2) module"]
pub type LOAD_COUNT2 = crate::Reg<u32, _LOAD_COUNT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD_COUNT2;
#[doc = "`read()` method returns [load_count2::R](load_count2::R) reader structure"]
impl crate::Readable for LOAD_COUNT2 {}
#[doc = "`write(|w| ..)` method takes [load_count2::W](load_count2::W) writer structure"]
impl crate::Writable for LOAD_COUNT2 {}
#[doc = "Load Count2 Register"]
pub mod load_count2;
