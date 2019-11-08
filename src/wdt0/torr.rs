#[doc = "Reader of register torr"]
pub type R = crate::R<u32, super::TORR>;
#[doc = "Writer for register torr"]
pub type W = crate::W<u32, super::TORR>;
#[doc = "Register torr `reset()`'s with value 0"]
impl crate::ResetValue for super::TORR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `top0`"]
pub type TOP0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `top0`"]
pub struct TOP0_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `top1`"]
pub type TOP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `top1`"]
pub struct TOP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - top (lower half)"]
    #[inline(always)]
    pub fn top0(&self) -> TOP0_R {
        TOP0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - top (upper half)"]
    #[inline(always)]
    pub fn top1(&self) -> TOP1_R {
        TOP1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - top (lower half)"]
    #[inline(always)]
    pub fn top0(&mut self) -> TOP0_W {
        TOP0_W { w: self }
    }
    #[doc = "Bits 4:7 - top (upper half)"]
    #[inline(always)]
    pub fn top1(&mut self) -> TOP1_W {
        TOP1_W { w: self }
    }
}
