#[doc = "Reader of register fifo_threshold"]
pub type R = crate::R<u64, super::FIFO_THRESHOLD>;
#[doc = "Writer for register fifo_threshold"]
pub type W = crate::W<u64, super::FIFO_THRESHOLD>;
#[doc = "Register fifo_threshold `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO_THRESHOLD {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `full_threshold`"]
pub type FULL_THRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `full_threshold`"]
pub struct FULL_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u64) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `empty_threshold`"]
pub type EMPTY_THRESHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `empty_threshold`"]
pub struct EMPTY_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> EMPTY_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u64) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - FIFO full threshold"]
    #[inline(always)]
    pub fn full_threshold(&self) -> FULL_THRESHOLD_R {
        FULL_THRESHOLD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - FIFO empty threshold"]
    #[inline(always)]
    pub fn empty_threshold(&self) -> EMPTY_THRESHOLD_R {
        EMPTY_THRESHOLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - FIFO full threshold"]
    #[inline(always)]
    pub fn full_threshold(&mut self) -> FULL_THRESHOLD_W {
        FULL_THRESHOLD_W { w: self }
    }
    #[doc = "Bits 4:7 - FIFO empty threshold"]
    #[inline(always)]
    pub fn empty_threshold(&mut self) -> EMPTY_THRESHOLD_W {
        EMPTY_THRESHOLD_W { w: self }
    }
}
