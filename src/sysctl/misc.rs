#[doc = "Reader of register misc"]
pub type R = crate::R<u32, super::MISC>;
#[doc = "Writer for register misc"]
pub type W = crate::W<u32, super::MISC>;
#[doc = "Register misc `reset()`'s with value 0"]
impl crate::ResetValue for super::MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `debug_sel`"]
pub type DEBUG_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `debug_sel`"]
pub struct DEBUG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `spi_dvp_data_enable`"]
pub type SPI_DVP_DATA_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `spi_dvp_data_enable`"]
pub struct SPI_DVP_DATA_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DVP_DATA_ENABLE_W<'a> {
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
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn debug_sel(&self) -> DEBUG_SEL_R {
        DEBUG_SEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_dvp_data_enable(&self) -> SPI_DVP_DATA_ENABLE_R {
        SPI_DVP_DATA_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn debug_sel(&mut self) -> DEBUG_SEL_W {
        DEBUG_SEL_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spi_dvp_data_enable(&mut self) -> SPI_DVP_DATA_ENABLE_W {
        SPI_DVP_DATA_ENABLE_W { w: self }
    }
}
