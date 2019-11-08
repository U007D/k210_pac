#[doc = "Reader of register con"]
pub type R = crate::R<u32, super::CON>;
#[doc = "Writer for register con"]
pub type W = crate::W<u32, super::CON>;
#[doc = "Register con `reset()`'s with value 0"]
impl crate::ResetValue for super::CON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `master_mode`"]
pub type MASTER_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `master_mode`"]
pub struct MASTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_MODE_W<'a> {
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
#[doc = "Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEED_A {
    #[doc = "0: STANDARD"]
    STANDARD,
    #[doc = "1: FAST"]
    FAST,
    #[doc = "2: HIGHSPEED"]
    HIGHSPEED,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        match variant {
            SPEED_A::STANDARD => 0,
            SPEED_A::FAST => 1,
            SPEED_A::HIGHSPEED => 2,
        }
    }
}
#[doc = "Reader of field `speed`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPEED_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPEED_A::STANDARD),
            1 => Val(SPEED_A::FAST),
            2 => Val(SPEED_A::HIGHSPEED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SPEED_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == SPEED_A::FAST
    }
    #[doc = "Checks if the value of the field is `HIGHSPEED`"]
    #[inline(always)]
    pub fn is_highspeed(&self) -> bool {
        *self == SPEED_A::HIGHSPEED
    }
}
#[doc = "Write proxy for field `speed`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPEED_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "STANDARD"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SPEED_A::STANDARD)
    }
    #[doc = "FAST"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(SPEED_A::FAST)
    }
    #[doc = "HIGHSPEED"]
    #[inline(always)]
    pub fn highspeed(self) -> &'a mut W {
        self.variant(SPEED_A::HIGHSPEED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Slave address width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_SLAVE_WIDTH_A {
    #[doc = "0: 7-bit address"]
    B7,
    #[doc = "1: 10-bit address"]
    B10,
}
impl From<ADDR_SLAVE_WIDTH_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_SLAVE_WIDTH_A) -> Self {
        match variant {
            ADDR_SLAVE_WIDTH_A::B7 => false,
            ADDR_SLAVE_WIDTH_A::B10 => true,
        }
    }
}
#[doc = "Reader of field `addr_slave_width`"]
pub type ADDR_SLAVE_WIDTH_R = crate::R<bool, ADDR_SLAVE_WIDTH_A>;
impl ADDR_SLAVE_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_SLAVE_WIDTH_A {
        match self.bits {
            false => ADDR_SLAVE_WIDTH_A::B7,
            true => ADDR_SLAVE_WIDTH_A::B10,
        }
    }
    #[doc = "Checks if the value of the field is `B7`"]
    #[inline(always)]
    pub fn is_b7(&self) -> bool {
        *self == ADDR_SLAVE_WIDTH_A::B7
    }
    #[doc = "Checks if the value of the field is `B10`"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ADDR_SLAVE_WIDTH_A::B10
    }
}
#[doc = "Write proxy for field `addr_slave_width`"]
pub struct ADDR_SLAVE_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_SLAVE_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR_SLAVE_WIDTH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "7-bit address"]
    #[inline(always)]
    pub fn b7(self) -> &'a mut W {
        self.variant(ADDR_SLAVE_WIDTH_A::B7)
    }
    #[doc = "10-bit address"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut W {
        self.variant(ADDR_SLAVE_WIDTH_A::B10)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `restart_en`"]
pub type RESTART_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `restart_en`"]
pub struct RESTART_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESTART_EN_W<'a> {
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
#[doc = "Reader of field `slave_disable`"]
pub type SLAVE_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `slave_disable`"]
pub struct SLAVE_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_DISABLE_W<'a> {
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
#[doc = "Reader of field `stop_det`"]
pub type STOP_DET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `stop_det`"]
pub struct STOP_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_DET_W<'a> {
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
#[doc = "Reader of field `tx_empty`"]
pub type TX_EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `tx_empty`"]
pub struct TX_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_EMPTY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Master Mode"]
    #[inline(always)]
    pub fn master_mode(&self) -> MASTER_MODE_R {
        MASTER_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Slave address width"]
    #[inline(always)]
    pub fn addr_slave_width(&self) -> ADDR_SLAVE_WIDTH_R {
        ADDR_SLAVE_WIDTH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable Restart"]
    #[inline(always)]
    pub fn restart_en(&self) -> RESTART_EN_R {
        RESTART_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Disable Slave"]
    #[inline(always)]
    pub fn slave_disable(&self) -> SLAVE_DISABLE_R {
        SLAVE_DISABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline(always)]
    pub fn stop_det(&self) -> STOP_DET_R {
        STOP_DET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TX_EMPTY_CTRL"]
    #[inline(always)]
    pub fn tx_empty(&self) -> TX_EMPTY_R {
        TX_EMPTY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Mode"]
    #[inline(always)]
    pub fn master_mode(&mut self) -> MASTER_MODE_W {
        MASTER_MODE_W { w: self }
    }
    #[doc = "Bits 1:2 - Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 3 - Slave address width"]
    #[inline(always)]
    pub fn addr_slave_width(&mut self) -> ADDR_SLAVE_WIDTH_W {
        ADDR_SLAVE_WIDTH_W { w: self }
    }
    #[doc = "Bit 5 - Enable Restart"]
    #[inline(always)]
    pub fn restart_en(&mut self) -> RESTART_EN_W {
        RESTART_EN_W { w: self }
    }
    #[doc = "Bit 6 - Disable Slave"]
    #[inline(always)]
    pub fn slave_disable(&mut self) -> SLAVE_DISABLE_W {
        SLAVE_DISABLE_W { w: self }
    }
    #[doc = "Bit 7 - STOP_DET_IFADDRESSED"]
    #[inline(always)]
    pub fn stop_det(&mut self) -> STOP_DET_W {
        STOP_DET_W { w: self }
    }
    #[doc = "Bit 8 - TX_EMPTY_CTRL"]
    #[inline(always)]
    pub fn tx_empty(&mut self) -> TX_EMPTY_W {
        TX_EMPTY_W { w: self }
    }
}
