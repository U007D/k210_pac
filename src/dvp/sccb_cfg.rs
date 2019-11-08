#[doc = "Reader of register sccb_cfg"]
pub type R = crate::R<u32, super::SCCB_CFG>;
#[doc = "Writer for register sccb_cfg"]
pub type W = crate::W<u32, super::SCCB_CFG>;
#[doc = "Register sccb_cfg `reset()`'s with value 0"]
impl crate::ResetValue for super::SCCB_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BYTE_NUM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE_NUM_A {
    #[doc = "1: BYTE_NUM_2"]
    NUM2,
    #[doc = "2: BYTE_NUM_3"]
    NUM3,
    #[doc = "3: BYTE_NUM_4"]
    NUM4,
}
impl From<BYTE_NUM_A> for u8 {
    #[inline(always)]
    fn from(variant: BYTE_NUM_A) -> Self {
        match variant {
            BYTE_NUM_A::NUM2 => 1,
            BYTE_NUM_A::NUM3 => 2,
            BYTE_NUM_A::NUM4 => 3,
        }
    }
}
#[doc = "Reader of field `byte_num`"]
pub type BYTE_NUM_R = crate::R<u8, BYTE_NUM_A>;
impl BYTE_NUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BYTE_NUM_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(BYTE_NUM_A::NUM2),
            2 => Val(BYTE_NUM_A::NUM3),
            3 => Val(BYTE_NUM_A::NUM4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NUM2`"]
    #[inline(always)]
    pub fn is_num2(&self) -> bool {
        *self == BYTE_NUM_A::NUM2
    }
    #[doc = "Checks if the value of the field is `NUM3`"]
    #[inline(always)]
    pub fn is_num3(&self) -> bool {
        *self == BYTE_NUM_A::NUM3
    }
    #[doc = "Checks if the value of the field is `NUM4`"]
    #[inline(always)]
    pub fn is_num4(&self) -> bool {
        *self == BYTE_NUM_A::NUM4
    }
}
#[doc = "Write proxy for field `byte_num`"]
pub struct BYTE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE_NUM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYTE_NUM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "BYTE_NUM_2"]
    #[inline(always)]
    pub fn num2(self) -> &'a mut W {
        self.variant(BYTE_NUM_A::NUM2)
    }
    #[doc = "BYTE_NUM_3"]
    #[inline(always)]
    pub fn num3(self) -> &'a mut W {
        self.variant(BYTE_NUM_A::NUM3)
    }
    #[doc = "BYTE_NUM_4"]
    #[inline(always)]
    pub fn num4(self) -> &'a mut W {
        self.variant(BYTE_NUM_A::NUM4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `scl_lcnt`"]
pub type SCL_LCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `scl_lcnt`"]
pub struct SCL_LCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_LCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `scl_hcnt`"]
pub type SCL_HCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `scl_hcnt`"]
pub struct SCL_HCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_HCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `rdata`"]
pub type RDATA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - BYTE_NUM"]
    #[inline(always)]
    pub fn byte_num(&self) -> BYTE_NUM_R {
        BYTE_NUM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - SCL_LCNT"]
    #[inline(always)]
    pub fn scl_lcnt(&self) -> SCL_LCNT_R {
        SCL_LCNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCL_HCNT"]
    #[inline(always)]
    pub fn scl_hcnt(&self) -> SCL_HCNT_R {
        SCL_HCNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RDATA"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BYTE_NUM"]
    #[inline(always)]
    pub fn byte_num(&mut self) -> BYTE_NUM_W {
        BYTE_NUM_W { w: self }
    }
    #[doc = "Bits 8:15 - SCL_LCNT"]
    #[inline(always)]
    pub fn scl_lcnt(&mut self) -> SCL_LCNT_W {
        SCL_LCNT_W { w: self }
    }
    #[doc = "Bits 16:23 - SCL_HCNT"]
    #[inline(always)]
    pub fn scl_hcnt(&mut self) -> SCL_HCNT_W {
        SCL_HCNT_W { w: self }
    }
}
