#[doc = "Reader of register interrupt_ctrl"]
pub type R = crate::R<u32, super::INTERRUPT_CTRL>;
#[doc = "Writer for register interrupt_ctrl"]
pub type W = crate::W<u32, super::INTERRUPT_CTRL>;
#[doc = "Register interrupt_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::INTERRUPT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tick_enable`"]
pub type TICK_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tick_enable`"]
pub struct TICK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_ENABLE_W<'a> {
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
#[doc = "Reader of field `alarm_enable`"]
pub type ALARM_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `alarm_enable`"]
pub struct ALARM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM_ENABLE_W<'a> {
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
#[doc = "Reader of field `tick_int_mode`"]
pub type TICK_INT_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `tick_int_mode`"]
pub struct TICK_INT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_INT_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `alarm_compare_mask`"]
pub type ALARM_COMPARE_MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `alarm_compare_mask`"]
pub struct ALARM_COMPARE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM_COMPARE_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TICK_ENABLE"]
    #[inline(always)]
    pub fn tick_enable(&self) -> TICK_ENABLE_R {
        TICK_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alarm_enable(&self) -> ALARM_ENABLE_R {
        ALARM_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Tick interrupt enable"]
    #[inline(always)]
    pub fn tick_int_mode(&self) -> TICK_INT_MODE_R {
        TICK_INT_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 24:31 - Alarm compare mask for interrupt"]
    #[inline(always)]
    pub fn alarm_compare_mask(&self) -> ALARM_COMPARE_MASK_R {
        ALARM_COMPARE_MASK_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TICK_ENABLE"]
    #[inline(always)]
    pub fn tick_enable(&mut self) -> TICK_ENABLE_W {
        TICK_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alarm_enable(&mut self) -> ALARM_ENABLE_W {
        ALARM_ENABLE_W { w: self }
    }
    #[doc = "Bits 2:3 - Tick interrupt enable"]
    #[inline(always)]
    pub fn tick_int_mode(&mut self) -> TICK_INT_MODE_W {
        TICK_INT_MODE_W { w: self }
    }
    #[doc = "Bits 24:31 - Alarm compare mask for interrupt"]
    #[inline(always)]
    pub fn alarm_compare_mask(&mut self) -> ALARM_COMPARE_MASK_W {
        ALARM_COMPARE_MASK_W { w: self }
    }
}
