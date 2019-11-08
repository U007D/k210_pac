#[doc = "Reader of register finish"]
pub type R = crate::R<u32, super::FINISH>;
#[doc = "Writer for register finish"]
pub type W = crate::W<u32, super::FINISH>;
#[doc = "Register finish `reset()`'s with value 0"]
impl crate::ResetValue for super::FINISH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "AES operation finished status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINISH_A {
    #[doc = "0: Operation not finished"]
    NOT_FINISHED,
    #[doc = "1: Operation finished"]
    FINISHED,
}
impl From<FINISH_A> for bool {
    #[inline(always)]
    fn from(variant: FINISH_A) -> Self {
        match variant {
            FINISH_A::NOT_FINISHED => false,
            FINISH_A::FINISHED => true,
        }
    }
}
#[doc = "Reader of field `finish`"]
pub type FINISH_R = crate::R<bool, FINISH_A>;
impl FINISH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINISH_A {
        match self.bits {
            false => FINISH_A::NOT_FINISHED,
            true => FINISH_A::FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FINISHED`"]
    #[inline(always)]
    pub fn is_not_finished(&self) -> bool {
        *self == FINISH_A::NOT_FINISHED
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == FINISH_A::FINISHED
    }
}
#[doc = "Write proxy for field `finish`"]
pub struct FINISH_W<'a> {
    w: &'a mut W,
}
impl<'a> FINISH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FINISH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Operation not finished"]
    #[inline(always)]
    pub fn not_finished(self) -> &'a mut W {
        self.variant(FINISH_A::NOT_FINISHED)
    }
    #[doc = "Operation finished"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut W {
        self.variant(FINISH_A::FINISHED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - AES operation finished status"]
    #[inline(always)]
    pub fn finish(&self) -> FINISH_R {
        FINISH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES operation finished status"]
    #[inline(always)]
    pub fn finish(&mut self) -> FINISH_W {
        FINISH_W { w: self }
    }
}
