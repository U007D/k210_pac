#[doc = "Reader of register status"]
pub type R = crate::R<u64, super::STATUS>;
#[doc = "Writer for register status"]
pub type W = crate::W<u64, super::STATUS>;
#[doc = "Register status `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cmpltd_blk_size`"]
pub type CMPLTD_BLK_SIZE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `cmpltd_blk_size`"]
pub struct CMPLTD_BLK_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPLTD_BLK_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x003f_ffff) | ((value as u64) & 0x003f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:21 - Completed block transfer size"]
    #[inline(always)]
    pub fn cmpltd_blk_size(&self) -> CMPLTD_BLK_SIZE_R {
        CMPLTD_BLK_SIZE_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Completed block transfer size"]
    #[inline(always)]
    pub fn cmpltd_blk_size(&mut self) -> CMPLTD_BLK_SIZE_W {
        CMPLTD_BLK_SIZE_W { w: self }
    }
}
