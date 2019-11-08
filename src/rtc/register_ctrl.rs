#[doc = "Reader of register register_ctrl"]
pub type R = crate::R<u32, super::REGISTER_CTRL>;
#[doc = "Writer for register register_ctrl"]
pub type W = crate::W<u32, super::REGISTER_CTRL>;
#[doc = "Register register_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::REGISTER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `read_enable`"]
pub type READ_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `read_enable`"]
pub struct READ_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_ENABLE_W<'a> {
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
#[doc = "Reader of field `write_enable`"]
pub type WRITE_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `write_enable`"]
pub struct WRITE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ENABLE_W<'a> {
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
#[doc = "Reader of field `timer_mask`"]
pub type TIMER_MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `timer_mask`"]
pub struct TIMER_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 13)) | (((value as u32) & 0xff) << 13);
        self.w
    }
}
#[doc = "Reader of field `alarm_mask`"]
pub type ALARM_MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `alarm_mask`"]
pub struct ALARM_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 21)) | (((value as u32) & 0xff) << 21);
        self.w
    }
}
#[doc = "Reader of field `initial_count_mask`"]
pub type INITIAL_COUNT_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `initial_count_mask`"]
pub struct INITIAL_COUNT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INITIAL_COUNT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `interrupt_register_mask`"]
pub type INTERRUPT_REGISTER_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `interrupt_register_mask`"]
pub struct INTERRUPT_REGISTER_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_REGISTER_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC timer read enable"]
    #[inline(always)]
    pub fn read_enable(&self) -> READ_ENABLE_R {
        READ_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC timer write enable"]
    #[inline(always)]
    pub fn write_enable(&self) -> WRITE_ENABLE_R {
        WRITE_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 13:20 - RTC timer mask"]
    #[inline(always)]
    pub fn timer_mask(&self) -> TIMER_MASK_R {
        TIMER_MASK_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:28 - RTC alarm mask"]
    #[inline(always)]
    pub fn alarm_mask(&self) -> ALARM_MASK_R {
        ALARM_MASK_R::new(((self.bits >> 21) & 0xff) as u8)
    }
    #[doc = "Bit 29 - RTC counter initial count value mask"]
    #[inline(always)]
    pub fn initial_count_mask(&self) -> INITIAL_COUNT_MASK_R {
        INITIAL_COUNT_MASK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - RTC interrupt register mask"]
    #[inline(always)]
    pub fn interrupt_register_mask(&self) -> INTERRUPT_REGISTER_MASK_R {
        INTERRUPT_REGISTER_MASK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC timer read enable"]
    #[inline(always)]
    pub fn read_enable(&mut self) -> READ_ENABLE_W {
        READ_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - RTC timer write enable"]
    #[inline(always)]
    pub fn write_enable(&mut self) -> WRITE_ENABLE_W {
        WRITE_ENABLE_W { w: self }
    }
    #[doc = "Bits 13:20 - RTC timer mask"]
    #[inline(always)]
    pub fn timer_mask(&mut self) -> TIMER_MASK_W {
        TIMER_MASK_W { w: self }
    }
    #[doc = "Bits 21:28 - RTC alarm mask"]
    #[inline(always)]
    pub fn alarm_mask(&mut self) -> ALARM_MASK_W {
        ALARM_MASK_W { w: self }
    }
    #[doc = "Bit 29 - RTC counter initial count value mask"]
    #[inline(always)]
    pub fn initial_count_mask(&mut self) -> INITIAL_COUNT_MASK_W {
        INITIAL_COUNT_MASK_W { w: self }
    }
    #[doc = "Bit 30 - RTC interrupt register mask"]
    #[inline(always)]
    pub fn interrupt_register_mask(&mut self) -> INTERRUPT_REGISTER_MASK_W {
        INTERRUPT_REGISTER_MASK_W { w: self }
    }
}
