#[doc = "Reader of register num_reg"]
pub type R = crate::R<u32, super::NUM_REG>;
#[doc = "Writer for register num_reg"]
pub type W = crate::W<u32, super::NUM_REG>;
#[doc = "Register num_reg `reset()`'s with value 0"]
impl crate::ResetValue for super::NUM_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `data_cnt`"]
pub type DATA_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `data_cnt`"]
pub struct DATA_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `data_num`"]
pub type DATA_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `data_num`"]
pub struct DATA_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The total amount of data calculated by SHA256 is set by this register, and the smallest unit is 512bit"]
    #[inline(always)]
    pub fn data_cnt(&self) -> DATA_CNT_R {
        DATA_CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Currently calculated block number. 512bit=1block"]
    #[inline(always)]
    pub fn data_num(&self) -> DATA_NUM_R {
        DATA_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The total amount of data calculated by SHA256 is set by this register, and the smallest unit is 512bit"]
    #[inline(always)]
    pub fn data_cnt(&mut self) -> DATA_CNT_W {
        DATA_CNT_W { w: self }
    }
    #[doc = "Bits 16:31 - Currently calculated block number. 512bit=1block"]
    #[inline(always)]
    pub fn data_num(&mut self) -> DATA_NUM_W {
        DATA_NUM_W { w: self }
    }
}
