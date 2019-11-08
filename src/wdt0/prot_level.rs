#[doc = "Reader of register prot_level"]
pub type R = crate::R<u32, super::PROT_LEVEL>;
#[doc = "Writer for register prot_level"]
pub type W = crate::W<u32, super::PROT_LEVEL>;
#[doc = "Register prot_level `reset()`'s with value 0"]
impl crate::ResetValue for super::PROT_LEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `prot_level`"]
pub type PROT_LEVEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `prot_level`"]
pub struct PROT_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - prot_level"]
    #[inline(always)]
    pub fn prot_level(&self) -> PROT_LEVEL_R {
        PROT_LEVEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - prot_level"]
    #[inline(always)]
    pub fn prot_level(&mut self) -> PROT_LEVEL_W {
        PROT_LEVEL_W { w: self }
    }
}
