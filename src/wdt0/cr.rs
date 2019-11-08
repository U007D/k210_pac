#[doc = "Reader of register cr"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register cr"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register cr `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `enable`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `enable`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "rmod\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMOD_A {
    #[doc = "0: RESET"]
    RESET,
    #[doc = "1: INTERRUPT"]
    INTERRUPT,
}
impl From<RMOD_A> for bool {
    #[inline(always)]
    fn from(variant: RMOD_A) -> Self {
        match variant {
            RMOD_A::RESET => false,
            RMOD_A::INTERRUPT => true,
        }
    }
}
#[doc = "Reader of field `rmod`"]
pub type RMOD_R = crate::R<bool, RMOD_A>;
impl RMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMOD_A {
        match self.bits {
            false => RMOD_A::RESET,
            true => RMOD_A::INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RMOD_A::RESET
    }
    #[doc = "Checks if the value of the field is `INTERRUPT`"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == RMOD_A::INTERRUPT
    }
}
#[doc = "Write proxy for field `rmod`"]
pub struct RMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RMOD_A::RESET)
    }
    #[doc = "INTERRUPT"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut W {
        self.variant(RMOD_A::INTERRUPT)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `rpl`"]
pub type RPL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `rpl`"]
pub struct RPL_W<'a> {
    w: &'a mut W,
}
impl<'a> RPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - rmod"]
    #[inline(always)]
    pub fn rmod(&self) -> RMOD_R {
        RMOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - rpl"]
    #[inline(always)]
    pub fn rpl(&self) -> RPL_R {
        RPL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - rmod"]
    #[inline(always)]
    pub fn rmod(&mut self) -> RMOD_W {
        RMOD_W { w: self }
    }
    #[doc = "Bits 2:4 - rpl"]
    #[inline(always)]
    pub fn rpl(&mut self) -> RPL_W {
        RPL_W { w: self }
    }
}
