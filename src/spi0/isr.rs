#[doc = "Reader of register isr"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Writer for register isr"]
pub type W = crate::W<u32, super::ISR>;
#[doc = "Register isr `reset()`'s with value 0"]
impl crate::ResetValue for super::ISR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
