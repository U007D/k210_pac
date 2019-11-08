#[doc = "Reader of register data_in_flag"]
pub type R = crate::R<u32, super::DATA_IN_FLAG>;
#[doc = "Writer for register data_in_flag"]
pub type W = crate::W<u32, super::DATA_IN_FLAG>;
#[doc = "Register data_in_flag `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_IN_FLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Data can be written to text_data or aad_data when this flag is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_IN_FLAG_A {
    #[doc = "0: Cannot input"]
    CANNOT_INPUT,
    #[doc = "1: Can input"]
    CAN_INPUT,
}
impl From<DATA_IN_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_IN_FLAG_A) -> Self {
        match variant {
            DATA_IN_FLAG_A::CANNOT_INPUT => false,
            DATA_IN_FLAG_A::CAN_INPUT => true,
        }
    }
}
#[doc = "Reader of field `data_in_flag`"]
pub type DATA_IN_FLAG_R = crate::R<bool, DATA_IN_FLAG_A>;
impl DATA_IN_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_IN_FLAG_A {
        match self.bits {
            false => DATA_IN_FLAG_A::CANNOT_INPUT,
            true => DATA_IN_FLAG_A::CAN_INPUT,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_INPUT`"]
    #[inline(always)]
    pub fn is_cannot_input(&self) -> bool {
        *self == DATA_IN_FLAG_A::CANNOT_INPUT
    }
    #[doc = "Checks if the value of the field is `CAN_INPUT`"]
    #[inline(always)]
    pub fn is_can_input(&self) -> bool {
        *self == DATA_IN_FLAG_A::CAN_INPUT
    }
}
#[doc = "Write proxy for field `data_in_flag`"]
pub struct DATA_IN_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_IN_FLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_IN_FLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cannot input"]
    #[inline(always)]
    pub fn cannot_input(self) -> &'a mut W {
        self.variant(DATA_IN_FLAG_A::CANNOT_INPUT)
    }
    #[doc = "Can input"]
    #[inline(always)]
    pub fn can_input(self) -> &'a mut W {
        self.variant(DATA_IN_FLAG_A::CAN_INPUT)
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
    #[doc = "Bit 0 - Data can be written to text_data or aad_data when this flag is set"]
    #[inline(always)]
    pub fn data_in_flag(&self) -> DATA_IN_FLAG_R {
        DATA_IN_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data can be written to text_data or aad_data when this flag is set"]
    #[inline(always)]
    pub fn data_in_flag(&mut self) -> DATA_IN_FLAG_W {
        DATA_IN_FLAG_W { w: self }
    }
}
