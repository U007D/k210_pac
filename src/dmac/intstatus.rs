#[doc = "Reader of register intstatus"]
pub type R = crate::R<u64, super::INTSTATUS>;
#[doc = "Writer for register intstatus"]
pub type W = crate::W<u64, super::INTSTATUS>;
#[doc = "Register intstatus `reset()`'s with value 0"]
impl crate::ResetValue for super::INTSTATUS {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ch1_intstat`"]
pub type CH1_INTSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch1_intstat`"]
pub struct CH1_INTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_INTSTAT_W<'a> {
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
#[doc = "Reader of field `ch2_intstat`"]
pub type CH2_INTSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch2_intstat`"]
pub struct CH2_INTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_INTSTAT_W<'a> {
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
#[doc = "Reader of field `ch3_intstat`"]
pub type CH3_INTSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch3_intstat`"]
pub struct CH3_INTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_INTSTAT_W<'a> {
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
#[doc = "Reader of field `ch4_intstat`"]
pub type CH4_INTSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch4_intstat`"]
pub struct CH4_INTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_INTSTAT_W<'a> {
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
#[doc = "Reader of field `ch5_intstat`"]
pub type CH5_INTSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch5_intstat`"]
pub struct CH5_INTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_INTSTAT_W<'a> {
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
#[doc = "Reader of field `ch6_intstat`"]
pub type CH6_INTSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ch6_intstat`"]
pub struct CH6_INTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_INTSTAT_W<'a> {
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
#[doc = "Reader of field `commonreg_intstat`"]
pub type COMMONREG_INTSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `commonreg_intstat`"]
pub struct COMMONREG_INTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMONREG_INTSTAT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Channel 1 interrupt bit"]
    #[inline(always)]
    pub fn ch1_intstat(&self) -> CH1_INTSTAT_R {
        CH1_INTSTAT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 2 interrupt bit"]
    #[inline(always)]
    pub fn ch2_intstat(&self) -> CH2_INTSTAT_R {
        CH2_INTSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 3 interrupt bit"]
    #[inline(always)]
    pub fn ch3_intstat(&self) -> CH3_INTSTAT_R {
        CH3_INTSTAT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 4 interrupt bit"]
    #[inline(always)]
    pub fn ch4_intstat(&self) -> CH4_INTSTAT_R {
        CH4_INTSTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 5 interrupt bit"]
    #[inline(always)]
    pub fn ch5_intstat(&self) -> CH5_INTSTAT_R {
        CH5_INTSTAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 6 interrupt bit"]
    #[inline(always)]
    pub fn ch6_intstat(&self) -> CH6_INTSTAT_R {
        CH6_INTSTAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Common register status bit"]
    #[inline(always)]
    pub fn commonreg_intstat(&self) -> COMMONREG_INTSTAT_R {
        COMMONREG_INTSTAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 1 interrupt bit"]
    #[inline(always)]
    pub fn ch1_intstat(&mut self) -> CH1_INTSTAT_W {
        CH1_INTSTAT_W { w: self }
    }
    #[doc = "Bit 1 - Channel 2 interrupt bit"]
    #[inline(always)]
    pub fn ch2_intstat(&mut self) -> CH2_INTSTAT_W {
        CH2_INTSTAT_W { w: self }
    }
    #[doc = "Bit 2 - Channel 3 interrupt bit"]
    #[inline(always)]
    pub fn ch3_intstat(&mut self) -> CH3_INTSTAT_W {
        CH3_INTSTAT_W { w: self }
    }
    #[doc = "Bit 3 - Channel 4 interrupt bit"]
    #[inline(always)]
    pub fn ch4_intstat(&mut self) -> CH4_INTSTAT_W {
        CH4_INTSTAT_W { w: self }
    }
    #[doc = "Bit 4 - Channel 5 interrupt bit"]
    #[inline(always)]
    pub fn ch5_intstat(&mut self) -> CH5_INTSTAT_W {
        CH5_INTSTAT_W { w: self }
    }
    #[doc = "Bit 5 - Channel 6 interrupt bit"]
    #[inline(always)]
    pub fn ch6_intstat(&mut self) -> CH6_INTSTAT_W {
        CH6_INTSTAT_W { w: self }
    }
    #[doc = "Bit 16 - Common register status bit"]
    #[inline(always)]
    pub fn commonreg_intstat(&mut self) -> COMMONREG_INTSTAT_W {
        COMMONREG_INTSTAT_W { w: self }
    }
}
