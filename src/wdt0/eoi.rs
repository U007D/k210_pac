#[doc = "Reader of register eoi"]
pub type R = crate::R<u32, super::EOI>;
#[doc = "Writer for register eoi"]
pub type W = crate::W<u32, super::EOI>;
#[doc = "Register eoi `reset()`'s with value 0"]
impl crate::ResetValue for super::EOI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `eoi`"]
pub type EOI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `eoi`"]
pub struct EOI_W<'a> {
    w: &'a mut W,
}
impl<'a> EOI_W<'a> {
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
    #[doc = "Bit 0 - eoi"]
    #[inline(always)]
    pub fn eoi(&self) -> EOI_R {
        EOI_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eoi"]
    #[inline(always)]
    pub fn eoi(&mut self) -> EOI_W {
        EOI_W { w: self }
    }
}
