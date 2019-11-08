#[doc = "Reader of register llp"]
pub type R = crate::R<u64, super::LLP>;
#[doc = "Writer for register llp"]
pub type W = crate::W<u64, super::LLP>;
#[doc = "Register llp `reset()`'s with value 0"]
impl crate::ResetValue for super::LLP {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LLI master select"]
pub type LMS_A = super::ctl::SMS_A;
#[doc = "Reader of field `lms`"]
pub type LMS_R = crate::R<bool, super::ctl::SMS_A>;
#[doc = "Write proxy for field `lms`"]
pub struct LMS_W<'a> {
    w: &'a mut W,
}
impl<'a> LMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LMS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AXI master 1"]
    #[inline(always)]
    pub fn axi_master_1(self) -> &'a mut W {
        self.variant(super::ctl::SMS_A::AXI_MASTER_1)
    }
    #[doc = "AXI master 2"]
    #[inline(always)]
    pub fn axi_master_2(self) -> &'a mut W {
        self.variant(super::ctl::SMS_A::AXI_MASTER_2)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u64) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `loc`"]
pub type LOC_R = crate::R<u64, u64>;
#[doc = "Write proxy for field `loc`"]
pub struct LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> LOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u64) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff_ffff_ffff << 6))
            | (((value as u64) & 0x03ff_ffff_ffff_ffff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LLI master select"]
    #[inline(always)]
    pub fn lms(&self) -> LMS_R {
        LMS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 6:63 - Starting address memeory of LLI block"]
    #[inline(always)]
    pub fn loc(&self) -> LOC_R {
        LOC_R::new(((self.bits >> 6) & 0x03ff_ffff_ffff_ffff) as u64)
    }
}
impl W {
    #[doc = "Bit 0 - LLI master select"]
    #[inline(always)]
    pub fn lms(&mut self) -> LMS_W {
        LMS_W { w: self }
    }
    #[doc = "Bits 6:63 - Starting address memeory of LLI block"]
    #[inline(always)]
    pub fn loc(&mut self) -> LOC_W {
        LOC_W { w: self }
    }
}
