#[doc = "Reader of register dma_sel1"]
pub type R = crate::R<u32, super::DMA_SEL1>;
#[doc = "Writer for register dma_sel1"]
pub type W = crate::W<u32, super::DMA_SEL1>;
#[doc = "Register dma_sel1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_SEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = ""]
pub type DMA_SEL5_A = super::dma_sel0::DMA_SEL0_A;
#[doc = "Reader of field `dma_sel5`"]
pub type DMA_SEL5_R = crate::R<u8, super::dma_sel0::DMA_SEL0_A>;
#[doc = "Write proxy for field `dma_sel5`"]
pub struct DMA_SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_SEL5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(super::dma_sel0::DMA_SEL0_A::I2S0_BF_VOICE_REQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn dma_sel5(&self) -> DMA_SEL5_R {
        DMA_SEL5_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn dma_sel5(&mut self) -> DMA_SEL5_W {
        DMA_SEL5_W { w: self }
    }
}
