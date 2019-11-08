#[doc = "Reader of register chen"]
pub type R = crate::R<u64, super::CHEN>;
#[doc = "Writer for register chen"]
pub type W = crate::W<u64, super::CHEN>;
#[doc = "Register chen `reset()`'s with value 0"]
impl crate::ResetValue for super::CHEN {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ch1_en`"]
pub type CH1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch1_en`"]
pub struct CH1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_EN_W<'a> {
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
#[doc = "Reader of field `ch2_en`"]
pub type CH2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch2_en`"]
pub struct CH2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u64) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ch3_en`"]
pub type CH3_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch3_en`"]
pub struct CH3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u64) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ch4_en`"]
pub type CH4_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch4_en`"]
pub struct CH4_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u64) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ch5_en`"]
pub type CH5_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch5_en`"]
pub struct CH5_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u64) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ch6_en`"]
pub type CH6_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch6_en`"]
pub struct CH6_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u64) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `ch1_en_we`"]
pub type CH1_EN_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch1_en_we`"]
pub struct CH1_EN_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_EN_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u64) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ch2_en_we`"]
pub type CH2_EN_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch2_en_we`"]
pub struct CH2_EN_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_EN_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u64) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ch3_en_we`"]
pub type CH3_EN_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch3_en_we`"]
pub struct CH3_EN_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_EN_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u64) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ch4_en_we`"]
pub type CH4_EN_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch4_en_we`"]
pub struct CH4_EN_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_EN_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u64) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ch5_en_we`"]
pub type CH5_EN_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch5_en_we`"]
pub struct CH5_EN_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_EN_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u64) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ch6_en_we`"]
pub type CH6_EN_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch6_en_we`"]
pub struct CH6_EN_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_EN_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u64) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ch1_susp`"]
pub type CH1_SUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch1_susp`"]
pub struct CH1_SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_SUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u64) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ch2_susp`"]
pub type CH2_SUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch2_susp`"]
pub struct CH2_SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_SUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u64) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ch3_susp`"]
pub type CH3_SUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch3_susp`"]
pub struct CH3_SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_SUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u64) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `ch4_susp`"]
pub type CH4_SUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch4_susp`"]
pub struct CH4_SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_SUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u64) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ch5_susp`"]
pub type CH5_SUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch5_susp`"]
pub struct CH5_SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_SUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u64) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `ch6_susp`"]
pub type CH6_SUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch6_susp`"]
pub struct CH6_SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_SUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u64) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ch1_susp_we`"]
pub type CH1_SUSP_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch1_susp_we`"]
pub struct CH1_SUSP_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_SUSP_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u64) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `ch2_susp_we`"]
pub type CH2_SUSP_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch2_susp_we`"]
pub struct CH2_SUSP_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_SUSP_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u64) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `ch3_susp_we`"]
pub type CH3_SUSP_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch3_susp_we`"]
pub struct CH3_SUSP_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_SUSP_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u64) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `ch4_susp_we`"]
pub type CH4_SUSP_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch4_susp_we`"]
pub struct CH4_SUSP_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_SUSP_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u64) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `ch5_susp_we`"]
pub type CH5_SUSP_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch5_susp_we`"]
pub struct CH5_SUSP_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_SUSP_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u64) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `ch6_susp_we`"]
pub type CH6_SUSP_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch6_susp_we`"]
pub struct CH6_SUSP_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_SUSP_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u64) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `ch1_abort`"]
pub type CH1_ABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch1_abort`"]
pub struct CH1_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 32)) | (((value as u64) & 0x01) << 32);
        self.w
    }
}
#[doc = "Reader of field `ch2_abort`"]
pub type CH2_ABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch2_abort`"]
pub struct CH2_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 33)) | (((value as u64) & 0x01) << 33);
        self.w
    }
}
#[doc = "Reader of field `ch3_abort`"]
pub type CH3_ABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch3_abort`"]
pub struct CH3_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 34)) | (((value as u64) & 0x01) << 34);
        self.w
    }
}
#[doc = "Reader of field `ch4_abort`"]
pub type CH4_ABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch4_abort`"]
pub struct CH4_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 35)) | (((value as u64) & 0x01) << 35);
        self.w
    }
}
#[doc = "Reader of field `ch5_abort`"]
pub type CH5_ABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch5_abort`"]
pub struct CH5_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 36)) | (((value as u64) & 0x01) << 36);
        self.w
    }
}
#[doc = "Reader of field `ch6_abort`"]
pub type CH6_ABORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch6_abort`"]
pub struct CH6_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_ABORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 37)) | (((value as u64) & 0x01) << 37);
        self.w
    }
}
#[doc = "Reader of field `ch1_abort_we`"]
pub type CH1_ABORT_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch1_abort_we`"]
pub struct CH1_ABORT_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_ABORT_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 40)) | (((value as u64) & 0x01) << 40);
        self.w
    }
}
#[doc = "Reader of field `ch2_abort_we`"]
pub type CH2_ABORT_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch2_abort_we`"]
pub struct CH2_ABORT_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_ABORT_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 41)) | (((value as u64) & 0x01) << 41);
        self.w
    }
}
#[doc = "Reader of field `ch3_abort_we`"]
pub type CH3_ABORT_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch3_abort_we`"]
pub struct CH3_ABORT_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_ABORT_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 42)) | (((value as u64) & 0x01) << 42);
        self.w
    }
}
#[doc = "Reader of field `ch4_abort_we`"]
pub type CH4_ABORT_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch4_abort_we`"]
pub struct CH4_ABORT_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_ABORT_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 43)) | (((value as u64) & 0x01) << 43);
        self.w
    }
}
#[doc = "Reader of field `ch5_abort_we`"]
pub type CH5_ABORT_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch5_abort_we`"]
pub struct CH5_ABORT_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_ABORT_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 44)) | (((value as u64) & 0x01) << 44);
        self.w
    }
}
#[doc = "Reader of field `ch6_abort_we`"]
pub type CH6_ABORT_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch6_abort_we`"]
pub struct CH6_ABORT_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_ABORT_WE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 45)) | (((value as u64) & 0x01) << 45);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline(always)]
    pub fn ch1_en(&self) -> CH1_EN_R {
        CH1_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable channel 2"]
    #[inline(always)]
    pub fn ch2_en(&self) -> CH2_EN_R {
        CH2_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable channel 3"]
    #[inline(always)]
    pub fn ch3_en(&self) -> CH3_EN_R {
        CH3_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable channel 4"]
    #[inline(always)]
    pub fn ch4_en(&self) -> CH4_EN_R {
        CH4_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable channel 5"]
    #[inline(always)]
    pub fn ch5_en(&self) -> CH5_EN_R {
        CH5_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable channel 6"]
    #[inline(always)]
    pub fn ch6_en(&self) -> CH6_EN_R {
        CH6_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write enable channel 1"]
    #[inline(always)]
    pub fn ch1_en_we(&self) -> CH1_EN_WE_R {
        CH1_EN_WE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write enable channel 2"]
    #[inline(always)]
    pub fn ch2_en_we(&self) -> CH2_EN_WE_R {
        CH2_EN_WE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write enable channel 3"]
    #[inline(always)]
    pub fn ch3_en_we(&self) -> CH3_EN_WE_R {
        CH3_EN_WE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write enable channel 4"]
    #[inline(always)]
    pub fn ch4_en_we(&self) -> CH4_EN_WE_R {
        CH4_EN_WE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write enable channel 5"]
    #[inline(always)]
    pub fn ch5_en_we(&self) -> CH5_EN_WE_R {
        CH5_EN_WE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write enable channel 6"]
    #[inline(always)]
    pub fn ch6_en_we(&self) -> CH6_EN_WE_R {
        CH6_EN_WE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Suspend request channel 1"]
    #[inline(always)]
    pub fn ch1_susp(&self) -> CH1_SUSP_R {
        CH1_SUSP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Suspend request channel 2"]
    #[inline(always)]
    pub fn ch2_susp(&self) -> CH2_SUSP_R {
        CH2_SUSP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Suspend request channel 3"]
    #[inline(always)]
    pub fn ch3_susp(&self) -> CH3_SUSP_R {
        CH3_SUSP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Suspend request channel 4"]
    #[inline(always)]
    pub fn ch4_susp(&self) -> CH4_SUSP_R {
        CH4_SUSP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Suspend request channel 5"]
    #[inline(always)]
    pub fn ch5_susp(&self) -> CH5_SUSP_R {
        CH5_SUSP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Suspend request channel 6"]
    #[inline(always)]
    pub fn ch6_susp(&self) -> CH6_SUSP_R {
        CH6_SUSP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable write to ch1_susp bit"]
    #[inline(always)]
    pub fn ch1_susp_we(&self) -> CH1_SUSP_WE_R {
        CH1_SUSP_WE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable write to ch2_susp bit"]
    #[inline(always)]
    pub fn ch2_susp_we(&self) -> CH2_SUSP_WE_R {
        CH2_SUSP_WE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable write to ch3_susp bit"]
    #[inline(always)]
    pub fn ch3_susp_we(&self) -> CH3_SUSP_WE_R {
        CH3_SUSP_WE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable write to ch4_susp bit"]
    #[inline(always)]
    pub fn ch4_susp_we(&self) -> CH4_SUSP_WE_R {
        CH4_SUSP_WE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable write to ch5_susp bit"]
    #[inline(always)]
    pub fn ch5_susp_we(&self) -> CH5_SUSP_WE_R {
        CH5_SUSP_WE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable write to ch6_susp bit"]
    #[inline(always)]
    pub fn ch6_susp_we(&self) -> CH6_SUSP_WE_R {
        CH6_SUSP_WE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 32 - Abort request channel 1"]
    #[inline(always)]
    pub fn ch1_abort(&self) -> CH1_ABORT_R {
        CH1_ABORT_R::new(((self.bits >> 32) & 0x01) != 0)
    }
    #[doc = "Bit 33 - Abort request channel 2"]
    #[inline(always)]
    pub fn ch2_abort(&self) -> CH2_ABORT_R {
        CH2_ABORT_R::new(((self.bits >> 33) & 0x01) != 0)
    }
    #[doc = "Bit 34 - Abort request channel 3"]
    #[inline(always)]
    pub fn ch3_abort(&self) -> CH3_ABORT_R {
        CH3_ABORT_R::new(((self.bits >> 34) & 0x01) != 0)
    }
    #[doc = "Bit 35 - Abort request channel 4"]
    #[inline(always)]
    pub fn ch4_abort(&self) -> CH4_ABORT_R {
        CH4_ABORT_R::new(((self.bits >> 35) & 0x01) != 0)
    }
    #[doc = "Bit 36 - Abort request channel 5"]
    #[inline(always)]
    pub fn ch5_abort(&self) -> CH5_ABORT_R {
        CH5_ABORT_R::new(((self.bits >> 36) & 0x01) != 0)
    }
    #[doc = "Bit 37 - Abort request channel 6"]
    #[inline(always)]
    pub fn ch6_abort(&self) -> CH6_ABORT_R {
        CH6_ABORT_R::new(((self.bits >> 37) & 0x01) != 0)
    }
    #[doc = "Bit 40 - Enable write to ch1_abort bit"]
    #[inline(always)]
    pub fn ch1_abort_we(&self) -> CH1_ABORT_WE_R {
        CH1_ABORT_WE_R::new(((self.bits >> 40) & 0x01) != 0)
    }
    #[doc = "Bit 41 - Enable write to ch2_abort bit"]
    #[inline(always)]
    pub fn ch2_abort_we(&self) -> CH2_ABORT_WE_R {
        CH2_ABORT_WE_R::new(((self.bits >> 41) & 0x01) != 0)
    }
    #[doc = "Bit 42 - Enable write to ch3_abort bit"]
    #[inline(always)]
    pub fn ch3_abort_we(&self) -> CH3_ABORT_WE_R {
        CH3_ABORT_WE_R::new(((self.bits >> 42) & 0x01) != 0)
    }
    #[doc = "Bit 43 - Enable write to ch4_abort bit"]
    #[inline(always)]
    pub fn ch4_abort_we(&self) -> CH4_ABORT_WE_R {
        CH4_ABORT_WE_R::new(((self.bits >> 43) & 0x01) != 0)
    }
    #[doc = "Bit 44 - Enable write to ch5_abort bit"]
    #[inline(always)]
    pub fn ch5_abort_we(&self) -> CH5_ABORT_WE_R {
        CH5_ABORT_WE_R::new(((self.bits >> 44) & 0x01) != 0)
    }
    #[doc = "Bit 45 - Enable write to ch6_abort bit"]
    #[inline(always)]
    pub fn ch6_abort_we(&self) -> CH6_ABORT_WE_R {
        CH6_ABORT_WE_R::new(((self.bits >> 45) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline(always)]
    pub fn ch1_en(&mut self) -> CH1_EN_W {
        CH1_EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable channel 2"]
    #[inline(always)]
    pub fn ch2_en(&mut self) -> CH2_EN_W {
        CH2_EN_W { w: self }
    }
    #[doc = "Bit 2 - Enable channel 3"]
    #[inline(always)]
    pub fn ch3_en(&mut self) -> CH3_EN_W {
        CH3_EN_W { w: self }
    }
    #[doc = "Bit 3 - Enable channel 4"]
    #[inline(always)]
    pub fn ch4_en(&mut self) -> CH4_EN_W {
        CH4_EN_W { w: self }
    }
    #[doc = "Bit 4 - Enable channel 5"]
    #[inline(always)]
    pub fn ch5_en(&mut self) -> CH5_EN_W {
        CH5_EN_W { w: self }
    }
    #[doc = "Bit 5 - Enable channel 6"]
    #[inline(always)]
    pub fn ch6_en(&mut self) -> CH6_EN_W {
        CH6_EN_W { w: self }
    }
    #[doc = "Bit 8 - Write enable channel 1"]
    #[inline(always)]
    pub fn ch1_en_we(&mut self) -> CH1_EN_WE_W {
        CH1_EN_WE_W { w: self }
    }
    #[doc = "Bit 9 - Write enable channel 2"]
    #[inline(always)]
    pub fn ch2_en_we(&mut self) -> CH2_EN_WE_W {
        CH2_EN_WE_W { w: self }
    }
    #[doc = "Bit 10 - Write enable channel 3"]
    #[inline(always)]
    pub fn ch3_en_we(&mut self) -> CH3_EN_WE_W {
        CH3_EN_WE_W { w: self }
    }
    #[doc = "Bit 11 - Write enable channel 4"]
    #[inline(always)]
    pub fn ch4_en_we(&mut self) -> CH4_EN_WE_W {
        CH4_EN_WE_W { w: self }
    }
    #[doc = "Bit 12 - Write enable channel 5"]
    #[inline(always)]
    pub fn ch5_en_we(&mut self) -> CH5_EN_WE_W {
        CH5_EN_WE_W { w: self }
    }
    #[doc = "Bit 13 - Write enable channel 6"]
    #[inline(always)]
    pub fn ch6_en_we(&mut self) -> CH6_EN_WE_W {
        CH6_EN_WE_W { w: self }
    }
    #[doc = "Bit 16 - Suspend request channel 1"]
    #[inline(always)]
    pub fn ch1_susp(&mut self) -> CH1_SUSP_W {
        CH1_SUSP_W { w: self }
    }
    #[doc = "Bit 17 - Suspend request channel 2"]
    #[inline(always)]
    pub fn ch2_susp(&mut self) -> CH2_SUSP_W {
        CH2_SUSP_W { w: self }
    }
    #[doc = "Bit 18 - Suspend request channel 3"]
    #[inline(always)]
    pub fn ch3_susp(&mut self) -> CH3_SUSP_W {
        CH3_SUSP_W { w: self }
    }
    #[doc = "Bit 19 - Suspend request channel 4"]
    #[inline(always)]
    pub fn ch4_susp(&mut self) -> CH4_SUSP_W {
        CH4_SUSP_W { w: self }
    }
    #[doc = "Bit 20 - Suspend request channel 5"]
    #[inline(always)]
    pub fn ch5_susp(&mut self) -> CH5_SUSP_W {
        CH5_SUSP_W { w: self }
    }
    #[doc = "Bit 21 - Suspend request channel 6"]
    #[inline(always)]
    pub fn ch6_susp(&mut self) -> CH6_SUSP_W {
        CH6_SUSP_W { w: self }
    }
    #[doc = "Bit 24 - Enable write to ch1_susp bit"]
    #[inline(always)]
    pub fn ch1_susp_we(&mut self) -> CH1_SUSP_WE_W {
        CH1_SUSP_WE_W { w: self }
    }
    #[doc = "Bit 25 - Enable write to ch2_susp bit"]
    #[inline(always)]
    pub fn ch2_susp_we(&mut self) -> CH2_SUSP_WE_W {
        CH2_SUSP_WE_W { w: self }
    }
    #[doc = "Bit 26 - Enable write to ch3_susp bit"]
    #[inline(always)]
    pub fn ch3_susp_we(&mut self) -> CH3_SUSP_WE_W {
        CH3_SUSP_WE_W { w: self }
    }
    #[doc = "Bit 27 - Enable write to ch4_susp bit"]
    #[inline(always)]
    pub fn ch4_susp_we(&mut self) -> CH4_SUSP_WE_W {
        CH4_SUSP_WE_W { w: self }
    }
    #[doc = "Bit 28 - Enable write to ch5_susp bit"]
    #[inline(always)]
    pub fn ch5_susp_we(&mut self) -> CH5_SUSP_WE_W {
        CH5_SUSP_WE_W { w: self }
    }
    #[doc = "Bit 29 - Enable write to ch6_susp bit"]
    #[inline(always)]
    pub fn ch6_susp_we(&mut self) -> CH6_SUSP_WE_W {
        CH6_SUSP_WE_W { w: self }
    }
    #[doc = "Bit 32 - Abort request channel 1"]
    #[inline(always)]
    pub fn ch1_abort(&mut self) -> CH1_ABORT_W {
        CH1_ABORT_W { w: self }
    }
    #[doc = "Bit 33 - Abort request channel 2"]
    #[inline(always)]
    pub fn ch2_abort(&mut self) -> CH2_ABORT_W {
        CH2_ABORT_W { w: self }
    }
    #[doc = "Bit 34 - Abort request channel 3"]
    #[inline(always)]
    pub fn ch3_abort(&mut self) -> CH3_ABORT_W {
        CH3_ABORT_W { w: self }
    }
    #[doc = "Bit 35 - Abort request channel 4"]
    #[inline(always)]
    pub fn ch4_abort(&mut self) -> CH4_ABORT_W {
        CH4_ABORT_W { w: self }
    }
    #[doc = "Bit 36 - Abort request channel 5"]
    #[inline(always)]
    pub fn ch5_abort(&mut self) -> CH5_ABORT_W {
        CH5_ABORT_W { w: self }
    }
    #[doc = "Bit 37 - Abort request channel 6"]
    #[inline(always)]
    pub fn ch6_abort(&mut self) -> CH6_ABORT_W {
        CH6_ABORT_W { w: self }
    }
    #[doc = "Bit 40 - Enable write to ch1_abort bit"]
    #[inline(always)]
    pub fn ch1_abort_we(&mut self) -> CH1_ABORT_WE_W {
        CH1_ABORT_WE_W { w: self }
    }
    #[doc = "Bit 41 - Enable write to ch2_abort bit"]
    #[inline(always)]
    pub fn ch2_abort_we(&mut self) -> CH2_ABORT_WE_W {
        CH2_ABORT_WE_W { w: self }
    }
    #[doc = "Bit 42 - Enable write to ch3_abort bit"]
    #[inline(always)]
    pub fn ch3_abort_we(&mut self) -> CH3_ABORT_WE_W {
        CH3_ABORT_WE_W { w: self }
    }
    #[doc = "Bit 43 - Enable write to ch4_abort bit"]
    #[inline(always)]
    pub fn ch4_abort_we(&mut self) -> CH4_ABORT_WE_W {
        CH4_ABORT_WE_W { w: self }
    }
    #[doc = "Bit 44 - Enable write to ch5_abort bit"]
    #[inline(always)]
    pub fn ch5_abort_we(&mut self) -> CH5_ABORT_WE_W {
        CH5_ABORT_WE_W { w: self }
    }
    #[doc = "Bit 45 - Enable write to ch6_abort bit"]
    #[inline(always)]
    pub fn ch6_abort_we(&mut self) -> CH6_ABORT_WE_W {
        CH6_ABORT_WE_W { w: self }
    }
}
