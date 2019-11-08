#[doc = "Reader of register interrupt_polarity"]
pub type R = crate::R<u32, super::INTERRUPT_POLARITY>;
#[doc = "Writer for register interrupt_polarity"]
pub type W = crate::W<u32, super::INTERRUPT_POLARITY>;
#[doc = "Register interrupt_polarity `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_POLARITY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
