#[doc = "Reader of register intr_stat"]
pub type R = crate::R<u32, super::INTR_STAT>;
#[doc = "Writer for register intr_stat"]
pub type W = crate::W<u32, super::INTR_STAT>;
#[doc = "Register intr_stat `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
impl R {}
impl W {}
