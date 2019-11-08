#[doc = "Reader of register status"]
pub type R = crate::R<u64, super::STATUS>;
#[doc = "Writer for register status"]
pub type W = crate::W<u64, super::STATUS>;
#[doc = "Register status `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `fft_done`"]
pub type FFT_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `fft_done`"]
pub struct FFT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> FFT_DONE_W<'a> {
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
    #[doc = "Bit 0 - FFT done"]
    #[inline(always)]
    pub fn fft_done(&self) -> FFT_DONE_R {
        FFT_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FFT done"]
    #[inline(always)]
    pub fn fft_done(&mut self) -> FFT_DONE_W {
        FFT_DONE_W { w: self }
    }
}
