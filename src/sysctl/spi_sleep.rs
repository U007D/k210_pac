#[doc = "Reader of register spi_sleep"]
pub type R = crate::R<u32, super::SPI_SLEEP>;
#[doc = "Writer for register spi_sleep"]
pub type W = crate::W<u32, super::SPI_SLEEP>;
#[doc = "Register spi_sleep `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_SLEEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ssi0_sleep`"]
pub type SSI0_SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ssi0_sleep`"]
pub struct SSI0_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI0_SLEEP_W<'a> {
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
#[doc = "Reader of field `ssi1_sleep`"]
pub type SSI1_SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ssi1_sleep`"]
pub struct SSI1_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI1_SLEEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ssi2_sleep`"]
pub type SSI2_SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ssi2_sleep`"]
pub struct SSI2_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI2_SLEEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ssi3_sleep`"]
pub type SSI3_SLEEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ssi3_sleep`"]
pub struct SSI3_SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI3_SLEEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ssi0_sleep(&self) -> SSI0_SLEEP_R {
        SSI0_SLEEP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ssi1_sleep(&self) -> SSI1_SLEEP_R {
        SSI1_SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ssi2_sleep(&self) -> SSI2_SLEEP_R {
        SSI2_SLEEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ssi3_sleep(&self) -> SSI3_SLEEP_R {
        SSI3_SLEEP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ssi0_sleep(&mut self) -> SSI0_SLEEP_W {
        SSI0_SLEEP_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ssi1_sleep(&mut self) -> SSI1_SLEEP_W {
        SSI1_SLEEP_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ssi2_sleep(&mut self) -> SSI2_SLEEP_W {
        SSI2_SLEEP_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ssi3_sleep(&mut self) -> SSI3_SLEEP_W {
        SSI3_SLEEP_W { w: self }
    }
}
