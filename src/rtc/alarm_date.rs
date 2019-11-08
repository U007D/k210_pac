#[doc = "Reader of register alarm_date"]
pub type R = crate::R<u32, super::ALARM_DATE>;
#[doc = "Writer for register alarm_date"]
pub type W = crate::W<u32, super::ALARM_DATE>;
#[doc = "Register alarm_date `reset()`'s with value 0"]
impl crate::ResetValue for super::ALARM_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `week`"]
pub type WEEK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `week`"]
pub struct WEEK_W<'a> {
    w: &'a mut W,
}
impl<'a> WEEK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `day`"]
pub type DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `day`"]
pub struct DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `month`"]
pub type MONTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `month`"]
pub struct MONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `year`"]
pub type YEAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `year`"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Week. Range \\[0,6\\]. 0 is Sunday."]
    #[inline(always)]
    pub fn week(&self) -> WEEK_R {
        WEEK_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - Day. Range \\[1,31\\] or \\[1,30\\] or \\[1,29\\] or \\[1,28\\]"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Month. Range \\[1,12\\]"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Year. Range \\[0,99\\]"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Week. Range \\[0,6\\]. 0 is Sunday."]
    #[inline(always)]
    pub fn week(&mut self) -> WEEK_W {
        WEEK_W { w: self }
    }
    #[doc = "Bits 8:12 - Day. Range \\[1,31\\] or \\[1,30\\] or \\[1,29\\] or \\[1,28\\]"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W {
        DAY_W { w: self }
    }
    #[doc = "Bits 16:19 - Month. Range \\[1,12\\]"]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W {
        MONTH_W { w: self }
    }
    #[doc = "Bits 20:31 - Year. Range \\[0,99\\]"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
}
