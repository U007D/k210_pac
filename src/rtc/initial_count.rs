#[doc = "Reader of register initial_count"]
pub type R = crate::R<u32, super::INITIAL_COUNT>;
#[doc = "Writer for register initial_count"]
pub type W = crate::W<u32, super::INITIAL_COUNT>;
#[doc = "Register initial_count `reset()`'s with value 0"]
impl crate::ResetValue for super::INITIAL_COUNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `count`"]
pub type COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `count`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RTC counter initial value"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTC counter initial value"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
}
