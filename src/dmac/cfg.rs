#[doc = "Reader of register cfg"]
pub type R = crate::R<u64, super::CFG>;
#[doc = "Writer for register cfg"]
pub type W = crate::W<u64, super::CFG>;
#[doc = "Register cfg `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dmac_en`"]
pub type DMAC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dmac_en`"]
pub struct DMAC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAC_EN_W<'a> {
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
#[doc = "Reader of field `int_en`"]
pub type INT_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `int_en`"]
pub struct INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u64) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable DMAC"]
    #[inline(always)]
    pub fn dmac_en(&self) -> DMAC_EN_R {
        DMAC_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Globally enable interrupt generation"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMAC"]
    #[inline(always)]
    pub fn dmac_en(&mut self) -> DMAC_EN_W {
        DMAC_EN_W { w: self }
    }
    #[doc = "Bit 1 - Globally enable interrupt generation"]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W {
        INT_EN_W { w: self }
    }
}
