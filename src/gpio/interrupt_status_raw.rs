#[doc = "Reader of register interrupt_status_raw"]
pub type R = crate::R<u32, super::INTERRUPT_STATUS_RAW>;
#[doc = "Writer for register interrupt_status_raw"]
pub type W = crate::W<u32, super::INTERRUPT_STATUS_RAW>;
#[doc = "Register interrupt_status_raw `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_STATUS_RAW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
