#[doc = "Reader of register clr_intr"]
pub type R = crate::R<u32, super::CLR_INTR>;
#[doc = "Reader of field `clr`"]
pub type CLR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CLR"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 0x01) != 0)
    }
}
