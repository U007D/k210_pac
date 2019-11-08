#[doc = "Reader of register dma_sel0"]
pub type R = crate::R<u32, super::DMA_SEL0>;
#[doc = "Writer for register dma_sel0"]
pub type W = crate::W<u32, super::DMA_SEL0>;
#[doc = "Register dma_sel0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_SEL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_SEL0_A {
    #[doc = "0: `0`"]
    SSI0_RX_REQ,
    #[doc = "1: `1`"]
    SSI0_TX_REQ,
    #[doc = "2: `10`"]
    SSI1_RX_REQ,
    #[doc = "3: `11`"]
    SSI1_TX_REQ,
    #[doc = "4: `100`"]
    SSI2_RX_REQ,
    #[doc = "5: `101`"]
    SSI2_TX_REQ,
    #[doc = "6: `110`"]
    SSI3_RX_REQ,
    #[doc = "7: `111`"]
    SSI3_TX_REQ,
    #[doc = "8: `1000`"]
    I2C0_RX_REQ,
    #[doc = "9: `1001`"]
    I2C0_TX_REQ,
    #[doc = "10: `1010`"]
    I2C1_RX_REQ,
    #[doc = "11: `1011`"]
    I2C1_TX_REQ,
    #[doc = "12: `1100`"]
    I2C2_RX_REQ,
    #[doc = "13: `1101`"]
    I2C2_TX_REQ,
    #[doc = "14: `1110`"]
    UART1_RX_REQ,
    #[doc = "15: `1111`"]
    UART1_TX_REQ,
    #[doc = "16: `10000`"]
    UART2_RX_REQ,
    #[doc = "17: `10001`"]
    UART2_TX_REQ,
    #[doc = "18: `10010`"]
    UART3_RX_REQ,
    #[doc = "19: `10011`"]
    UART3_TX_REQ,
    #[doc = "20: `10100`"]
    AES_REQ,
    #[doc = "21: `10101`"]
    SHA_RX_REQ,
    #[doc = "22: `10110`"]
    AI_RX_REQ,
    #[doc = "23: `10111`"]
    FFT_RX_REQ,
    #[doc = "24: `11000`"]
    FFT_TX_REQ,
    #[doc = "25: `11001`"]
    I2S0_TX_REQ,
    #[doc = "26: `11010`"]
    I2S0_RX_REQ,
    #[doc = "27: `11011`"]
    I2S1_TX_REQ,
    #[doc = "28: `11100`"]
    I2S1_RX_REQ,
    #[doc = "29: `11101`"]
    I2S2_TX_REQ,
    #[doc = "30: `11110`"]
    I2S2_RX_REQ,
    #[doc = "31: `11111`"]
    I2S0_BF_DIR_REQ,
    #[doc = "32: `100000`"]
    I2S0_BF_VOICE_REQ,
}
impl From<DMA_SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA_SEL0_A) -> Self {
        match variant {
            DMA_SEL0_A::SSI0_RX_REQ => 0,
            DMA_SEL0_A::SSI0_TX_REQ => 1,
            DMA_SEL0_A::SSI1_RX_REQ => 2,
            DMA_SEL0_A::SSI1_TX_REQ => 3,
            DMA_SEL0_A::SSI2_RX_REQ => 4,
            DMA_SEL0_A::SSI2_TX_REQ => 5,
            DMA_SEL0_A::SSI3_RX_REQ => 6,
            DMA_SEL0_A::SSI3_TX_REQ => 7,
            DMA_SEL0_A::I2C0_RX_REQ => 8,
            DMA_SEL0_A::I2C0_TX_REQ => 9,
            DMA_SEL0_A::I2C1_RX_REQ => 10,
            DMA_SEL0_A::I2C1_TX_REQ => 11,
            DMA_SEL0_A::I2C2_RX_REQ => 12,
            DMA_SEL0_A::I2C2_TX_REQ => 13,
            DMA_SEL0_A::UART1_RX_REQ => 14,
            DMA_SEL0_A::UART1_TX_REQ => 15,
            DMA_SEL0_A::UART2_RX_REQ => 16,
            DMA_SEL0_A::UART2_TX_REQ => 17,
            DMA_SEL0_A::UART3_RX_REQ => 18,
            DMA_SEL0_A::UART3_TX_REQ => 19,
            DMA_SEL0_A::AES_REQ => 20,
            DMA_SEL0_A::SHA_RX_REQ => 21,
            DMA_SEL0_A::AI_RX_REQ => 22,
            DMA_SEL0_A::FFT_RX_REQ => 23,
            DMA_SEL0_A::FFT_TX_REQ => 24,
            DMA_SEL0_A::I2S0_TX_REQ => 25,
            DMA_SEL0_A::I2S0_RX_REQ => 26,
            DMA_SEL0_A::I2S1_TX_REQ => 27,
            DMA_SEL0_A::I2S1_RX_REQ => 28,
            DMA_SEL0_A::I2S2_TX_REQ => 29,
            DMA_SEL0_A::I2S2_RX_REQ => 30,
            DMA_SEL0_A::I2S0_BF_DIR_REQ => 31,
            DMA_SEL0_A::I2S0_BF_VOICE_REQ => 32,
        }
    }
}
#[doc = "Reader of field `dma_sel0`"]
pub type DMA_SEL0_R = crate::R<u8, DMA_SEL0_A>;
impl DMA_SEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DMA_SEL0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DMA_SEL0_A::SSI0_RX_REQ),
            1 => Val(DMA_SEL0_A::SSI0_TX_REQ),
            2 => Val(DMA_SEL0_A::SSI1_RX_REQ),
            3 => Val(DMA_SEL0_A::SSI1_TX_REQ),
            4 => Val(DMA_SEL0_A::SSI2_RX_REQ),
            5 => Val(DMA_SEL0_A::SSI2_TX_REQ),
            6 => Val(DMA_SEL0_A::SSI3_RX_REQ),
            7 => Val(DMA_SEL0_A::SSI3_TX_REQ),
            8 => Val(DMA_SEL0_A::I2C0_RX_REQ),
            9 => Val(DMA_SEL0_A::I2C0_TX_REQ),
            10 => Val(DMA_SEL0_A::I2C1_RX_REQ),
            11 => Val(DMA_SEL0_A::I2C1_TX_REQ),
            12 => Val(DMA_SEL0_A::I2C2_RX_REQ),
            13 => Val(DMA_SEL0_A::I2C2_TX_REQ),
            14 => Val(DMA_SEL0_A::UART1_RX_REQ),
            15 => Val(DMA_SEL0_A::UART1_TX_REQ),
            16 => Val(DMA_SEL0_A::UART2_RX_REQ),
            17 => Val(DMA_SEL0_A::UART2_TX_REQ),
            18 => Val(DMA_SEL0_A::UART3_RX_REQ),
            19 => Val(DMA_SEL0_A::UART3_TX_REQ),
            20 => Val(DMA_SEL0_A::AES_REQ),
            21 => Val(DMA_SEL0_A::SHA_RX_REQ),
            22 => Val(DMA_SEL0_A::AI_RX_REQ),
            23 => Val(DMA_SEL0_A::FFT_RX_REQ),
            24 => Val(DMA_SEL0_A::FFT_TX_REQ),
            25 => Val(DMA_SEL0_A::I2S0_TX_REQ),
            26 => Val(DMA_SEL0_A::I2S0_RX_REQ),
            27 => Val(DMA_SEL0_A::I2S1_TX_REQ),
            28 => Val(DMA_SEL0_A::I2S1_RX_REQ),
            29 => Val(DMA_SEL0_A::I2S2_TX_REQ),
            30 => Val(DMA_SEL0_A::I2S2_RX_REQ),
            31 => Val(DMA_SEL0_A::I2S0_BF_DIR_REQ),
            32 => Val(DMA_SEL0_A::I2S0_BF_VOICE_REQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI0_RX_REQ`"]
    #[inline(always)]
    pub fn is_ssi0_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::SSI0_RX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI0_TX_REQ`"]
    #[inline(always)]
    pub fn is_ssi0_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::SSI0_TX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI1_RX_REQ`"]
    #[inline(always)]
    pub fn is_ssi1_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::SSI1_RX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI1_TX_REQ`"]
    #[inline(always)]
    pub fn is_ssi1_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::SSI1_TX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI2_RX_REQ`"]
    #[inline(always)]
    pub fn is_ssi2_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::SSI2_RX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI2_TX_REQ`"]
    #[inline(always)]
    pub fn is_ssi2_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::SSI2_TX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI3_RX_REQ`"]
    #[inline(always)]
    pub fn is_ssi3_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::SSI3_RX_REQ
    }
    #[doc = "Checks if the value of the field is `SSI3_TX_REQ`"]
    #[inline(always)]
    pub fn is_ssi3_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::SSI3_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C0_RX_REQ`"]
    #[inline(always)]
    pub fn is_i2c0_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2C0_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C0_TX_REQ`"]
    #[inline(always)]
    pub fn is_i2c0_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2C0_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C1_RX_REQ`"]
    #[inline(always)]
    pub fn is_i2c1_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2C1_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C1_TX_REQ`"]
    #[inline(always)]
    pub fn is_i2c1_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2C1_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C2_RX_REQ`"]
    #[inline(always)]
    pub fn is_i2c2_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2C2_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2C2_TX_REQ`"]
    #[inline(always)]
    pub fn is_i2c2_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2C2_TX_REQ
    }
    #[doc = "Checks if the value of the field is `UART1_RX_REQ`"]
    #[inline(always)]
    pub fn is_uart1_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::UART1_RX_REQ
    }
    #[doc = "Checks if the value of the field is `UART1_TX_REQ`"]
    #[inline(always)]
    pub fn is_uart1_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::UART1_TX_REQ
    }
    #[doc = "Checks if the value of the field is `UART2_RX_REQ`"]
    #[inline(always)]
    pub fn is_uart2_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::UART2_RX_REQ
    }
    #[doc = "Checks if the value of the field is `UART2_TX_REQ`"]
    #[inline(always)]
    pub fn is_uart2_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::UART2_TX_REQ
    }
    #[doc = "Checks if the value of the field is `UART3_RX_REQ`"]
    #[inline(always)]
    pub fn is_uart3_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::UART3_RX_REQ
    }
    #[doc = "Checks if the value of the field is `UART3_TX_REQ`"]
    #[inline(always)]
    pub fn is_uart3_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::UART3_TX_REQ
    }
    #[doc = "Checks if the value of the field is `AES_REQ`"]
    #[inline(always)]
    pub fn is_aes_req(&self) -> bool {
        *self == DMA_SEL0_A::AES_REQ
    }
    #[doc = "Checks if the value of the field is `SHA_RX_REQ`"]
    #[inline(always)]
    pub fn is_sha_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::SHA_RX_REQ
    }
    #[doc = "Checks if the value of the field is `AI_RX_REQ`"]
    #[inline(always)]
    pub fn is_ai_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::AI_RX_REQ
    }
    #[doc = "Checks if the value of the field is `FFT_RX_REQ`"]
    #[inline(always)]
    pub fn is_fft_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::FFT_RX_REQ
    }
    #[doc = "Checks if the value of the field is `FFT_TX_REQ`"]
    #[inline(always)]
    pub fn is_fft_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::FFT_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S0_TX_REQ`"]
    #[inline(always)]
    pub fn is_i2s0_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2S0_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S0_RX_REQ`"]
    #[inline(always)]
    pub fn is_i2s0_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2S0_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S1_TX_REQ`"]
    #[inline(always)]
    pub fn is_i2s1_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2S1_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S1_RX_REQ`"]
    #[inline(always)]
    pub fn is_i2s1_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2S1_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S2_TX_REQ`"]
    #[inline(always)]
    pub fn is_i2s2_tx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2S2_TX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S2_RX_REQ`"]
    #[inline(always)]
    pub fn is_i2s2_rx_req(&self) -> bool {
        *self == DMA_SEL0_A::I2S2_RX_REQ
    }
    #[doc = "Checks if the value of the field is `I2S0_BF_DIR_REQ`"]
    #[inline(always)]
    pub fn is_i2s0_bf_dir_req(&self) -> bool {
        *self == DMA_SEL0_A::I2S0_BF_DIR_REQ
    }
    #[doc = "Checks if the value of the field is `I2S0_BF_VOICE_REQ`"]
    #[inline(always)]
    pub fn is_i2s0_bf_voice_req(&self) -> bool {
        *self == DMA_SEL0_A::I2S0_BF_VOICE_REQ
    }
}
#[doc = "Write proxy for field `dma_sel0`"]
pub struct DMA_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_SEL0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_BF_VOICE_REQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = ""]
pub type DMA_SEL1_A = DMA_SEL0_A;
#[doc = "Reader of field `dma_sel1`"]
pub type DMA_SEL1_R = crate::R<u8, DMA_SEL0_A>;
#[doc = "Write proxy for field `dma_sel1`"]
pub struct DMA_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_SEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_BF_VOICE_REQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = ""]
pub type DMA_SEL2_A = DMA_SEL0_A;
#[doc = "Reader of field `dma_sel2`"]
pub type DMA_SEL2_R = crate::R<u8, DMA_SEL0_A>;
#[doc = "Write proxy for field `dma_sel2`"]
pub struct DMA_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_SEL2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_BF_VOICE_REQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = ""]
pub type DMA_SEL3_A = DMA_SEL0_A;
#[doc = "Reader of field `dma_sel3`"]
pub type DMA_SEL3_R = crate::R<u8, DMA_SEL0_A>;
#[doc = "Write proxy for field `dma_sel3`"]
pub struct DMA_SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_SEL3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_BF_VOICE_REQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = ""]
pub type DMA_SEL4_A = DMA_SEL0_A;
#[doc = "Reader of field `dma_sel4`"]
pub type DMA_SEL4_R = crate::R<u8, DMA_SEL0_A>;
#[doc = "Write proxy for field `dma_sel4`"]
pub struct DMA_SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_SEL4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ssi0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI0_RX_REQ)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ssi0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI0_TX_REQ)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ssi1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI1_RX_REQ)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn ssi1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI1_TX_REQ)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ssi2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI2_RX_REQ)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn ssi2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI2_TX_REQ)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn ssi3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI3_RX_REQ)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn ssi3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SSI3_TX_REQ)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn i2c0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C0_RX_REQ)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn i2c0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C0_TX_REQ)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn i2c1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C1_RX_REQ)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn i2c1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C1_TX_REQ)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn i2c2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C2_RX_REQ)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn i2c2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2C2_TX_REQ)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn uart1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART1_RX_REQ)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn uart1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART1_TX_REQ)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn uart2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART2_RX_REQ)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn uart2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART2_TX_REQ)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn uart3_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART3_RX_REQ)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn uart3_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::UART3_TX_REQ)
    }
    #[doc = "`10100`"]
    #[inline(always)]
    pub fn aes_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::AES_REQ)
    }
    #[doc = "`10101`"]
    #[inline(always)]
    pub fn sha_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::SHA_RX_REQ)
    }
    #[doc = "`10110`"]
    #[inline(always)]
    pub fn ai_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::AI_RX_REQ)
    }
    #[doc = "`10111`"]
    #[inline(always)]
    pub fn fft_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::FFT_RX_REQ)
    }
    #[doc = "`11000`"]
    #[inline(always)]
    pub fn fft_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::FFT_TX_REQ)
    }
    #[doc = "`11001`"]
    #[inline(always)]
    pub fn i2s0_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_TX_REQ)
    }
    #[doc = "`11010`"]
    #[inline(always)]
    pub fn i2s0_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_RX_REQ)
    }
    #[doc = "`11011`"]
    #[inline(always)]
    pub fn i2s1_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S1_TX_REQ)
    }
    #[doc = "`11100`"]
    #[inline(always)]
    pub fn i2s1_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S1_RX_REQ)
    }
    #[doc = "`11101`"]
    #[inline(always)]
    pub fn i2s2_tx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S2_TX_REQ)
    }
    #[doc = "`11110`"]
    #[inline(always)]
    pub fn i2s2_rx_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S2_RX_REQ)
    }
    #[doc = "`11111`"]
    #[inline(always)]
    pub fn i2s0_bf_dir_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_BF_DIR_REQ)
    }
    #[doc = "`100000`"]
    #[inline(always)]
    pub fn i2s0_bf_voice_req(self) -> &'a mut W {
        self.variant(DMA_SEL0_A::I2S0_BF_VOICE_REQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn dma_sel0(&self) -> DMA_SEL0_R {
        DMA_SEL0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn dma_sel1(&self) -> DMA_SEL1_R {
        DMA_SEL1_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn dma_sel2(&self) -> DMA_SEL2_R {
        DMA_SEL2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn dma_sel3(&self) -> DMA_SEL3_R {
        DMA_SEL3_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn dma_sel4(&self) -> DMA_SEL4_R {
        DMA_SEL4_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn dma_sel0(&mut self) -> DMA_SEL0_W {
        DMA_SEL0_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn dma_sel1(&mut self) -> DMA_SEL1_W {
        DMA_SEL1_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn dma_sel2(&mut self) -> DMA_SEL2_W {
        DMA_SEL2_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn dma_sel3(&mut self) -> DMA_SEL3_W {
        DMA_SEL3_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn dma_sel4(&mut self) -> DMA_SEL4_W {
        DMA_SEL4_W { w: self }
    }
}
