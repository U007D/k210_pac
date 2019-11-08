#[doc = "Reader of register interrupt_debounce"]
pub type R = crate::R<u32, super::INTERRUPT_DEBOUNCE>;
#[doc = "Writer for register interrupt_debounce"]
pub type W = crate::W<u32, super::INTERRUPT_DEBOUNCE>;
#[doc = "Register interrupt_debounce `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_DEBOUNCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
