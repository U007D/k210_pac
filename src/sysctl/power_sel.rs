#[doc = "Reader of register power_sel"]
pub type R = crate::R<u32, super::POWER_SEL>;
#[doc = "Writer for register power_sel"]
pub type W = crate::W<u32, super::POWER_SEL>;
#[doc = "Register power_sel `reset()`'s with value 0"]
impl crate::ResetValue for super::POWER_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `power_mode_sel0`"]
pub type POWER_MODE_SEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `power_mode_sel0`"]
pub struct POWER_MODE_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_SEL0_W<'a> {
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
#[doc = "Reader of field `power_mode_sel1`"]
pub type POWER_MODE_SEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `power_mode_sel1`"]
pub struct POWER_MODE_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_SEL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `power_mode_sel2`"]
pub type POWER_MODE_SEL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `power_mode_sel2`"]
pub struct POWER_MODE_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_SEL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `power_mode_sel3`"]
pub type POWER_MODE_SEL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `power_mode_sel3`"]
pub struct POWER_MODE_SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_SEL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `power_mode_sel4`"]
pub type POWER_MODE_SEL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `power_mode_sel4`"]
pub struct POWER_MODE_SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_SEL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `power_mode_sel5`"]
pub type POWER_MODE_SEL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `power_mode_sel5`"]
pub struct POWER_MODE_SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_SEL5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `power_mode_sel6`"]
pub type POWER_MODE_SEL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `power_mode_sel6`"]
pub struct POWER_MODE_SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_SEL6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `power_mode_sel7`"]
pub type POWER_MODE_SEL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `power_mode_sel7`"]
pub struct POWER_MODE_SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_MODE_SEL7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn power_mode_sel0(&self) -> POWER_MODE_SEL0_R {
        POWER_MODE_SEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn power_mode_sel1(&self) -> POWER_MODE_SEL1_R {
        POWER_MODE_SEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn power_mode_sel2(&self) -> POWER_MODE_SEL2_R {
        POWER_MODE_SEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn power_mode_sel3(&self) -> POWER_MODE_SEL3_R {
        POWER_MODE_SEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn power_mode_sel4(&self) -> POWER_MODE_SEL4_R {
        POWER_MODE_SEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn power_mode_sel5(&self) -> POWER_MODE_SEL5_R {
        POWER_MODE_SEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn power_mode_sel6(&self) -> POWER_MODE_SEL6_R {
        POWER_MODE_SEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn power_mode_sel7(&self) -> POWER_MODE_SEL7_R {
        POWER_MODE_SEL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn power_mode_sel0(&mut self) -> POWER_MODE_SEL0_W {
        POWER_MODE_SEL0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn power_mode_sel1(&mut self) -> POWER_MODE_SEL1_W {
        POWER_MODE_SEL1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn power_mode_sel2(&mut self) -> POWER_MODE_SEL2_W {
        POWER_MODE_SEL2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn power_mode_sel3(&mut self) -> POWER_MODE_SEL3_W {
        POWER_MODE_SEL3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn power_mode_sel4(&mut self) -> POWER_MODE_SEL4_W {
        POWER_MODE_SEL4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn power_mode_sel5(&mut self) -> POWER_MODE_SEL5_W {
        POWER_MODE_SEL5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn power_mode_sel6(&mut self) -> POWER_MODE_SEL6_W {
        POWER_MODE_SEL6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn power_mode_sel7(&mut self) -> POWER_MODE_SEL7_W {
        POWER_MODE_SEL7_W { w: self }
    }
}
