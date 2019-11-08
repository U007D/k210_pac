#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Timeout Range Register"]
    pub torr: TORR,
    #[doc = "0x08 - Current Counter Value Register"]
    pub ccvr: CCVR,
    #[doc = "0x0c - Counter Restart Register"]
    pub crr: CRR,
    #[doc = "0x10 - Interrupt Status Register"]
    pub stat: STAT,
    #[doc = "0x14 - Interrupt Clear Register"]
    pub eoi: EOI,
    _reserved6: [u8; 4usize],
    #[doc = "0x1c - Protection level Register"]
    pub prot_level: PROT_LEVEL,
    _reserved7: [u8; 196usize],
    #[doc = "0xe4 - Component Parameters Register 5"]
    pub comp_param_5: COMP_PARAM_5,
    #[doc = "0xe8 - Component Parameters Register 4"]
    pub comp_param_4: COMP_PARAM_4,
    #[doc = "0xec - Component Parameters Register 3"]
    pub comp_param_3: COMP_PARAM_3,
    #[doc = "0xf0 - Component Parameters Register 2"]
    pub comp_param_2: COMP_PARAM_2,
    #[doc = "0xf4 - Component Parameters Register 1"]
    pub comp_param_1: COMP_PARAM_1,
    #[doc = "0xf8 - Component Version Register"]
    pub comp_version: COMP_VERSION,
    #[doc = "0xfc - Component Type Register"]
    pub comp_type: COMP_TYPE,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Timeout Range Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [torr](torr) module"]
pub type TORR = crate::Reg<u32, _TORR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TORR;
#[doc = "`read()` method returns [torr::R](torr::R) reader structure"]
impl crate::Readable for TORR {}
#[doc = "`write(|w| ..)` method takes [torr::W](torr::W) writer structure"]
impl crate::Writable for TORR {}
#[doc = "Timeout Range Register"]
pub mod torr;
#[doc = "Current Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ccvr](ccvr) module"]
pub type CCVR = crate::Reg<u32, _CCVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCVR;
#[doc = "`read()` method returns [ccvr::R](ccvr::R) reader structure"]
impl crate::Readable for CCVR {}
#[doc = "`write(|w| ..)` method takes [ccvr::W](ccvr::W) writer structure"]
impl crate::Writable for CCVR {}
#[doc = "Current Counter Value Register"]
pub mod ccvr;
#[doc = "Counter Restart Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crr](crr) module"]
pub type CRR = crate::Reg<u32, _CRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRR;
#[doc = "`read()` method returns [crr::R](crr::R) reader structure"]
impl crate::Readable for CRR {}
#[doc = "`write(|w| ..)` method takes [crr::W](crr::W) writer structure"]
impl crate::Writable for CRR {}
#[doc = "Counter Restart Register"]
pub mod crr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "Interrupt Status Register"]
pub mod stat;
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
#[doc = "Protection level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prot_level](prot_level) module"]
pub type PROT_LEVEL = crate::Reg<u32, _PROT_LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROT_LEVEL;
#[doc = "`read()` method returns [prot_level::R](prot_level::R) reader structure"]
impl crate::Readable for PROT_LEVEL {}
#[doc = "`write(|w| ..)` method takes [prot_level::W](prot_level::W) writer structure"]
impl crate::Writable for PROT_LEVEL {}
#[doc = "Protection level Register"]
pub mod prot_level;
#[doc = "Component Parameters Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_param_5](comp_param_5) module"]
pub type COMP_PARAM_5 = crate::Reg<u32, _COMP_PARAM_5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_PARAM_5;
#[doc = "`read()` method returns [comp_param_5::R](comp_param_5::R) reader structure"]
impl crate::Readable for COMP_PARAM_5 {}
#[doc = "`write(|w| ..)` method takes [comp_param_5::W](comp_param_5::W) writer structure"]
impl crate::Writable for COMP_PARAM_5 {}
#[doc = "Component Parameters Register 5"]
pub mod comp_param_5;
#[doc = "Component Parameters Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_param_4](comp_param_4) module"]
pub type COMP_PARAM_4 = crate::Reg<u32, _COMP_PARAM_4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_PARAM_4;
#[doc = "`read()` method returns [comp_param_4::R](comp_param_4::R) reader structure"]
impl crate::Readable for COMP_PARAM_4 {}
#[doc = "`write(|w| ..)` method takes [comp_param_4::W](comp_param_4::W) writer structure"]
impl crate::Writable for COMP_PARAM_4 {}
#[doc = "Component Parameters Register 4"]
pub mod comp_param_4;
#[doc = "Component Parameters Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_param_3](comp_param_3) module"]
pub type COMP_PARAM_3 = crate::Reg<u32, _COMP_PARAM_3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_PARAM_3;
#[doc = "`read()` method returns [comp_param_3::R](comp_param_3::R) reader structure"]
impl crate::Readable for COMP_PARAM_3 {}
#[doc = "`write(|w| ..)` method takes [comp_param_3::W](comp_param_3::W) writer structure"]
impl crate::Writable for COMP_PARAM_3 {}
#[doc = "Component Parameters Register 3"]
pub mod comp_param_3;
#[doc = "Component Parameters Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_param_2](comp_param_2) module"]
pub type COMP_PARAM_2 = crate::Reg<u32, _COMP_PARAM_2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_PARAM_2;
#[doc = "`read()` method returns [comp_param_2::R](comp_param_2::R) reader structure"]
impl crate::Readable for COMP_PARAM_2 {}
#[doc = "`write(|w| ..)` method takes [comp_param_2::W](comp_param_2::W) writer structure"]
impl crate::Writable for COMP_PARAM_2 {}
#[doc = "Component Parameters Register 2"]
pub mod comp_param_2;
#[doc = "Component Parameters Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_param_1](comp_param_1) module"]
pub type COMP_PARAM_1 = crate::Reg<u32, _COMP_PARAM_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_PARAM_1;
#[doc = "`read()` method returns [comp_param_1::R](comp_param_1::R) reader structure"]
impl crate::Readable for COMP_PARAM_1 {}
#[doc = "`write(|w| ..)` method takes [comp_param_1::W](comp_param_1::W) writer structure"]
impl crate::Writable for COMP_PARAM_1 {}
#[doc = "Component Parameters Register 1"]
pub mod comp_param_1;
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
#[doc = "Component Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [comp_type](comp_type) module"]
pub type COMP_TYPE = crate::Reg<u32, _COMP_TYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMP_TYPE;
#[doc = "`read()` method returns [comp_type::R](comp_type::R) reader structure"]
impl crate::Readable for COMP_TYPE {}
#[doc = "`write(|w| ..)` method takes [comp_type::W](comp_type::W) writer structure"]
impl crate::Writable for COMP_TYPE {}
#[doc = "Component Type Register"]
pub mod comp_type;
