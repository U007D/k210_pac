#[doc = "Reader of register rom_error"]
pub type R = crate::R<u32, super::ROM_ERROR>;
#[doc = "Writer for register rom_error"]
pub type W = crate::W<u32, super::ROM_ERROR>;
#[doc = "Register rom_error `reset()`'s with value 0"]
impl crate::ResetValue for super::ROM_ERROR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `rom_mul_error`"]
pub type ROM_MUL_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rom_mul_error`"]
pub struct ROM_MUL_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_MUL_ERROR_W<'a> {
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
#[doc = "Reader of field `rom_one_error`"]
pub type ROM_ONE_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `rom_one_error`"]
pub struct ROM_ONE_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_ONE_ERROR_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rom_mul_error(&self) -> ROM_MUL_ERROR_R {
        ROM_MUL_ERROR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rom_one_error(&self) -> ROM_ONE_ERROR_R {
        ROM_ONE_ERROR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rom_mul_error(&mut self) -> ROM_MUL_ERROR_W {
        ROM_MUL_ERROR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rom_one_error(&mut self) -> ROM_ONE_ERROR_W {
        ROM_ONE_ERROR_W { w: self }
    }
}
