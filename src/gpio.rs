#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data (output) registers"]
    pub data_output: DATA_OUTPUT,
    #[doc = "0x04 - Data direction registers"]
    pub direction: DIRECTION,
    #[doc = "0x08 - Data source registers"]
    pub source: SOURCE,
    _reserved3: [u8; 36usize],
    #[doc = "0x30 - Interrupt enable/disable registers"]
    pub interrupt_enable: INTERRUPT_ENABLE,
    #[doc = "0x34 - Interrupt mask registers"]
    pub interrupt_mask: INTERRUPT_MASK,
    #[doc = "0x38 - Interrupt level registers"]
    pub interrupt_level: INTERRUPT_LEVEL,
    #[doc = "0x3c - Interrupt polarity registers"]
    pub interrupt_polarity: INTERRUPT_POLARITY,
    #[doc = "0x40 - Interrupt status registers"]
    pub interrupt_status: INTERRUPT_STATUS,
    #[doc = "0x44 - Raw interrupt status registers"]
    pub interrupt_status_raw: INTERRUPT_STATUS_RAW,
    #[doc = "0x48 - Interrupt debounce registers"]
    pub interrupt_debounce: INTERRUPT_DEBOUNCE,
    #[doc = "0x4c - Registers for clearing interrupts"]
    pub interrupt_clear: INTERRUPT_CLEAR,
    #[doc = "0x50 - External port (data input) registers"]
    pub data_input: DATA_INPUT,
    _reserved12: [u8; 12usize],
    #[doc = "0x60 - Sync level registers"]
    pub sync_level: SYNC_LEVEL,
    #[doc = "0x64 - ID code"]
    pub id_code: ID_CODE,
    #[doc = "0x68 - Interrupt both edge type"]
    pub interrupt_bothedge: INTERRUPT_BOTHEDGE,
}
#[doc = "Data (output) registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_output](data_output) module"]
pub type DATA_OUTPUT = crate::Reg<u32, _DATA_OUTPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_OUTPUT;
#[doc = "`read()` method returns [data_output::R](data_output::R) reader structure"]
impl crate::Readable for DATA_OUTPUT {}
#[doc = "`write(|w| ..)` method takes [data_output::W](data_output::W) writer structure"]
impl crate::Writable for DATA_OUTPUT {}
#[doc = "Data (output) registers"]
pub mod data_output;
#[doc = "Data direction registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [direction](direction) module"]
pub type DIRECTION = crate::Reg<u32, _DIRECTION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIRECTION;
#[doc = "`read()` method returns [direction::R](direction::R) reader structure"]
impl crate::Readable for DIRECTION {}
#[doc = "`write(|w| ..)` method takes [direction::W](direction::W) writer structure"]
impl crate::Writable for DIRECTION {}
#[doc = "Data direction registers"]
pub mod direction;
#[doc = "Data source registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [source](source) module"]
pub type SOURCE = crate::Reg<u32, _SOURCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOURCE;
#[doc = "`read()` method returns [source::R](source::R) reader structure"]
impl crate::Readable for SOURCE {}
#[doc = "`write(|w| ..)` method takes [source::W](source::W) writer structure"]
impl crate::Writable for SOURCE {}
#[doc = "Data source registers"]
pub mod source;
#[doc = "Interrupt enable/disable registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_enable](interrupt_enable) module"]
pub type INTERRUPT_ENABLE = crate::Reg<u32, _INTERRUPT_ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_ENABLE;
#[doc = "`read()` method returns [interrupt_enable::R](interrupt_enable::R) reader structure"]
impl crate::Readable for INTERRUPT_ENABLE {}
#[doc = "`write(|w| ..)` method takes [interrupt_enable::W](interrupt_enable::W) writer structure"]
impl crate::Writable for INTERRUPT_ENABLE {}
#[doc = "Interrupt enable/disable registers"]
pub mod interrupt_enable;
#[doc = "Interrupt mask registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_mask](interrupt_mask) module"]
pub type INTERRUPT_MASK = crate::Reg<u32, _INTERRUPT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_MASK;
#[doc = "`read()` method returns [interrupt_mask::R](interrupt_mask::R) reader structure"]
impl crate::Readable for INTERRUPT_MASK {}
#[doc = "`write(|w| ..)` method takes [interrupt_mask::W](interrupt_mask::W) writer structure"]
impl crate::Writable for INTERRUPT_MASK {}
#[doc = "Interrupt mask registers"]
pub mod interrupt_mask;
#[doc = "Interrupt level registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_level](interrupt_level) module"]
pub type INTERRUPT_LEVEL = crate::Reg<u32, _INTERRUPT_LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_LEVEL;
#[doc = "`read()` method returns [interrupt_level::R](interrupt_level::R) reader structure"]
impl crate::Readable for INTERRUPT_LEVEL {}
#[doc = "`write(|w| ..)` method takes [interrupt_level::W](interrupt_level::W) writer structure"]
impl crate::Writable for INTERRUPT_LEVEL {}
#[doc = "Interrupt level registers"]
pub mod interrupt_level;
#[doc = "Interrupt polarity registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_polarity](interrupt_polarity) module"]
pub type INTERRUPT_POLARITY = crate::Reg<u32, _INTERRUPT_POLARITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_POLARITY;
#[doc = "`read()` method returns [interrupt_polarity::R](interrupt_polarity::R) reader structure"]
impl crate::Readable for INTERRUPT_POLARITY {}
#[doc = "`write(|w| ..)` method takes [interrupt_polarity::W](interrupt_polarity::W) writer structure"]
impl crate::Writable for INTERRUPT_POLARITY {}
#[doc = "Interrupt polarity registers"]
pub mod interrupt_polarity;
#[doc = "Interrupt status registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_status](interrupt_status) module"]
pub type INTERRUPT_STATUS = crate::Reg<u32, _INTERRUPT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_STATUS;
#[doc = "`read()` method returns [interrupt_status::R](interrupt_status::R) reader structure"]
impl crate::Readable for INTERRUPT_STATUS {}
#[doc = "`write(|w| ..)` method takes [interrupt_status::W](interrupt_status::W) writer structure"]
impl crate::Writable for INTERRUPT_STATUS {}
#[doc = "Interrupt status registers"]
pub mod interrupt_status;
#[doc = "Raw interrupt status registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_status_raw](interrupt_status_raw) module"]
pub type INTERRUPT_STATUS_RAW = crate::Reg<u32, _INTERRUPT_STATUS_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_STATUS_RAW;
#[doc = "`read()` method returns [interrupt_status_raw::R](interrupt_status_raw::R) reader structure"]
impl crate::Readable for INTERRUPT_STATUS_RAW {}
#[doc = "`write(|w| ..)` method takes [interrupt_status_raw::W](interrupt_status_raw::W) writer structure"]
impl crate::Writable for INTERRUPT_STATUS_RAW {}
#[doc = "Raw interrupt status registers"]
pub mod interrupt_status_raw;
#[doc = "Interrupt debounce registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_debounce](interrupt_debounce) module"]
pub type INTERRUPT_DEBOUNCE = crate::Reg<u32, _INTERRUPT_DEBOUNCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_DEBOUNCE;
#[doc = "`read()` method returns [interrupt_debounce::R](interrupt_debounce::R) reader structure"]
impl crate::Readable for INTERRUPT_DEBOUNCE {}
#[doc = "`write(|w| ..)` method takes [interrupt_debounce::W](interrupt_debounce::W) writer structure"]
impl crate::Writable for INTERRUPT_DEBOUNCE {}
#[doc = "Interrupt debounce registers"]
pub mod interrupt_debounce;
#[doc = "Registers for clearing interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_clear](interrupt_clear) module"]
pub type INTERRUPT_CLEAR = crate::Reg<u32, _INTERRUPT_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CLEAR;
#[doc = "`read()` method returns [interrupt_clear::R](interrupt_clear::R) reader structure"]
impl crate::Readable for INTERRUPT_CLEAR {}
#[doc = "`write(|w| ..)` method takes [interrupt_clear::W](interrupt_clear::W) writer structure"]
impl crate::Writable for INTERRUPT_CLEAR {}
#[doc = "Registers for clearing interrupts"]
pub mod interrupt_clear;
#[doc = "External port (data input) registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_input](data_input) module"]
pub type DATA_INPUT = crate::Reg<u32, _DATA_INPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_INPUT;
#[doc = "`read()` method returns [data_input::R](data_input::R) reader structure"]
impl crate::Readable for DATA_INPUT {}
#[doc = "`write(|w| ..)` method takes [data_input::W](data_input::W) writer structure"]
impl crate::Writable for DATA_INPUT {}
#[doc = "External port (data input) registers"]
pub mod data_input;
#[doc = "Sync level registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sync_level](sync_level) module"]
pub type SYNC_LEVEL = crate::Reg<u32, _SYNC_LEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC_LEVEL;
#[doc = "`read()` method returns [sync_level::R](sync_level::R) reader structure"]
impl crate::Readable for SYNC_LEVEL {}
#[doc = "`write(|w| ..)` method takes [sync_level::W](sync_level::W) writer structure"]
impl crate::Writable for SYNC_LEVEL {}
#[doc = "Sync level registers"]
pub mod sync_level;
#[doc = "ID code\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [id_code](id_code) module"]
pub type ID_CODE = crate::Reg<u32, _ID_CODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ID_CODE;
#[doc = "`read()` method returns [id_code::R](id_code::R) reader structure"]
impl crate::Readable for ID_CODE {}
#[doc = "`write(|w| ..)` method takes [id_code::W](id_code::W) writer structure"]
impl crate::Writable for ID_CODE {}
#[doc = "ID code"]
pub mod id_code;
#[doc = "Interrupt both edge type\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_bothedge](interrupt_bothedge) module"]
pub type INTERRUPT_BOTHEDGE = crate::Reg<u32, _INTERRUPT_BOTHEDGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_BOTHEDGE;
#[doc = "`read()` method returns [interrupt_bothedge::R](interrupt_bothedge::R) reader structure"]
impl crate::Readable for INTERRUPT_BOTHEDGE {}
#[doc = "`write(|w| ..)` method takes [interrupt_bothedge::W](interrupt_bothedge::W) writer structure"]
impl crate::Writable for INTERRUPT_BOTHEDGE {}
#[doc = "Interrupt both edge type"]
pub mod interrupt_bothedge;
