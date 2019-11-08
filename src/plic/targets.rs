#[doc = "Priority Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [threshold](threshold) module"]
pub type THRESHOLD = crate::Reg<u32, _THRESHOLD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THRESHOLD;
#[doc = "`read()` method returns [threshold::R](threshold::R) reader structure"]
impl crate::Readable for THRESHOLD {}
#[doc = "`write(|w| ..)` method takes [threshold::W](threshold::W) writer structure"]
impl crate::Writable for THRESHOLD {}
#[doc = "Priority Threshold Register"]
pub mod threshold;
#[doc = "Claim/Complete Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [claim](claim) module"]
pub type CLAIM = crate::Reg<u32, _CLAIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIM;
#[doc = "`read()` method returns [claim::R](claim::R) reader structure"]
impl crate::Readable for CLAIM {}
#[doc = "`write(|w| ..)` method takes [claim::W](claim::W) writer structure"]
impl crate::Writable for CLAIM {}
#[doc = "Claim/Complete Register"]
pub mod claim;
#[doc = "Padding to make sure targets is an array\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [_reserved](_reserved) module"]
pub type _RESERVED = crate::Reg<u32, __RESERVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __RESERVED;
#[doc = "`read()` method returns [_reserved::R](_reserved::R) reader structure"]
impl crate::Readable for _RESERVED {}
#[doc = "`write(|w| ..)` method takes [_reserved::W](_reserved::W) writer structure"]
impl crate::Writable for _RESERVED {}
#[doc = "Padding to make sure targets is an array"]
pub mod _reserved;
