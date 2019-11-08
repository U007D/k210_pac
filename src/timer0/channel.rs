#[doc = "Load Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [load_count](load_count) module"]
pub type LOAD_COUNT = crate::Reg<u32, _LOAD_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOAD_COUNT;
#[doc = "`read()` method returns [load_count::R](load_count::R) reader structure"]
impl crate::Readable for LOAD_COUNT {}
#[doc = "`write(|w| ..)` method takes [load_count::W](load_count::W) writer structure"]
impl crate::Writable for LOAD_COUNT {}
#[doc = "Load Count Register"]
pub mod load_count;
#[doc = "Current Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [current_value](current_value) module"]
pub type CURRENT_VALUE = crate::Reg<u32, _CURRENT_VALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CURRENT_VALUE;
#[doc = "`read()` method returns [current_value::R](current_value::R) reader structure"]
impl crate::Readable for CURRENT_VALUE {}
#[doc = "`write(|w| ..)` method takes [current_value::W](current_value::W) writer structure"]
impl crate::Writable for CURRENT_VALUE {}
#[doc = "Current Value Register"]
pub mod current_value;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "Control Register"]
pub mod control;
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
