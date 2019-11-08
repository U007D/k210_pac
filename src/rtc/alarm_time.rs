#[doc = "Reader of register alarm_time"]
pub type R = crate::R<u32, super::ALARM_TIME>;
#[doc = "Writer for register alarm_time"]
pub type W = crate::W<u32, super::ALARM_TIME>;
#[doc = "Register alarm_time `reset()`'s with value 0"]
impl crate::ResetValue for super::ALARM_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `second`"]
pub type SECOND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `second`"]
pub struct SECOND_W<'a> {
    w: &'a mut W,
}
impl<'a> SECOND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `minute`"]
pub type MINUTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `minute`"]
pub struct MINUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `hour`"]
pub type HOUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `hour`"]
pub struct HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:15 - Second. Range \\[0,59\\]"]
    #[inline(always)]
    pub fn second(&self) -> SECOND_R {
        SECOND_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Minute. Range \\[0,59\\]"]
    #[inline(always)]
    pub fn minute(&self) -> MINUTE_R {
        MINUTE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - Hour. Range \\[0,23\\]"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 10:15 - Second. Range \\[0,59\\]"]
    #[inline(always)]
    pub fn second(&mut self) -> SECOND_W {
        SECOND_W { w: self }
    }
    #[doc = "Bits 16:21 - Minute. Range \\[0,59\\]"]
    #[inline(always)]
    pub fn minute(&mut self) -> MINUTE_W {
        MINUTE_W { w: self }
    }
    #[doc = "Bits 24:28 - Hour. Range \\[0,23\\]"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W {
        HOUR_W { w: self }
    }
}
