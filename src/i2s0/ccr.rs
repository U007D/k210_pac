#[doc = "Reader of register ccr"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Writer for register ccr"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register ccr `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Gating of sclk\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_GATE_A {
    #[doc = "0: Clock gating is disabled"]
    NO,
    #[doc = "1: Gating after 12 sclk cycles"]
    CYCLES12,
    #[doc = "2: Gating after 16 sclk cycles"]
    CYCLES16,
    #[doc = "3: Gating after 20 sclk cycles"]
    CYCLES20,
    #[doc = "4: Gating after 24 sclk cycles"]
    CYCLES24,
}
impl From<CLK_GATE_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_GATE_A) -> Self {
        match variant {
            CLK_GATE_A::NO => 0,
            CLK_GATE_A::CYCLES12 => 1,
            CLK_GATE_A::CYCLES16 => 2,
            CLK_GATE_A::CYCLES20 => 3,
            CLK_GATE_A::CYCLES24 => 4,
        }
    }
}
#[doc = "Reader of field `clk_gate`"]
pub type CLK_GATE_R = crate::R<u8, CLK_GATE_A>;
impl CLK_GATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_GATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_GATE_A::NO),
            1 => Val(CLK_GATE_A::CYCLES12),
            2 => Val(CLK_GATE_A::CYCLES16),
            3 => Val(CLK_GATE_A::CYCLES20),
            4 => Val(CLK_GATE_A::CYCLES24),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CLK_GATE_A::NO
    }
    #[doc = "Checks if the value of the field is `CYCLES12`"]
    #[inline(always)]
    pub fn is_cycles12(&self) -> bool {
        *self == CLK_GATE_A::CYCLES12
    }
    #[doc = "Checks if the value of the field is `CYCLES16`"]
    #[inline(always)]
    pub fn is_cycles16(&self) -> bool {
        *self == CLK_GATE_A::CYCLES16
    }
    #[doc = "Checks if the value of the field is `CYCLES20`"]
    #[inline(always)]
    pub fn is_cycles20(&self) -> bool {
        *self == CLK_GATE_A::CYCLES20
    }
    #[doc = "Checks if the value of the field is `CYCLES24`"]
    #[inline(always)]
    pub fn is_cycles24(&self) -> bool {
        *self == CLK_GATE_A::CYCLES24
    }
}
#[doc = "Write proxy for field `clk_gate`"]
pub struct CLK_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_GATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_GATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock gating is disabled"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CLK_GATE_A::NO)
    }
    #[doc = "Gating after 12 sclk cycles"]
    #[inline(always)]
    pub fn cycles12(self) -> &'a mut W {
        self.variant(CLK_GATE_A::CYCLES12)
    }
    #[doc = "Gating after 16 sclk cycles"]
    #[inline(always)]
    pub fn cycles16(self) -> &'a mut W {
        self.variant(CLK_GATE_A::CYCLES16)
    }
    #[doc = "Gating after 20 sclk cycles"]
    #[inline(always)]
    pub fn cycles20(self) -> &'a mut W {
        self.variant(CLK_GATE_A::CYCLES20)
    }
    #[doc = "Gating after 24 sclk cycles"]
    #[inline(always)]
    pub fn cycles24(self) -> &'a mut W {
        self.variant(CLK_GATE_A::CYCLES24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "The number of sclk cycles for which the word select line stayd in the left aligned or right aligned mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_WORD_SIZE_A {
    #[doc = "0: 16 sclk cycles"]
    CYCLES16,
    #[doc = "1: 24 sclk cycles"]
    CYCLES24,
    #[doc = "2: 32 sclk cycles"]
    CYCLES32,
}
impl From<CLK_WORD_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_WORD_SIZE_A) -> Self {
        match variant {
            CLK_WORD_SIZE_A::CYCLES16 => 0,
            CLK_WORD_SIZE_A::CYCLES24 => 1,
            CLK_WORD_SIZE_A::CYCLES32 => 2,
        }
    }
}
#[doc = "Reader of field `clk_word_size`"]
pub type CLK_WORD_SIZE_R = crate::R<u8, CLK_WORD_SIZE_A>;
impl CLK_WORD_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLK_WORD_SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLK_WORD_SIZE_A::CYCLES16),
            1 => Val(CLK_WORD_SIZE_A::CYCLES24),
            2 => Val(CLK_WORD_SIZE_A::CYCLES32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES16`"]
    #[inline(always)]
    pub fn is_cycles16(&self) -> bool {
        *self == CLK_WORD_SIZE_A::CYCLES16
    }
    #[doc = "Checks if the value of the field is `CYCLES24`"]
    #[inline(always)]
    pub fn is_cycles24(&self) -> bool {
        *self == CLK_WORD_SIZE_A::CYCLES24
    }
    #[doc = "Checks if the value of the field is `CYCLES32`"]
    #[inline(always)]
    pub fn is_cycles32(&self) -> bool {
        *self == CLK_WORD_SIZE_A::CYCLES32
    }
}
#[doc = "Write proxy for field `clk_word_size`"]
pub struct CLK_WORD_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_WORD_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_WORD_SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16 sclk cycles"]
    #[inline(always)]
    pub fn cycles16(self) -> &'a mut W {
        self.variant(CLK_WORD_SIZE_A::CYCLES16)
    }
    #[doc = "24 sclk cycles"]
    #[inline(always)]
    pub fn cycles24(self) -> &'a mut W {
        self.variant(CLK_WORD_SIZE_A::CYCLES24)
    }
    #[doc = "32 sclk cycles"]
    #[inline(always)]
    pub fn cycles32(self) -> &'a mut W {
        self.variant(CLK_WORD_SIZE_A::CYCLES32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Alignment mode setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALIGN_MODE_A {
    #[doc = "1: Standard I2S format"]
    STANDARD,
    #[doc = "2: Right aligned format"]
    RIGHT,
    #[doc = "4: Left aligned format"]
    LEFT,
}
impl From<ALIGN_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ALIGN_MODE_A) -> Self {
        match variant {
            ALIGN_MODE_A::STANDARD => 1,
            ALIGN_MODE_A::RIGHT => 2,
            ALIGN_MODE_A::LEFT => 4,
        }
    }
}
#[doc = "Reader of field `align_mode`"]
pub type ALIGN_MODE_R = crate::R<u8, ALIGN_MODE_A>;
impl ALIGN_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALIGN_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(ALIGN_MODE_A::STANDARD),
            2 => Val(ALIGN_MODE_A::RIGHT),
            4 => Val(ALIGN_MODE_A::LEFT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == ALIGN_MODE_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_MODE_A::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_MODE_A::LEFT
    }
}
#[doc = "Write proxy for field `align_mode`"]
pub struct ALIGN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIGN_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALIGN_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard I2S format"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(ALIGN_MODE_A::STANDARD)
    }
    #[doc = "Right aligned format"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_MODE_A::RIGHT)
    }
    #[doc = "Left aligned format"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_MODE_A::LEFT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `dma_tx_en`"]
pub type DMA_TX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dma_tx_en`"]
pub struct DMA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TX_EN_W<'a> {
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
#[doc = "Reader of field `dma_rx_en`"]
pub type DMA_RX_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dma_rx_en`"]
pub struct DMA_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `dma_divide_16`"]
pub type DMA_DIVIDE_16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dma_divide_16`"]
pub struct DMA_DIVIDE_16_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_DIVIDE_16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `sign_expand_en`"]
pub type SIGN_EXPAND_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sign_expand_en`"]
pub struct SIGN_EXPAND_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGN_EXPAND_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Gating of sclk"]
    #[inline(always)]
    pub fn clk_gate(&self) -> CLK_GATE_R {
        CLK_GATE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - The number of sclk cycles for which the word select line stayd in the left aligned or right aligned mode"]
    #[inline(always)]
    pub fn clk_word_size(&self) -> CLK_WORD_SIZE_R {
        CLK_WORD_SIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - Alignment mode setting"]
    #[inline(always)]
    pub fn align_mode(&self) -> ALIGN_MODE_R {
        ALIGN_MODE_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - DMA transmit enable control"]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMA receive enable control"]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Split 32bit data to two 16 bit data and filled in left and right channel. Used with dma_tx_en or dma_rx_en"]
    #[inline(always)]
    pub fn dma_divide_16(&self) -> DMA_DIVIDE_16_R {
        DMA_DIVIDE_16_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SIGN_EXPAND_EN"]
    #[inline(always)]
    pub fn sign_expand_en(&self) -> SIGN_EXPAND_EN_R {
        SIGN_EXPAND_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Gating of sclk"]
    #[inline(always)]
    pub fn clk_gate(&mut self) -> CLK_GATE_W {
        CLK_GATE_W { w: self }
    }
    #[doc = "Bits 3:4 - The number of sclk cycles for which the word select line stayd in the left aligned or right aligned mode"]
    #[inline(always)]
    pub fn clk_word_size(&mut self) -> CLK_WORD_SIZE_W {
        CLK_WORD_SIZE_W { w: self }
    }
    #[doc = "Bits 5:7 - Alignment mode setting"]
    #[inline(always)]
    pub fn align_mode(&mut self) -> ALIGN_MODE_W {
        ALIGN_MODE_W { w: self }
    }
    #[doc = "Bit 8 - DMA transmit enable control"]
    #[inline(always)]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W {
        DMA_TX_EN_W { w: self }
    }
    #[doc = "Bit 9 - DMA receive enable control"]
    #[inline(always)]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W {
        DMA_RX_EN_W { w: self }
    }
    #[doc = "Bit 10 - Split 32bit data to two 16 bit data and filled in left and right channel. Used with dma_tx_en or dma_rx_en"]
    #[inline(always)]
    pub fn dma_divide_16(&mut self) -> DMA_DIVIDE_16_W {
        DMA_DIVIDE_16_W { w: self }
    }
    #[doc = "Bit 11 - SIGN_EXPAND_EN"]
    #[inline(always)]
    pub fn sign_expand_en(&mut self) -> SIGN_EXPAND_EN_W {
        SIGN_EXPAND_EN_W { w: self }
    }
}
