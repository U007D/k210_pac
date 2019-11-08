#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Layer arguments FIFO: each layer is defined by writing 12 successive argument values to this register"]
    pub layer_argument_fifo: LAYER_ARGUMENT_FIFO,
    #[doc = "0x08 - Interrupt status"]
    pub interrupt_status: INTERRUPT_STATUS,
    #[doc = "0x10 - Interrupt raw"]
    pub interrupt_raw: INTERRUPT_RAW,
    #[doc = "0x18 - Interrupt mask: 0 enables the interrupt, 1 masks the interrupt"]
    pub interrupt_mask: INTERRUPT_MASK,
    #[doc = "0x20 - Interrupt clear: write 1 to a bit to clear interrupt"]
    pub interrupt_clear: INTERRUPT_CLEAR,
    #[doc = "0x28 - FIFO threshold"]
    pub fifo_threshold: FIFO_THRESHOLD,
    #[doc = "0x30 - FIFO data output"]
    pub fifo_data_out: FIFO_DATA_OUT,
    #[doc = "0x38 - FIFO control"]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x40 - Eight bit mode"]
    pub eight_bit_mode: EIGHT_BIT_MODE,
}
#[doc = "Layer arguments FIFO: each layer is defined by writing 12 successive argument values to this register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [layer_argument_fifo](layer_argument_fifo) module"]
pub type LAYER_ARGUMENT_FIFO = crate::Reg<u64, _LAYER_ARGUMENT_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LAYER_ARGUMENT_FIFO;
#[doc = "`read()` method returns [layer_argument_fifo::R](layer_argument_fifo::R) reader structure"]
impl crate::Readable for LAYER_ARGUMENT_FIFO {}
#[doc = "`write(|w| ..)` method takes [layer_argument_fifo::W](layer_argument_fifo::W) writer structure"]
impl crate::Writable for LAYER_ARGUMENT_FIFO {}
#[doc = "Layer arguments FIFO: each layer is defined by writing 12 successive argument values to this register"]
pub mod layer_argument_fifo;
#[doc = "Interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_status](interrupt_status) module"]
pub type INTERRUPT_STATUS = crate::Reg<u64, _INTERRUPT_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_STATUS;
#[doc = "`read()` method returns [interrupt_status::R](interrupt_status::R) reader structure"]
impl crate::Readable for INTERRUPT_STATUS {}
#[doc = "`write(|w| ..)` method takes [interrupt_status::W](interrupt_status::W) writer structure"]
impl crate::Writable for INTERRUPT_STATUS {}
#[doc = "Interrupt status"]
pub mod interrupt_status;
#[doc = "Interrupt raw\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_raw](interrupt_raw) module"]
pub type INTERRUPT_RAW = crate::Reg<u64, _INTERRUPT_RAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_RAW;
#[doc = "`read()` method returns [interrupt_raw::R](interrupt_raw::R) reader structure"]
impl crate::Readable for INTERRUPT_RAW {}
#[doc = "`write(|w| ..)` method takes [interrupt_raw::W](interrupt_raw::W) writer structure"]
impl crate::Writable for INTERRUPT_RAW {}
#[doc = "Interrupt raw"]
pub mod interrupt_raw;
#[doc = "Interrupt mask: 0 enables the interrupt, 1 masks the interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_mask](interrupt_mask) module"]
pub type INTERRUPT_MASK = crate::Reg<u64, _INTERRUPT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_MASK;
#[doc = "`read()` method returns [interrupt_mask::R](interrupt_mask::R) reader structure"]
impl crate::Readable for INTERRUPT_MASK {}
#[doc = "`write(|w| ..)` method takes [interrupt_mask::W](interrupt_mask::W) writer structure"]
impl crate::Writable for INTERRUPT_MASK {}
#[doc = "Interrupt mask: 0 enables the interrupt, 1 masks the interrupt"]
pub mod interrupt_mask;
#[doc = "Interrupt clear: write 1 to a bit to clear interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [interrupt_clear](interrupt_clear) module"]
pub type INTERRUPT_CLEAR = crate::Reg<u64, _INTERRUPT_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRUPT_CLEAR;
#[doc = "`read()` method returns [interrupt_clear::R](interrupt_clear::R) reader structure"]
impl crate::Readable for INTERRUPT_CLEAR {}
#[doc = "`write(|w| ..)` method takes [interrupt_clear::W](interrupt_clear::W) writer structure"]
impl crate::Writable for INTERRUPT_CLEAR {}
#[doc = "Interrupt clear: write 1 to a bit to clear interrupt"]
pub mod interrupt_clear;
#[doc = "FIFO threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifo_threshold](fifo_threshold) module"]
pub type FIFO_THRESHOLD = crate::Reg<u64, _FIFO_THRESHOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_THRESHOLD;
#[doc = "`read()` method returns [fifo_threshold::R](fifo_threshold::R) reader structure"]
impl crate::Readable for FIFO_THRESHOLD {}
#[doc = "`write(|w| ..)` method takes [fifo_threshold::W](fifo_threshold::W) writer structure"]
impl crate::Writable for FIFO_THRESHOLD {}
#[doc = "FIFO threshold"]
pub mod fifo_threshold;
#[doc = "FIFO data output\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifo_data_out](fifo_data_out) module"]
pub type FIFO_DATA_OUT = crate::Reg<u64, _FIFO_DATA_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_DATA_OUT;
#[doc = "`read()` method returns [fifo_data_out::R](fifo_data_out::R) reader structure"]
impl crate::Readable for FIFO_DATA_OUT {}
#[doc = "`write(|w| ..)` method takes [fifo_data_out::W](fifo_data_out::W) writer structure"]
impl crate::Writable for FIFO_DATA_OUT {}
#[doc = "FIFO data output"]
pub mod fifo_data_out;
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
#[doc = "Eight bit mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eight_bit_mode](eight_bit_mode) module"]
pub type EIGHT_BIT_MODE = crate::Reg<u64, _EIGHT_BIT_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EIGHT_BIT_MODE;
#[doc = "`read()` method returns [eight_bit_mode::R](eight_bit_mode::R) reader structure"]
impl crate::Readable for EIGHT_BIT_MODE {}
#[doc = "`write(|w| ..)` method takes [eight_bit_mode::W](eight_bit_mode::W) writer structure"]
impl crate::Writable for EIGHT_BIT_MODE {}
#[doc = "Eight bit mode"]
pub mod eight_bit_mode;
