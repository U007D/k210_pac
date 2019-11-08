#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FPIOA GPIO multiplexer io array"]
    pub io: [IO; 48],
    #[doc = "0xc0 - FPIOA GPIO multiplexer tie enable array"]
    pub tie_en: [TIE_EN; 8],
    #[doc = "0xe0 - FPIOA GPIO multiplexer tie value array"]
    pub tie_val: [TIE_VAL; 8],
}
#[doc = "FPIOA GPIO multiplexer io array\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [io](io) module"]
pub type IO = crate::Reg<u32, _IO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IO;
#[doc = "`read()` method returns [io::R](io::R) reader structure"]
impl crate::Readable for IO {}
#[doc = "`write(|w| ..)` method takes [io::W](io::W) writer structure"]
impl crate::Writable for IO {}
#[doc = "FPIOA GPIO multiplexer io array"]
pub mod io;
#[doc = "FPIOA GPIO multiplexer tie enable array\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tie_en](tie_en) module"]
pub type TIE_EN = crate::Reg<u32, _TIE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIE_EN;
#[doc = "`read()` method returns [tie_en::R](tie_en::R) reader structure"]
impl crate::Readable for TIE_EN {}
#[doc = "`write(|w| ..)` method takes [tie_en::W](tie_en::W) writer structure"]
impl crate::Writable for TIE_EN {}
#[doc = "FPIOA GPIO multiplexer tie enable array"]
pub mod tie_en;
#[doc = "FPIOA GPIO multiplexer tie value array\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tie_val](tie_val) module"]
pub type TIE_VAL = crate::Reg<u32, _TIE_VAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIE_VAL;
#[doc = "`read()` method returns [tie_val::R](tie_val::R) reader structure"]
impl crate::Readable for TIE_VAL {}
#[doc = "`write(|w| ..)` method takes [tie_val::W](tie_val::W) writer structure"]
impl crate::Writable for TIE_VAL {}
#[doc = "FPIOA GPIO multiplexer tie value array"]
pub mod tie_val;
