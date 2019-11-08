#[doc = "Reader of register blk_tfr"]
pub type R = crate::R<u64, super::BLK_TFR>;
#[doc = "Writer for register blk_tfr"]
pub type W = crate::W<u64, super::BLK_TFR>;
#[doc = "Register blk_tfr `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK_TFR {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `resumereq`"]
pub type RESUMEREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `resumereq`"]
pub struct RESUMEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEREQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u64) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Block transfer resume request"]
    #[inline(always)]
    pub fn resumereq(&self) -> RESUMEREQ_R {
        RESUMEREQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Block transfer resume request"]
    #[inline(always)]
    pub fn resumereq(&mut self) -> RESUMEREQ_W {
        RESUMEREQ_W { w: self }
    }
}
