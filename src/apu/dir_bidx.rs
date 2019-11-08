#[doc = "Reader of register dir_bidx[%s]"]
pub type R = crate::R<u32, super::DIR_BIDX>;
#[doc = "Writer for register dir_bidx[%s]"]
pub type W = crate::W<u32, super::DIR_BIDX>;
#[doc = "Register dir_bidx[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::DIR_BIDX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rd_idx0`"]
pub type RD_IDX0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rd_idx0`"]
pub struct RD_IDX0_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_IDX0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `rd_idx1`"]
pub type RD_IDX1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rd_idx1`"]
pub struct RD_IDX1_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_IDX1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `rd_idx2`"]
pub type RD_IDX2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rd_idx2`"]
pub struct RD_IDX2_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_IDX2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `rd_idx3`"]
pub type RD_IDX3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rd_idx3`"]
pub struct RD_IDX3_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_IDX3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - rd_idx0"]
    #[inline(always)]
    pub fn rd_idx0(&self) -> RD_IDX0_R {
        RD_IDX0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - rd_idx1"]
    #[inline(always)]
    pub fn rd_idx1(&self) -> RD_IDX1_R {
        RD_IDX1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - rd_idx2"]
    #[inline(always)]
    pub fn rd_idx2(&self) -> RD_IDX2_R {
        RD_IDX2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - rd_idx3"]
    #[inline(always)]
    pub fn rd_idx3(&self) -> RD_IDX3_R {
        RD_IDX3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - rd_idx0"]
    #[inline(always)]
    pub fn rd_idx0(&mut self) -> RD_IDX0_W {
        RD_IDX0_W { w: self }
    }
    #[doc = "Bits 8:13 - rd_idx1"]
    #[inline(always)]
    pub fn rd_idx1(&mut self) -> RD_IDX1_W {
        RD_IDX1_W { w: self }
    }
    #[doc = "Bits 16:21 - rd_idx2"]
    #[inline(always)]
    pub fn rd_idx2(&mut self) -> RD_IDX2_W {
        RD_IDX2_W { w: self }
    }
    #[doc = "Bits 24:29 - rd_idx3"]
    #[inline(always)]
    pub fn rd_idx3(&mut self) -> RD_IDX3_W {
        RD_IDX3_W { w: self }
    }
}
