#[doc = "Reader of register tag_chk"]
pub type R = crate::R<u32, super::TAG_CHK>;
#[doc = "Writer for register tag_chk"]
pub type W = crate::W<u32, super::TAG_CHK>;
#[doc = "Register tag_chk `reset()`'s with value 0"]
impl crate::ResetValue for super::TAG_CHK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Tag check status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAG_CHK_A {
    #[doc = "0: Check not finished"]
    BUSY,
    #[doc = "1: Check failed"]
    FAIL,
    #[doc = "2: Check success"]
    SUCCESS,
}
impl From<TAG_CHK_A> for u8 {
    #[inline(always)]
    fn from(variant: TAG_CHK_A) -> Self {
        match variant {
            TAG_CHK_A::BUSY => 0,
            TAG_CHK_A::FAIL => 1,
            TAG_CHK_A::SUCCESS => 2,
        }
    }
}
#[doc = "Reader of field `tag_chk`"]
pub type TAG_CHK_R = crate::R<u8, TAG_CHK_A>;
impl TAG_CHK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TAG_CHK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TAG_CHK_A::BUSY),
            1 => Val(TAG_CHK_A::FAIL),
            2 => Val(TAG_CHK_A::SUCCESS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == TAG_CHK_A::BUSY
    }
    #[doc = "Checks if the value of the field is `FAIL`"]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == TAG_CHK_A::FAIL
    }
    #[doc = "Checks if the value of the field is `SUCCESS`"]
    #[inline(always)]
    pub fn is_success(&self) -> bool {
        *self == TAG_CHK_A::SUCCESS
    }
}
#[doc = "Write proxy for field `tag_chk`"]
pub struct TAG_CHK_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_CHK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAG_CHK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Check not finished"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(TAG_CHK_A::BUSY)
    }
    #[doc = "Check failed"]
    #[inline(always)]
    pub fn fail(self) -> &'a mut W {
        self.variant(TAG_CHK_A::FAIL)
    }
    #[doc = "Check success"]
    #[inline(always)]
    pub fn success(self) -> &'a mut W {
        self.variant(TAG_CHK_A::SUCCESS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Tag check status"]
    #[inline(always)]
    pub fn tag_chk(&self) -> TAG_CHK_R {
        TAG_CHK_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Tag check status"]
    #[inline(always)]
    pub fn tag_chk(&mut self) -> TAG_CHK_W {
        TAG_CHK_W { w: self }
    }
}
