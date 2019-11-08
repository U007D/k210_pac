#[doc = "Reader of register fifo_ctrl"]
pub type R = crate::R<u64, super::FIFO_CTRL>;
#[doc = "Writer for register fifo_ctrl"]
pub type W = crate::W<u64, super::FIFO_CTRL>;
#[doc = "Register fifo_ctrl `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO_CTRL {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `resp_fifo_flush`"]
pub type RESP_FIFO_FLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `resp_fifo_flush`"]
pub struct RESP_FIFO_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> RESP_FIFO_FLUSH_W<'a> {
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
#[doc = "Reader of field `cmd_fifo_flush`"]
pub type CMD_FIFO_FLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `cmd_fifo_flush`"]
pub struct CMD_FIFO_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_FIFO_FLUSH_W<'a> {
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
#[doc = "Reader of field `gs_fifo_flush`"]
pub type GS_FIFO_FLUSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `gs_fifo_flush`"]
pub struct GS_FIFO_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> GS_FIFO_FLUSH_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Response memory initialization flag"]
    #[inline(always)]
    pub fn resp_fifo_flush(&self) -> RESP_FIFO_FLUSH_R {
        RESP_FIFO_FLUSH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command memory initialization flag"]
    #[inline(always)]
    pub fn cmd_fifo_flush(&self) -> CMD_FIFO_FLUSH_R {
        CMD_FIFO_FLUSH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output interface memory initialization flag"]
    #[inline(always)]
    pub fn gs_fifo_flush(&self) -> GS_FIFO_FLUSH_R {
        GS_FIFO_FLUSH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Response memory initialization flag"]
    #[inline(always)]
    pub fn resp_fifo_flush(&mut self) -> RESP_FIFO_FLUSH_W {
        RESP_FIFO_FLUSH_W { w: self }
    }
    #[doc = "Bit 1 - Command memory initialization flag"]
    #[inline(always)]
    pub fn cmd_fifo_flush(&mut self) -> CMD_FIFO_FLUSH_W {
        CMD_FIFO_FLUSH_W { w: self }
    }
    #[doc = "Bit 2 - Output interface memory initialization flag"]
    #[inline(always)]
    pub fn gs_fifo_flush(&mut self) -> GS_FIFO_FLUSH_W {
        GS_FIFO_FLUSH_W { w: self }
    }
}
