#[doc = "Reader of register dma_sel"]
pub type R = crate::R<u32, super::DMA_SEL>;
#[doc = "Writer for register dma_sel"]
pub type W = crate::W<u32, super::DMA_SEL>;
#[doc = "Register dma_sel `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dma_sel`"]
pub type DMA_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dma_sel`"]
pub struct DMA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Output to DMA if set, to CPU otherwise"]
    #[inline(always)]
    pub fn dma_sel(&self) -> DMA_SEL_R {
        DMA_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output to DMA if set, to CPU otherwise"]
    #[inline(always)]
    pub fn dma_sel(&mut self) -> DMA_SEL_W {
        DMA_SEL_W { w: self }
    }
}
