#[doc = "Reader of register sat_counter"]
pub type R = crate::R<u32, super::SAT_COUNTER>;
#[doc = "Writer for register sat_counter"]
pub type W = crate::W<u32, super::SAT_COUNTER>;
#[doc = "Register sat_counter `reset()`'s with value 0"]
impl crate::ResetValue for super::SAT_COUNTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `counter`"]
pub type COUNTER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `counter`"]
pub struct COUNTER_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `total`"]
pub type TOTAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `total`"]
pub struct TOTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Total"]
    #[inline(always)]
    pub fn total(&self) -> TOTAL_R {
        TOTAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter"]
    #[inline(always)]
    pub fn counter(&mut self) -> COUNTER_W {
        COUNTER_W { w: self }
    }
    #[doc = "Bits 16:31 - Total"]
    #[inline(always)]
    pub fn total(&mut self) -> TOTAL_W {
        TOTAL_W { w: self }
    }
}
