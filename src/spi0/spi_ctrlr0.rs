#[doc = "Reader of register spi_ctrlr0"]
pub type R = crate::R<u32, super::SPI_CTRLR0>;
#[doc = "Writer for register spi_ctrlr0"]
pub type W = crate::W<u32, super::SPI_CTRLR0>;
#[doc = "Register spi_ctrlr0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_CTRLR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "instruction_address_trans_mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AITM_A {
    #[doc = "0: STANDARD"]
    STANDARD,
    #[doc = "1: ADDR_STANDARD"]
    ADDR_STANDARD,
    #[doc = "2: AS_FRAME_FORMAT"]
    AS_FRAME_FORMAT,
}
impl From<AITM_A> for u8 {
    #[inline(always)]
    fn from(variant: AITM_A) -> Self {
        match variant {
            AITM_A::STANDARD => 0,
            AITM_A::ADDR_STANDARD => 1,
            AITM_A::AS_FRAME_FORMAT => 2,
        }
    }
}
#[doc = "Reader of field `aitm`"]
pub type AITM_R = crate::R<u8, AITM_A>;
impl AITM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AITM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AITM_A::STANDARD),
            1 => Val(AITM_A::ADDR_STANDARD),
            2 => Val(AITM_A::AS_FRAME_FORMAT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == AITM_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `ADDR_STANDARD`"]
    #[inline(always)]
    pub fn is_addr_standard(&self) -> bool {
        *self == AITM_A::ADDR_STANDARD
    }
    #[doc = "Checks if the value of the field is `AS_FRAME_FORMAT`"]
    #[inline(always)]
    pub fn is_as_frame_format(&self) -> bool {
        *self == AITM_A::AS_FRAME_FORMAT
    }
}
#[doc = "Write proxy for field `aitm`"]
pub struct AITM_W<'a> {
    w: &'a mut W,
}
impl<'a> AITM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AITM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "STANDARD"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(AITM_A::STANDARD)
    }
    #[doc = "ADDR_STANDARD"]
    #[inline(always)]
    pub fn addr_standard(self) -> &'a mut W {
        self.variant(AITM_A::ADDR_STANDARD)
    }
    #[doc = "AS_FRAME_FORMAT"]
    #[inline(always)]
    // For consistency with k210 data sheet
    #[allow(clippy::wrong_self_convention)]
    pub fn as_frame_format(self) -> &'a mut W {
        self.variant(AITM_A::AS_FRAME_FORMAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `addr_length`"]
pub type ADDR_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `addr_length`"]
pub struct ADDR_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Reader of field `inst_length`"]
pub type INST_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `inst_length`"]
pub struct INST_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> INST_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `wait_cycles`"]
pub type WAIT_CYCLES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `wait_cycles`"]
pub struct WAIT_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - instruction_address_trans_mode"]
    #[inline(always)]
    pub fn aitm(&self) -> AITM_R {
        AITM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - ADDR_LENGTH"]
    #[inline(always)]
    pub fn addr_length(&self) -> ADDR_LENGTH_R {
        ADDR_LENGTH_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - INSTRUCTION_LENGTH"]
    #[inline(always)]
    pub fn inst_length(&self) -> INST_LENGTH_R {
        INST_LENGTH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 11:15 - WAIT_CYCLES"]
    #[inline(always)]
    pub fn wait_cycles(&self) -> WAIT_CYCLES_R {
        WAIT_CYCLES_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - instruction_address_trans_mode"]
    #[inline(always)]
    pub fn aitm(&mut self) -> AITM_W {
        AITM_W { w: self }
    }
    #[doc = "Bits 2:5 - ADDR_LENGTH"]
    #[inline(always)]
    pub fn addr_length(&mut self) -> ADDR_LENGTH_W {
        ADDR_LENGTH_W { w: self }
    }
    #[doc = "Bits 8:9 - INSTRUCTION_LENGTH"]
    #[inline(always)]
    pub fn inst_length(&mut self) -> INST_LENGTH_W {
        INST_LENGTH_W { w: self }
    }
    #[doc = "Bits 11:15 - WAIT_CYCLES"]
    #[inline(always)]
    pub fn wait_cycles(&mut self) -> WAIT_CYCLES_W {
        WAIT_CYCLES_W { w: self }
    }
}
