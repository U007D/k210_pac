#[doc = "Reader of register data_out_flag"]
pub type R = crate::R<u32, super::DATA_OUT_FLAG>;
#[doc = "Writer for register data_out_flag"]
pub type W = crate::W<u32, super::DATA_OUT_FLAG>;
#[doc = "Register data_out_flag `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_OUT_FLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Data can be read from out_data when this flag is set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_OUT_FLAG_A {
    #[doc = "0: Data cannot output"]
    CANNOT_OUTPUT,
    #[doc = "1: Data can output"]
    CAN_OUTPUT,
}
impl From<DATA_OUT_FLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DATA_OUT_FLAG_A) -> Self {
        match variant {
            DATA_OUT_FLAG_A::CANNOT_OUTPUT => false,
            DATA_OUT_FLAG_A::CAN_OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `data_out_flag`"]
pub type DATA_OUT_FLAG_R = crate::R<bool, DATA_OUT_FLAG_A>;
impl DATA_OUT_FLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_OUT_FLAG_A {
        match self.bits {
            false => DATA_OUT_FLAG_A::CANNOT_OUTPUT,
            true => DATA_OUT_FLAG_A::CAN_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `CANNOT_OUTPUT`"]
    #[inline(always)]
    pub fn is_cannot_output(&self) -> bool {
        *self == DATA_OUT_FLAG_A::CANNOT_OUTPUT
    }
    #[doc = "Checks if the value of the field is `CAN_OUTPUT`"]
    #[inline(always)]
    pub fn is_can_output(&self) -> bool {
        *self == DATA_OUT_FLAG_A::CAN_OUTPUT
    }
}
#[doc = "Write proxy for field `data_out_flag`"]
pub struct DATA_OUT_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_OUT_FLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_OUT_FLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data cannot output"]
    #[inline(always)]
    pub fn cannot_output(self) -> &'a mut W {
        self.variant(DATA_OUT_FLAG_A::CANNOT_OUTPUT)
    }
    #[doc = "Data can output"]
    #[inline(always)]
    pub fn can_output(self) -> &'a mut W {
        self.variant(DATA_OUT_FLAG_A::CAN_OUTPUT)
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
    #[doc = "Bit 0 - Data can be read from out_data when this flag is set"]
    #[inline(always)]
    pub fn data_out_flag(&self) -> DATA_OUT_FLAG_R {
        DATA_OUT_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data can be read from out_data when this flag is set"]
    #[inline(always)]
    pub fn data_out_flag(&mut self) -> DATA_OUT_FLAG_W {
        DATA_OUT_FLAG_W { w: self }
    }
}
