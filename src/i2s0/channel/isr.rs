#[doc = "Reader of register isr"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `rxda`"]
pub type RXDA_R = crate::R<bool, bool>;
#[doc = "Reader of field `rxfo`"]
pub type RXFO_R = crate::R<bool, bool>;
#[doc = "Reader of field `txfe`"]
pub type TXFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `txfo`"]
pub type TXFO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Status of receiver data avaliable interrupt"]
    #[inline(always)]
    pub fn rxda(&self) -> RXDA_R {
        RXDA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of data overrun interrupt for RX channel"]
    #[inline(always)]
    pub fn rxfo(&self) -> RXFO_R {
        RXFO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of transmit empty triger interrupt"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of data overrun interrupt for the TX channel"]
    #[inline(always)]
    pub fn txfo(&self) -> TXFO_R {
        TXFO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
