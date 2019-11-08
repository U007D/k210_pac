#[doc = "Reader of register comp_param_2"]
pub type R = crate::R<u32, super::COMP_PARAM_2>;
#[doc = "Writer for register comp_param_2"]
pub type W = crate::W<u32, super::COMP_PARAM_2>;
#[doc = "Register comp_param_2 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP_PARAM_2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `cnt_rst`"]
pub type CNT_RST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `cnt_rst`"]
pub struct CNT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_RST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - cnt_rst"]
    #[inline(always)]
    pub fn cnt_rst(&self) -> CNT_RST_R {
        CNT_RST_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - cnt_rst"]
    #[inline(always)]
    pub fn cnt_rst(&mut self) -> CNT_RST_W {
        CNT_RST_W { w: self }
    }
}
