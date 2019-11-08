#[doc = "Reader of register tcr"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register tcr"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register tcr `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Desired data resolution of transmitter"]
pub type WLEN_A = super::rcr::WLEN_A;
#[doc = "Reader of field `wlen`"]
pub type WLEN_R = crate::R<u8, super::rcr::WLEN_A>;
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
        self.variant(super::rcr::WLEN_A::IGNORE)
    }
    #[doc = "12-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution12(self) -> &'a mut W {
        self.variant(super::rcr::WLEN_A::RESOLUTION12)
    }
    #[doc = "16-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution16(self) -> &'a mut W {
        self.variant(super::rcr::WLEN_A::RESOLUTION16)
    }
    #[doc = "20-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution20(self) -> &'a mut W {
        self.variant(super::rcr::WLEN_A::RESOLUTION20)
    }
    #[doc = "24-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution24(self) -> &'a mut W {
        self.variant(super::rcr::WLEN_A::RESOLUTION24)
    }
    #[doc = "32-bit data resolution of the receiver"]
    #[inline(always)]
    pub fn resolution32(self) -> &'a mut W {
        self.variant(super::rcr::WLEN_A::RESOLUTION32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Desired data resolution of transmitter"]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Desired data resolution of transmitter"]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W { w: self }
    }
}
