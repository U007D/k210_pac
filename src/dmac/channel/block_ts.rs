#[doc = "Reader of register block_ts"]
pub type R = crate::R<u64, super::BLOCK_TS>;
#[doc = "Writer for register block_ts"]
pub type W = crate::W<u64, super::BLOCK_TS>;
#[doc = "Register block_ts `reset()`'s with value 0"]
impl crate::ResetValue for super::BLOCK_TS {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `block_ts`"]
pub type BLOCK_TS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `block_ts`"]
pub struct BLOCK_TS_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCK_TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u64) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Block transfer size"]
    #[inline(always)]
    pub fn block_ts(&self) -> BLOCK_TS_R {
        BLOCK_TS_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Block transfer size"]
    #[inline(always)]
    pub fn block_ts(&mut self) -> BLOCK_TS_W {
        BLOCK_TS_W { w: self }
    }
}
