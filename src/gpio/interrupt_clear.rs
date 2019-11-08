#[doc = "Reader of register interrupt_clear"]
pub type R = crate::R<u32, super::INTERRUPT_CLEAR>;
#[doc = "Writer for register interrupt_clear"]
pub type W = crate::W<u32, super::INTERRUPT_CLEAR>;
#[doc = "Register interrupt_clear `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_CLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
