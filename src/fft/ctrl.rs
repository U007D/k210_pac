#[doc = "Reader of register ctrl"]
pub type R = crate::R<u64, super::CTRL>;
#[doc = "Writer for register ctrl"]
pub type W = crate::W<u64, super::CTRL>;
#[doc = "Register ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FFT calculation data length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POINT_A {
    #[doc = "0: 512 point"]
    P512,
    #[doc = "1: 256 point"]
    P256,
    #[doc = "2: 128 point"]
    P128,
    #[doc = "3: 64 point"]
    P64,
}
impl From<POINT_A> for u8 {
    #[inline(always)]
    fn from(variant: POINT_A) -> Self {
        match variant {
            POINT_A::P512 => 0,
            POINT_A::P256 => 1,
            POINT_A::P128 => 2,
            POINT_A::P64 => 3,
        }
    }
}
#[doc = "Reader of field `point`"]
pub type POINT_R = crate::R<u8, POINT_A>;
impl POINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, POINT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(POINT_A::P512),
            1 => Val(POINT_A::P256),
            2 => Val(POINT_A::P128),
            3 => Val(POINT_A::P64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P512`"]
    #[inline(always)]
    pub fn is_p512(&self) -> bool {
        *self == POINT_A::P512
    }
    #[doc = "Checks if the value of the field is `P256`"]
    #[inline(always)]
    pub fn is_p256(&self) -> bool {
        *self == POINT_A::P256
    }
    #[doc = "Checks if the value of the field is `P128`"]
    #[inline(always)]
    pub fn is_p128(&self) -> bool {
        *self == POINT_A::P128
    }
    #[doc = "Checks if the value of the field is `P64`"]
    #[inline(always)]
    pub fn is_p64(&self) -> bool {
        *self == POINT_A::P64
    }
}
#[doc = "Write proxy for field `point`"]
pub struct POINT_W<'a> {
    w: &'a mut W,
}
impl<'a> POINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POINT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "512 point"]
    #[inline(always)]
    pub fn p512(self) -> &'a mut W {
        self.variant(POINT_A::P512)
    }
    #[doc = "256 point"]
    #[inline(always)]
    pub fn p256(self) -> &'a mut W {
        self.variant(POINT_A::P256)
    }
    #[doc = "128 point"]
    #[inline(always)]
    pub fn p128(self) -> &'a mut W {
        self.variant(POINT_A::P128)
    }
    #[doc = "64 point"]
    #[inline(always)]
    pub fn p64(self) -> &'a mut W {
        self.variant(POINT_A::P64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u64) & 0x07);
        self.w
    }
}
#[doc = "FFT mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: FFT mode"]
    FFT,
    #[doc = "1: Inverse FFT mode"]
    IFFT,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::FFT => false,
            MODE_A::IFFT => true,
        }
    }
}
#[doc = "Reader of field `mode`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::FFT,
            true => MODE_A::IFFT,
        }
    }
    #[doc = "Checks if the value of the field is `FFT`"]
    #[inline(always)]
    pub fn is_fft(&self) -> bool {
        *self == MODE_A::FFT
    }
    #[doc = "Checks if the value of the field is `IFFT`"]
    #[inline(always)]
    pub fn is_ifft(&self) -> bool {
        *self == MODE_A::IFFT
    }
}
#[doc = "Write proxy for field `mode`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FFT mode"]
    #[inline(always)]
    pub fn fft(self) -> &'a mut W {
        self.variant(MODE_A::FFT)
    }
    #[doc = "Inverse FFT mode"]
    #[inline(always)]
    pub fn ifft(self) -> &'a mut W {
        self.variant(MODE_A::IFFT)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u64) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `shift`"]
pub type SHIFT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `shift`"]
pub struct SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 4)) | (((value as u64) & 0x01ff) << 4);
        self.w
    }
}
#[doc = "Reader of field `enable`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `enable`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u64) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `dma_send`"]
pub type DMA_SEND_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dma_send`"]
pub struct DMA_SEND_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_SEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u64) & 0x01) << 14);
        self.w
    }
}
#[doc = "Input data arrangement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT_MODE_A {
    #[doc = "0: RIRI (real imaginary interleaved)"]
    RIRI,
    #[doc = "1: RRRR (only real part)"]
    RRRR,
    #[doc = "2: First input the real part and then input the imaginary part"]
    RRII,
}
impl From<INPUT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT_MODE_A) -> Self {
        match variant {
            INPUT_MODE_A::RIRI => 0,
            INPUT_MODE_A::RRRR => 1,
            INPUT_MODE_A::RRII => 2,
        }
    }
}
#[doc = "Reader of field `input_mode`"]
pub type INPUT_MODE_R = crate::R<u8, INPUT_MODE_A>;
impl INPUT_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUT_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INPUT_MODE_A::RIRI),
            1 => Val(INPUT_MODE_A::RRRR),
            2 => Val(INPUT_MODE_A::RRII),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RIRI`"]
    #[inline(always)]
    pub fn is_riri(&self) -> bool {
        *self == INPUT_MODE_A::RIRI
    }
    #[doc = "Checks if the value of the field is `RRRR`"]
    #[inline(always)]
    pub fn is_rrrr(&self) -> bool {
        *self == INPUT_MODE_A::RRRR
    }
    #[doc = "Checks if the value of the field is `RRII`"]
    #[inline(always)]
    pub fn is_rrii(&self) -> bool {
        *self == INPUT_MODE_A::RRII
    }
}
#[doc = "Write proxy for field `input_mode`"]
pub struct INPUT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RIRI (real imaginary interleaved)"]
    #[inline(always)]
    pub fn riri(self) -> &'a mut W {
        self.variant(INPUT_MODE_A::RIRI)
    }
    #[doc = "RRRR (only real part)"]
    #[inline(always)]
    pub fn rrrr(self) -> &'a mut W {
        self.variant(INPUT_MODE_A::RRRR)
    }
    #[doc = "First input the real part and then input the imaginary part"]
    #[inline(always)]
    pub fn rrii(self) -> &'a mut W {
        self.variant(INPUT_MODE_A::RRII)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u64) & 0x03) << 15);
        self.w
    }
}
#[doc = "Effective width of input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_MODE_A {
    #[doc = "0: 64 bit effective"]
    WIDTH_64,
    #[doc = "1: 128 bit effective"]
    WIDTH_128,
}
impl From<DATA_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_MODE_A) -> Self {
        match variant {
            DATA_MODE_A::WIDTH_64 => false,
            DATA_MODE_A::WIDTH_128 => true,
        }
    }
}
#[doc = "Reader of field `data_mode`"]
pub type DATA_MODE_R = crate::R<bool, DATA_MODE_A>;
impl DATA_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_MODE_A {
        match self.bits {
            false => DATA_MODE_A::WIDTH_64,
            true => DATA_MODE_A::WIDTH_128,
        }
    }
    #[doc = "Checks if the value of the field is `WIDTH_64`"]
    #[inline(always)]
    pub fn is_width_64(&self) -> bool {
        *self == DATA_MODE_A::WIDTH_64
    }
    #[doc = "Checks if the value of the field is `WIDTH_128`"]
    #[inline(always)]
    pub fn is_width_128(&self) -> bool {
        *self == DATA_MODE_A::WIDTH_128
    }
}
#[doc = "Write proxy for field `data_mode`"]
pub struct DATA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "64 bit effective"]
    #[inline(always)]
    pub fn width_64(self) -> &'a mut W {
        self.variant(DATA_MODE_A::WIDTH_64)
    }
    #[doc = "128 bit effective"]
    #[inline(always)]
    pub fn width_128(self) -> &'a mut W {
        self.variant(DATA_MODE_A::WIDTH_128)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u64) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - FFT calculation data length"]
    #[inline(always)]
    pub fn point(&self) -> POINT_R {
        POINT_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - FFT mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:12 - Corresponding to the nine layer butterfly shift operation, 0x0: does not shift; 0x1: shift 1st layer. ..."]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new(((self.bits >> 4) & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - FFT enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FFT DMA enable"]
    #[inline(always)]
    pub fn dma_send(&self) -> DMA_SEND_R {
        DMA_SEND_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - Input data arrangement"]
    #[inline(always)]
    pub fn input_mode(&self) -> INPUT_MODE_R {
        INPUT_MODE_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Effective width of input data"]
    #[inline(always)]
    pub fn data_mode(&self) -> DATA_MODE_R {
        DATA_MODE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FFT calculation data length"]
    #[inline(always)]
    pub fn point(&mut self) -> POINT_W {
        POINT_W { w: self }
    }
    #[doc = "Bit 3 - FFT mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 4:12 - Corresponding to the nine layer butterfly shift operation, 0x0: does not shift; 0x1: shift 1st layer. ..."]
    #[inline(always)]
    pub fn shift(&mut self) -> SHIFT_W {
        SHIFT_W { w: self }
    }
    #[doc = "Bit 13 - FFT enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 14 - FFT DMA enable"]
    #[inline(always)]
    pub fn dma_send(&mut self) -> DMA_SEND_W {
        DMA_SEND_W { w: self }
    }
    #[doc = "Bits 15:16 - Input data arrangement"]
    #[inline(always)]
    pub fn input_mode(&mut self) -> INPUT_MODE_W {
        INPUT_MODE_W { w: self }
    }
    #[doc = "Bit 17 - Effective width of input data"]
    #[inline(always)]
    pub fn data_mode(&mut self) -> DATA_MODE_W {
        DATA_MODE_W { w: self }
    }
}
