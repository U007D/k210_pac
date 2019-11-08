#[doc = "Reader of register rcr"]
pub type R = crate::R<u32, super::RCR>;
#[doc = "Writer for register rcr"]
pub type W = crate::W<u32, super::RCR>;
#[doc = "Register rcr `reset()`'s with value 0"]
impl crate::ResetValue for super::RCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Desired data resolution of receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WLEN_A {
    #[doc = "0: Ignore the word length"]
    IGNORE,
    #[doc = "1: 12-bit data resolution of the receiver"]
    RESOLUTION12,
    #[doc = "2: 16-bit data resolution of the receiver"]
    RESOLUTION16,
    #[doc = "3: 20-bit data resolution of the receiver"]
    RESOLUTION20,
    #[doc = "4: 24-bit data resolution of the receiver"]
    RESOLUTION24,
    #[doc = "5: 32-bit data resolution of the receiver"]
    RESOLUTION32,
}
impl From<WLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WLEN_A) -> Self {
        match variant {
            WLEN_A::IGNORE => 0,
            WLEN_A::RESOLUTION12 => 1,
            WLEN_A::RESOLUTION16 => 2,
            WLEN_A::RESOLUTION20 => 3,
            WLEN_A::RESOLUTION24 => 4,
            WLEN_A::RESOLUTION32 => 5,
        }
    }
}
#[doc = "Reader of field `wlen`"]
pub type WLEN_R = crate::R<u8, WLEN_A>;
impl WLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WLEN_A::IGNORE),
            1 => Val(WLEN_A::RESOLUTION12),
            2 => Val(WLEN_A::RESOLUTION16),
            3 => Val(WLEN_A::RESOLUTION20),
            4 => Val(WLEN_A::RESOLUTION24),
            5 => Val(WLEN_A::RESOLUTION32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == WLEN_A::IGNORE
    }
    #[doc = "Checks if the value of the field is `RESOLUTION12`"]
    #[inline(always)]
    pub fn is_resolution12(&self) -> bool {
        *self == WLEN_A::RESOLUTION12
    }
    #[doc = "Checks if the value of the field is `RESOLUTION16`"]
    #[inline(always)]
    pub fn is_resolution16(&self) -> bool {
        *self == WLEN_A::RESOLUTION16
    }
    #[doc = "Checks if the value of the field is `RESOLUTION20`"]
    #[inline(always)]
    pub fn is_resolution20(&self) -> bool {
        *self == WLEN_A::RESOLUTION20
    }
    #[doc = "Checks if the value of the field is `RESOLUTION24`"]
    #[inline(always)]
    pub fn is_resolution24(&self) -> bool {
        *self == WLEN_A::RESOLUTION24
    }
    #[doc = "Checks if the value of the field is `RESOLUTION32`"]
    #[inline(always)]
    pub fn is_resolution32(&self) -> bool {
        *self == WLEN_A::RESOLUTION32
    }
}
#[doc = "Write proxy for field `wlen`"]
pub struct WLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Ignore the word length"]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(WLEN_A::IGNORE)
    }
    #[doc = "12-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution12(self) -> &'a mut W {
        self.variant(WLEN_A::RESOLUTION12)
    }
    #[doc = "16-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution16(self) -> &'a mut W {
        self.variant(WLEN_A::RESOLUTION16)
    }
    #[doc = "20-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution20(self) -> &'a mut W {
        self.variant(WLEN_A::RESOLUTION20)
    }
    #[doc = "24-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution24(self) -> &'a mut W {
        self.variant(WLEN_A::RESOLUTION24)
    }
    #[doc = "32-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution32(self) -> &'a mut W {
        self.variant(WLEN_A::RESOLUTION32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Desired data resolution of receiver"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Desired data resolution of receiver"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W { w: self }
    }
}
