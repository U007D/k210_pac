#[doc = "Reader of register axi"]
pub type R = crate::R<u32, super::AXI>;
#[doc = "Writer for register axi"]
pub type W = crate::W<u32, super::AXI>;
#[doc = "Register axi `reset()`'s with value 0"]
impl crate::ResetValue for super::AXI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GM_MLEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GM_MLEN_A {
    #[doc = "0: GM_MLEN_1BYTE"]
    BYTE1,
    #[doc = "3: GM_MLEN_4BYTE"]
    BYTE4,
}
impl From<GM_MLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: GM_MLEN_A) -> Self {
        match variant {
            GM_MLEN_A::BYTE1 => 0,
            GM_MLEN_A::BYTE4 => 3,
        }
    }
}
#[doc = "Reader of field `gm_mlen`"]
pub type GM_MLEN_R = crate::R<u8, GM_MLEN_A>;
impl GM_MLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GM_MLEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GM_MLEN_A::BYTE1),
            3 => Val(GM_MLEN_A::BYTE4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE1`"]
    #[inline(always)]
    pub fn is_byte1(&self) -> bool {
        *self == GM_MLEN_A::BYTE1
    }
    #[doc = "Checks if the value of the field is `BYTE4`"]
    #[inline(always)]
    pub fn is_byte4(&self) -> bool {
        *self == GM_MLEN_A::BYTE4
    }
}
#[doc = "Write proxy for field `gm_mlen`"]
pub struct GM_MLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GM_MLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GM_MLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "GM_MLEN_1BYTE"]
    #[inline(always)]
    pub fn byte1(self) -> &'a mut W {
        self.variant(GM_MLEN_A::BYTE1)
    }
    #[doc = "GM_MLEN_4BYTE"]
    #[inline(always)]
    pub fn byte4(self) -> &'a mut W {
        self.variant(GM_MLEN_A::BYTE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GM_MLEN"]
    #[inline(always)]
    pub fn gm_mlen(&self) -> GM_MLEN_R {
        GM_MLEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GM_MLEN"]
    #[inline(always)]
    pub fn gm_mlen(&mut self) -> GM_MLEN_W {
        GM_MLEN_W { w: self }
    }
}
