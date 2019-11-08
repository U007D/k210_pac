#[doc = "Reader of register interrupt_enable"]
pub type R = crate::R<u32, super::INTERRUPT_ENABLE>;
#[doc = "Writer for register interrupt_enable"]
pub type W = crate::W<u32, super::INTERRUPT_ENABLE>;
#[doc = "Register interrupt_enable `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
