#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Dummy register: this peripheral is not implemented yet"]
    pub dummy: DUMMY,
}
#[doc = "Dummy register: this peripheral is not implemented yet\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dummy](dummy) module"]
pub type DUMMY = crate::Reg<u32, _DUMMY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DUMMY;
#[doc = "`read()` method returns [dummy::R](dummy::R) reader structure"]
impl crate::Readable for DUMMY {}
#[doc = "`write(|w| ..)` method takes [dummy::W](dummy::W) writer structure"]
impl crate::Writable for DUMMY {}
#[doc = "Dummy register: this peripheral is not implemented yet"]
pub mod dummy;
