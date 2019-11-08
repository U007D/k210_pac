#[doc = "Reader of register comp_type"]
pub type R = crate::R<u32, super::COMP_TYPE>;
#[doc = "Reader of field `value`"]
pub type VALUE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
