#[doc = "Reader of register sat_limits"]
pub type R = crate::R<u32, super::SAT_LIMITS>;
#[doc = "Writer for register sat_limits"]
pub type W = crate::W<u32, super::SAT_LIMITS>;
#[doc = "Register sat_limits `reset()`'s with value 0"]
impl crate::ResetValue for super::SAT_LIMITS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `upper`"]
pub type UPPER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `upper`"]
pub struct UPPER_W<'a> {
    w: &'a mut W,
}
impl<'a> UPPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `bottom`"]
pub type BOTTOM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `bottom`"]
pub struct BOTTOM_W<'a> {
    w: &'a mut W,
}
impl<'a> BOTTOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Upper limit"]
    #[inline(always)]
    pub fn upper(&self) -> UPPER_R {
        UPPER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Bottom limit"]
    #[inline(always)]
    pub fn bottom(&self) -> BOTTOM_R {
        BOTTOM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Upper limit"]
    #[inline(always)]
    pub fn upper(&mut self) -> UPPER_W {
        UPPER_W { w: self }
    }
    #[doc = "Bits 16:31 - Bottom limit"]
    #[inline(always)]
    pub fn bottom(&mut self) -> BOTTOM_W {
        BOTTOM_W { w: self }
    }
}
