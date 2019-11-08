#[doc = "Reader of register ss_scl_lcnt"]
pub type R = crate::R<u32, super::SS_SCL_LCNT>;
#[doc = "Writer for register ss_scl_lcnt"]
pub type W = crate::W<u32, super::SS_SCL_LCNT>;
#[doc = "Register ss_scl_lcnt `reset()`'s with value 0"]
impl crate::ResetValue for super::SS_SCL_LCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `count`"]
pub type COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `count`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - COUNT"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - COUNT"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
}
