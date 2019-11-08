#[doc = "Reader of register int_stat"]
pub type R = crate::R<u32, super::INT_STAT>;
#[doc = "Writer for register int_stat"]
pub type W = crate::W<u32, super::INT_STAT>;
#[doc = "Register int_stat `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `dir_search_data_rdy`"]
pub type DIR_SEARCH_DATA_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `dir_search_data_rdy`"]
pub struct DIR_SEARCH_DATA_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_SEARCH_DATA_RDY_W<'a> {
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
#[doc = "Reader of field `voc_buf_data_rdy`"]
pub type VOC_BUF_DATA_RDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `voc_buf_data_rdy`"]
pub struct VOC_BUF_DATA_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> VOC_BUF_DATA_RDY_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Sound direction searching data ready interrupt event"]
    #[inline(always)]
    pub fn dir_search_data_rdy(&self) -> DIR_SEARCH_DATA_RDY_R {
        DIR_SEARCH_DATA_RDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Voice output stream buffer data ready interrupt event"]
    #[inline(always)]
    pub fn voc_buf_data_rdy(&self) -> VOC_BUF_DATA_RDY_R {
        VOC_BUF_DATA_RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sound direction searching data ready interrupt event"]
    #[inline(always)]
    pub fn dir_search_data_rdy(&mut self) -> DIR_SEARCH_DATA_RDY_W {
        DIR_SEARCH_DATA_RDY_W { w: self }
    }
    #[doc = "Bit 1 - Voice output stream buffer data ready interrupt event"]
    #[inline(always)]
    pub fn voc_buf_data_rdy(&mut self) -> VOC_BUF_DATA_RDY_W {
        VOC_BUF_DATA_RDY_W { w: self }
    }
}
