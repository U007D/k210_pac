#[doc = "Reader of register comp_param_3"]
pub type R = crate::R<u32, super::COMP_PARAM_3>;
#[doc = "Writer for register comp_param_3"]
pub type W = crate::W<u32, super::COMP_PARAM_3>;
#[doc = "Register comp_param_3 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP_PARAM_3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `top_rst`"]
pub type TOP_RST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `top_rst`"]
pub struct TOP_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_RST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - top_rst"]
    #[inline(always)]
    pub fn top_rst(&self) -> TOP_RST_R {
        TOP_RST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - top_rst"]
    #[inline(always)]
    pub fn top_rst(&mut self) -> TOP_RST_W {
        TOP_RST_W { w: self }
    }
}
