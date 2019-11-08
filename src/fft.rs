#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FFT input data fifo"]
    pub input_fifo: INPUT_FIFO,
    #[doc = "0x08 - FFT control register"]
    pub ctrl: CTRL,
    #[doc = "0x10 - FIFO control"]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x18 - intr_mask"]
    pub interruptmask: INTERRUPTMASK,
    #[doc = "0x20 - Interrupt clear"]
    pub intr_clear: INTR_CLEAR,
    #[doc = "0x28 - FFT status register"]
    pub status: STATUS,
    #[doc = "0x30 - FFT status raw"]
    pub status_raw: STATUS_RAW,
    #[doc = "0x38 - FFT output FIFO"]
    pub output_fifo: OUTPUT_FIFO,
}
#[doc = "FFT input data fifo\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [input_fifo](input_fifo) module"]
pub type INPUT_FIFO = crate::Reg<u64, _INPUT_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPUT_FIFO;
#[doc = "`read()` method returns [input_fifo::R](input_fifo::R) reader structure"]
impl crate::Readable for INPUT_FIFO {}
#[doc = "`write(|w| ..)` method takes [input_fifo::W](input_fifo::W) writer structure"]
impl crate::Writable for INPUT_FIFO {}
#[doc = "FFT input data fifo"]
pub mod input_fifo;
#[doc = "FFT control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u64, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "FFT control register"]
pub mod ctrl;
#[doc = "FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifo_ctrl](fifo_ctrl) module"]
pub type FIFO_CTRL = crate::Reg<u64, _FIFO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_CTRL;
#[doc = "`read()` method returns [fifo_ctrl::R](fifo_ctrl::R) reader structure"]
impl crate::Readable for FIFO_CTRL {}
#[doc = "`write(|w| ..)` method takes [fifo_ctrl::W](fifo_ctrl::W) writer structure"]
impl crate::Writable for FIFO_CTRL {}
#[doc = "FIFO control"]
pub mod fifo_ctrl;
#[doc = "intr_mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interruptmask](interruptmask) module"]
pub type INTERRUPTMASK = crate::Reg<u64, _INTERRUPTMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPTMASK;
#[doc = "`read()` method returns [interruptmask::R](interruptmask::R) reader structure"]
impl crate::Readable for INTERRUPTMASK {}
#[doc = "`write(|w| ..)` method takes [interruptmask::W](interruptmask::W) writer structure"]
impl crate::Writable for INTERRUPTMASK {}
#[doc = "intr_mask"]
pub mod interruptmask;
#[doc = "Interrupt clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intr_clear](intr_clear) module"]
pub type INTR_CLEAR = crate::Reg<u64, _INTR_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTR_CLEAR;
#[doc = "`read()` method returns [intr_clear::R](intr_clear::R) reader structure"]
impl crate::Readable for INTR_CLEAR {}
#[doc = "`write(|w| ..)` method takes [intr_clear::W](intr_clear::W) writer structure"]
impl crate::Writable for INTR_CLEAR {}
#[doc = "Interrupt clear"]
pub mod intr_clear;
#[doc = "FFT status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u64, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "FFT status register"]
pub mod status;
#[doc = "FFT status raw\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status_raw](status_raw) module"]
pub type STATUS_RAW = crate::Reg<u64, _STATUS_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS_RAW;
#[doc = "`read()` method returns [status_raw::R](status_raw::R) reader structure"]
impl crate::Readable for STATUS_RAW {}
#[doc = "`write(|w| ..)` method takes [status_raw::W](status_raw::W) writer structure"]
impl crate::Writable for STATUS_RAW {}
#[doc = "FFT status raw"]
pub mod status_raw;
#[doc = "FFT output FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [output_fifo](output_fifo) module"]
pub type OUTPUT_FIFO = crate::Reg<u64, _OUTPUT_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPUT_FIFO;
#[doc = "`read()` method returns [output_fifo::R](output_fifo::R) reader structure"]
impl crate::Readable for OUTPUT_FIFO {}
#[doc = "`write(|w| ..)` method takes [output_fifo::W](output_fifo::W) writer structure"]
impl crate::Writable for OUTPUT_FIFO {}
#[doc = "FFT output FIFO"]
pub mod output_fifo;
