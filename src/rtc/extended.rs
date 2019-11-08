#[doc = "Reader of register extended"]
pub type R = crate::R<u32, super::EXTENDED>;
#[doc = "Writer for register extended"]
pub type W = crate::W<u32, super::EXTENDED>;
#[doc = "Register extended `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTENDED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `century`"]
pub type CENTURY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `century`"]
pub struct CENTURY_W<'a> {
    w: &'a mut W,
}
impl<'a> CENTURY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Is leap year. 1 is leap year, 0 is not leap year\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEAP_YEAR_A {
    #[doc = "0: 0 is not leap year"]
    NOT_LEAP,
    #[doc = "1: 1 is leap year"]
    LEAP,
}
impl From<LEAP_YEAR_A> for bool {
    #[inline(always)]
    fn from(variant: LEAP_YEAR_A) -> Self {
        match variant {
            LEAP_YEAR_A::NOT_LEAP => false,
            LEAP_YEAR_A::LEAP => true,
        }
    }
}
#[doc = "Reader of field `leap_year`"]
pub type LEAP_YEAR_R = crate::R<bool, LEAP_YEAR_A>;
impl LEAP_YEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEAP_YEAR_A {
        match self.bits {
            false => LEAP_YEAR_A::NOT_LEAP,
            true => LEAP_YEAR_A::LEAP,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_LEAP`"]
    #[inline(always)]
    pub fn is_not_leap(&self) -> bool {
        *self == LEAP_YEAR_A::NOT_LEAP
    }
    #[doc = "Checks if the value of the field is `LEAP`"]
    #[inline(always)]
    pub fn is_leap(&self) -> bool {
        *self == LEAP_YEAR_A::LEAP
    }
}
#[doc = "Write proxy for field `leap_year`"]
pub struct LEAP_YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> LEAP_YEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEAP_YEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "0 is not leap year"]
    #[inline(always)]
    pub fn not_leap(self) -> &'a mut W {
        self.variant(LEAP_YEAR_A::NOT_LEAP)
    }
    #[doc = "1 is leap year"]
    #[inline(always)]
    pub fn leap(self) -> &'a mut W {
        self.variant(LEAP_YEAR_A::LEAP)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Century. Range \\[0,31\\]"]
    #[inline(always)]
    pub fn century(&self) -> CENTURY_R {
        CENTURY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Is leap year. 1 is leap year, 0 is not leap year"]
    #[inline(always)]
    pub fn leap_year(&self) -> LEAP_YEAR_R {
        LEAP_YEAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Century. Range \\[0,31\\]"]
    #[inline(always)]
    pub fn century(&mut self) -> CENTURY_W {
        CENTURY_W { w: self }
    }
    #[doc = "Bit 5 - Is leap year. 1 is leap year, 0 is not leap year"]
    #[inline(always)]
    pub fn leap_year(&mut self) -> LEAP_YEAR_W {
        LEAP_YEAR_W { w: self }
    }
}
