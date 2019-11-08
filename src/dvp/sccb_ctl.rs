#[doc = "Reader of register sccb_ctl"]
pub type R = crate::R<u32, super::SCCB_CTL>;
#[doc = "Writer for register sccb_ctl"]
pub type W = crate::W<u32, super::SCCB_CTL>;
#[doc = "Register sccb_ctl `reset()`'s with value 0"]
impl crate::ResetValue for super::SCCB_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `device_address`"]
pub type DEVICE_ADDRESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `device_address`"]
pub struct DEVICE_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICE_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `reg_address`"]
pub type REG_ADDRESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `reg_address`"]
pub struct REG_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `wdata_byte0`"]
pub type WDATA_BYTE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `wdata_byte0`"]
pub struct WDATA_BYTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_BYTE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `wdata_byte1`"]
pub type WDATA_BYTE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `wdata_byte1`"]
pub struct WDATA_BYTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_BYTE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DEVICE_ADDRESS"]
    #[inline(always)]
    pub fn device_address(&self) -> DEVICE_ADDRESS_R {
        DEVICE_ADDRESS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_ADDRESS"]
    #[inline(always)]
    pub fn reg_address(&self) -> REG_ADDRESS_R {
        REG_ADDRESS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - WDATA_BYTE0"]
    #[inline(always)]
    pub fn wdata_byte0(&self) -> WDATA_BYTE0_R {
        WDATA_BYTE0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - WDATA_BYTE1"]
    #[inline(always)]
    pub fn wdata_byte1(&self) -> WDATA_BYTE1_R {
        WDATA_BYTE1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DEVICE_ADDRESS"]
    #[inline(always)]
    pub fn device_address(&mut self) -> DEVICE_ADDRESS_W {
        DEVICE_ADDRESS_W { w: self }
    }
    #[doc = "Bits 8:15 - REG_ADDRESS"]
    #[inline(always)]
    pub fn reg_address(&mut self) -> REG_ADDRESS_W {
        REG_ADDRESS_W { w: self }
    }
    #[doc = "Bits 16:23 - WDATA_BYTE0"]
    #[inline(always)]
    pub fn wdata_byte0(&mut self) -> WDATA_BYTE0_W {
        WDATA_BYTE0_W { w: self }
    }
    #[doc = "Bits 24:31 - WDATA_BYTE1"]
    #[inline(always)]
    pub fn wdata_byte1(&mut self) -> WDATA_BYTE1_W {
        WDATA_BYTE1_W { w: self }
    }
}
