#[doc = "Reader of register tor"]
pub type R = crate::R<u32, super::TOR>;
#[doc = "Reader of field `txcho`"]
pub type TXCHO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read this bit to clear TX FIFO data overrun interrupt. 0x0 for TX FIFO write valid, 0x1 for TX FIFO write overrun"]
    #[inline(always)]
    pub fn txcho(&self) -> TXCHO_R {
        TXCHO_R::new((self.bits & 0x01) != 0)
    }
}
