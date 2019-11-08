#[doc = "Reader of register reset_status"]
pub type R = crate::R<u32, super::RESET_STATUS>;
#[doc = "Writer for register reset_status"]
pub type W = crate::W<u32, super::RESET_STATUS>;
#[doc = "Register reset_status `reset()`'s with value 0"]
impl crate::ResetValue for super::RESET_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `reset_sts_clr`"]
pub type RESET_STS_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `reset_sts_clr`"]
pub struct RESET_STS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_STS_CLR_W<'a> {
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
#[doc = "Reader of field `pin_reset_sts`"]
pub type PIN_RESET_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `pin_reset_sts`"]
pub struct PIN_RESET_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_RESET_STS_W<'a> {
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
#[doc = "Reader of field `wdt0_reset_sts`"]
pub type WDT0_RESET_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wdt0_reset_sts`"]
pub struct WDT0_RESET_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT0_RESET_STS_W<'a> {
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
#[doc = "Reader of field `wdt1_reset_sts`"]
pub type WDT1_RESET_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `wdt1_reset_sts`"]
pub struct WDT1_RESET_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT1_RESET_STS_W<'a> {
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
#[doc = "Reader of field `soft_reset_sts`"]
pub type SOFT_RESET_STS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `soft_reset_sts`"]
pub struct SOFT_RESET_STS_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RESET_STS_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_sts_clr(&self) -> RESET_STS_CLR_R {
        RESET_STS_CLR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pin_reset_sts(&self) -> PIN_RESET_STS_R {
        PIN_RESET_STS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wdt0_reset_sts(&self) -> WDT0_RESET_STS_R {
        WDT0_RESET_STS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdt1_reset_sts(&self) -> WDT1_RESET_STS_R {
        WDT1_RESET_STS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn soft_reset_sts(&self) -> SOFT_RESET_STS_R {
        SOFT_RESET_STS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reset_sts_clr(&mut self) -> RESET_STS_CLR_W {
        RESET_STS_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pin_reset_sts(&mut self) -> PIN_RESET_STS_W {
        PIN_RESET_STS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wdt0_reset_sts(&mut self) -> WDT0_RESET_STS_W {
        WDT0_RESET_STS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdt1_reset_sts(&mut self) -> WDT1_RESET_STS_W {
        WDT1_RESET_STS_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn soft_reset_sts(&mut self) -> SOFT_RESET_STS_W {
        SOFT_RESET_STS_W { w: self }
    }
}
