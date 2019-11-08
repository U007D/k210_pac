#[doc = "Reader of register clr_tx_abrt"]
pub type R = crate::R<u32, super::CLR_TX_ABRT>;
#[doc = "Reader of field `clr`"]
pub type CLR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 0x01) != 0)
    }
}
