#[doc = "Reader of register ror"]
pub type R = crate::R<u32, super::ROR>;
#[doc = "Reader of field `rxcho`"]
pub type RXCHO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Read this bit to clear RX FIFO data overrun interrupt. 0x0 for RX FIFO write valid, 0x1 for RX FIFO write overrun"]
    #[inline(always)]
    pub fn rxcho(&self) -> RXCHO_R {
        RXCHO_R::new((self.bits & 0x01) != 0)
    }
}
