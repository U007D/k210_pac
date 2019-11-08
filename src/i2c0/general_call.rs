#[doc = "Reader of register general_call"]
pub type R = crate::R<u32, super::GENERAL_CALL>;
#[doc = "Writer for register general_call"]
pub type W = crate::W<u32, super::GENERAL_CALL>;
#[doc = "Register general_call `reset()`'s with value 0"]
impl crate::ResetValue for super::GENERAL_CALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `call_enable`"]
pub type CALL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `call_enable`"]
pub struct CALL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALL_ENABLE_W<'a> {
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
    #[doc = "Bit 0 - CALL_ENABLE"]
    #[inline(always)]
    pub fn call_enable(&self) -> CALL_ENABLE_R {
        CALL_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CALL_ENABLE"]
    #[inline(always)]
    pub fn call_enable(&mut self) -> CALL_ENABLE_W {
        CALL_ENABLE_W { w: self }
    }
}
