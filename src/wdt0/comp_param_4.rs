#[doc = "Reader of register comp_param_4"]
pub type R = crate::R<u32, super::COMP_PARAM_4>;
#[doc = "Writer for register comp_param_4"]
pub type W = crate::W<u32, super::COMP_PARAM_4>;
#[doc = "Register comp_param_4 `reset()`'s with value 0"]
impl crate::ResetValue for super::COMP_PARAM_4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `user_top_init_max`"]
pub type USER_TOP_INIT_MAX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `user_top_init_max`"]
pub struct USER_TOP_INIT_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_TOP_INIT_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - user_top_init_max"]
    #[inline(always)]
    pub fn user_top_init_max(&self) -> USER_TOP_INIT_MAX_R {
        USER_TOP_INIT_MAX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - user_top_init_max"]
    #[inline(always)]
    pub fn user_top_init_max(&mut self) -> USER_TOP_INIT_MAX_W {
        USER_TOP_INIT_MAX_W { w: self }
    }
}
