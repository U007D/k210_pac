#[doc = "Reader of register swhssrc"]
pub type R = crate::R<u64, super::SWHSSRC>;
#[doc = "Writer for register swhssrc"]
pub type W = crate::W<u64, super::SWHSSRC>;
#[doc = "Register swhssrc `reset()`'s with value 0"]
impl crate::ResetValue for super::SWHSSRC {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `req`"]
pub type REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `req`"]
pub struct REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_W<'a> {
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
#[doc = "Reader of field `req_we`"]
pub type REQ_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `req_we`"]
pub struct REQ_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> REQ_WE_W<'a> {
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
#[doc = "Reader of field `sglreq`"]
pub type SGLREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sglreq`"]
pub struct SGLREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SGLREQ_W<'a> {
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
#[doc = "Reader of field `sglreq_we`"]
pub type SGLREQ_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sglreq_we`"]
pub struct SGLREQ_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> SGLREQ_WE_W<'a> {
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
#[doc = "Reader of field `lst`"]
pub type LST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lst`"]
pub struct LST_W<'a> {
    w: &'a mut W,
}
impl<'a> LST_W<'a> {
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
#[doc = "Reader of field `lst_we`"]
pub type LST_WE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lst_we`"]
pub struct LST_WE_W<'a> {
    w: &'a mut W,
}
impl<'a> LST_WE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Software handshake request for channel source"]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write enable bit for software handshake request"]
    #[inline(always)]
    pub fn req_we(&self) -> REQ_WE_R {
        REQ_WE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software handshake single request for channel source"]
    #[inline(always)]
    pub fn sglreq(&self) -> SGLREQ_R {
        SGLREQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write enable bit for software handshake"]
    #[inline(always)]
    pub fn sglreq_we(&self) -> SGLREQ_WE_R {
        SGLREQ_WE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software handshake last request for channel source"]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write enable bit for software handshake last request"]
    #[inline(always)]
    pub fn lst_we(&self) -> LST_WE_R {
        LST_WE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software handshake request for channel source"]
    #[inline(always)]
    pub fn req(&mut self) -> REQ_W {
        REQ_W { w: self }
    }
    #[doc = "Bit 1 - Write enable bit for software handshake request"]
    #[inline(always)]
    pub fn req_we(&mut self) -> REQ_WE_W {
        REQ_WE_W { w: self }
    }
    #[doc = "Bit 2 - Software handshake single request for channel source"]
    #[inline(always)]
    pub fn sglreq(&mut self) -> SGLREQ_W {
        SGLREQ_W { w: self }
    }
    #[doc = "Bit 3 - Write enable bit for software handshake"]
    #[inline(always)]
    pub fn sglreq_we(&mut self) -> SGLREQ_WE_W {
        SGLREQ_WE_W { w: self }
    }
    #[doc = "Bit 4 - Software handshake last request for channel source"]
    #[inline(always)]
    pub fn lst(&mut self) -> LST_W {
        LST_W { w: self }
    }
    #[doc = "Bit 5 - Write enable bit for software handshake last request"]
    #[inline(always)]
    pub fn lst_we(&mut self) -> LST_WE_W {
        LST_WE_W { w: self }
    }
}
