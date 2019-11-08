#[doc = "Reader of register interrupt_mask"]
pub type R = crate::R<u64, super::INTERRUPT_MASK>;
#[doc = "Writer for register interrupt_mask"]
pub type W = crate::W<u64, super::INTERRUPT_MASK>;
#[doc = "Register interrupt_mask `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_MASK {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `calc_done`"]
pub type CALC_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `calc_done`"]
pub struct CALC_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALC_DONE_W<'a> {
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
#[doc = "Reader of field `layer_cfg_almost_empty`"]
pub type LAYER_CFG_ALMOST_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `layer_cfg_almost_empty`"]
pub struct LAYER_CFG_ALMOST_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> LAYER_CFG_ALMOST_EMPTY_W<'a> {
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
#[doc = "Reader of field `layer_cfg_almost_full`"]
pub type LAYER_CFG_ALMOST_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `layer_cfg_almost_full`"]
pub struct LAYER_CFG_ALMOST_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> LAYER_CFG_ALMOST_FULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u64) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt raised when calculation is done"]
    #[inline(always)]
    pub fn calc_done(&self) -> CALC_DONE_R {
        CALC_DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt raised when layer arguments FIFO almost empty"]
    #[inline(always)]
    pub fn layer_cfg_almost_empty(&self) -> LAYER_CFG_ALMOST_EMPTY_R {
        LAYER_CFG_ALMOST_EMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt raised when layer arguments FIFO almost full"]
    #[inline(always)]
    pub fn layer_cfg_almost_full(&self) -> LAYER_CFG_ALMOST_FULL_R {
        LAYER_CFG_ALMOST_FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt raised when calculation is done"]
    #[inline(always)]
    pub fn calc_done(&mut self) -> CALC_DONE_W {
        CALC_DONE_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt raised when layer arguments FIFO almost empty"]
    #[inline(always)]
    pub fn layer_cfg_almost_empty(&mut self) -> LAYER_CFG_ALMOST_EMPTY_W {
        LAYER_CFG_ALMOST_EMPTY_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt raised when layer arguments FIFO almost full"]
    #[inline(always)]
    pub fn layer_cfg_almost_full(&mut self) -> LAYER_CFG_ALMOST_FULL_W {
        LAYER_CFG_ALMOST_FULL_W { w: self }
    }
}
