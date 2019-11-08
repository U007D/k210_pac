#[doc = "Reader of register function_reg_1"]
pub type R = crate::R<u32, super::FUNCTION_REG_1>;
#[doc = "Writer for register function_reg_1"]
pub type W = crate::W<u32, super::FUNCTION_REG_1>;
#[doc = "Register function_reg_1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FUNCTION_REG_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dma_en`"]
pub type DMA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dma_en`"]
pub struct DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `fifo_in_full`"]
pub type FIFO_IN_FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fifo_in_full`"]
pub struct FIFO_IN_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_IN_FULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SHA and DMA handshake signals enable. 1:enable; 0:disable"]
    #[inline(always)]
    pub fn dma_en(&self) -> DMA_EN_R {
        DMA_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1:SHA256 input fifo is full; 0:not full"]
    #[inline(always)]
    pub fn fifo_in_full(&self) -> FIFO_IN_FULL_R {
        FIFO_IN_FULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHA and DMA handshake signals enable. 1:enable; 0:disable"]
    #[inline(always)]
    pub fn dma_en(&mut self) -> DMA_EN_W {
        DMA_EN_W { w: self }
    }
    #[doc = "Bit 8 - 1:SHA256 input fifo is full; 0:not full"]
    #[inline(always)]
    pub fn fifo_in_full(&mut self) -> FIFO_IN_FULL_W {
        FIFO_IN_FULL_W { w: self }
    }
}
