#[doc = "Reader of register post_fir1_coef[%s]"]
pub type R = crate::R<u32, super::POST_FIR1_COEF>;
#[doc = "Writer for register post_fir1_coef[%s]"]
pub type W = crate::W<u32, super::POST_FIR1_COEF>;
#[doc = "Register post_fir1_coef[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::POST_FIR1_COEF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `tap0`"]
pub type TAP0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tap0`"]
pub struct TAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> TAP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `tap1`"]
pub type TAP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tap1`"]
pub struct TAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> TAP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Tap 0"]
    #[inline(always)]
    pub fn tap0(&self) -> TAP0_R {
        TAP0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Tap 1"]
    #[inline(always)]
    pub fn tap1(&self) -> TAP1_R {
        TAP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Tap 0"]
    #[inline(always)]
    pub fn tap0(&mut self) -> TAP0_W {
        TAP0_W { w: self }
    }
    #[doc = "Bits 16:31 - Tap 1"]
    #[inline(always)]
    pub fn tap1(&mut self) -> TAP1_W {
        TAP1_W { w: self }
    }
}
