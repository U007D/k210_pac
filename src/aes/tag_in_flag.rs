#[doc = "Reader of register tag_in_flag"]
pub type R = crate::R<u32, super::TAG_IN_FLAG>;
#[doc = "Writer for register tag_in_flag"]
pub type W = crate::W<u32, super::TAG_IN_FLAG>;
#[doc = "Register tag_in_flag `reset()`'s with value 0"]
impl crate::ResetValue for super::TAG_IN_FLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "GCM tag can be written to gcm_in_tag when this flag is set"]
pub type TAG_IN_FLAG_A = super::data_in_flag::DATA_IN_FLAG_A;
#[doc = "Reader of field `tag_in_flag`"]
pub type TAG_IN_FLAG_R = crate::R<bool, super::data_in_flag::DATA_IN_FLAG_A>;
#[doc = "Write proxy for field `tag_in_flag`"]
pub struct TAG_IN_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_IN_FLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAG_IN_FLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cannot input"]
    #[inline(always)]
    pub fn cannot_input(self) -> &'a mut W {
        self.variant(super::data_in_flag::DATA_IN_FLAG_A::CANNOT_INPUT)
    }
    #[doc = "Can input"]
    #[inline(always)]
    pub fn can_input(self) -> &'a mut W {
        self.variant(super::data_in_flag::DATA_IN_FLAG_A::CAN_INPUT)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - GCM tag can be written to gcm_in_tag when this flag is set"]
    #[inline(always)]
    pub fn tag_in_flag(&self) -> TAG_IN_FLAG_R {
        TAG_IN_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GCM tag can be written to gcm_in_tag when this flag is set"]
    #[inline(always)]
    pub fn tag_in_flag(&mut self) -> TAG_IN_FLAG_W {
        TAG_IN_FLAG_W { w: self }
    }
}
