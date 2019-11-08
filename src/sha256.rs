#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Calculated SHA256 return value"]
    pub result: [RESULT; 8],
    #[doc = "0x20 - SHA256 input data is written to this register"]
    pub data_in: DATA_IN,
    _reserved2: [u8; 4usize],
    #[doc = "0x28 - Counters register"]
    pub num_reg: NUM_REG,
    #[doc = "0x2c - Function configuration register 0"]
    pub function_reg_0: FUNCTION_REG_0,
    _reserved4: [u8; 4usize],
    #[doc = "0x34 - Function configuration register 1"]
    pub function_reg_1: FUNCTION_REG_1,
}
#[doc = "Calculated SHA256 return value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [result](result) module"]
pub type RESULT = crate::Reg<u32, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "`write(|w| ..)` method takes [result::W](result::W) writer structure"]
impl crate::Writable for RESULT {}
#[doc = "Calculated SHA256 return value"]
pub mod result;
#[doc = "SHA256 input data is written to this register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data_in](data_in) module"]
pub type DATA_IN = crate::Reg<u32, _DATA_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA_IN;
#[doc = "`read()` method returns [data_in::R](data_in::R) reader structure"]
impl crate::Readable for DATA_IN {}
#[doc = "`write(|w| ..)` method takes [data_in::W](data_in::W) writer structure"]
impl crate::Writable for DATA_IN {}
#[doc = "SHA256 input data is written to this register"]
pub mod data_in;
#[doc = "Counters register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [num_reg](num_reg) module"]
pub type NUM_REG = crate::Reg<u32, _NUM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NUM_REG;
#[doc = "`read()` method returns [num_reg::R](num_reg::R) reader structure"]
impl crate::Readable for NUM_REG {}
#[doc = "`write(|w| ..)` method takes [num_reg::W](num_reg::W) writer structure"]
impl crate::Writable for NUM_REG {}
#[doc = "Counters register"]
pub mod num_reg;
#[doc = "Function configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [function_reg_0](function_reg_0) module"]
pub type FUNCTION_REG_0 = crate::Reg<u32, _FUNCTION_REG_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNCTION_REG_0;
#[doc = "`read()` method returns [function_reg_0::R](function_reg_0::R) reader structure"]
impl crate::Readable for FUNCTION_REG_0 {}
#[doc = "`write(|w| ..)` method takes [function_reg_0::W](function_reg_0::W) writer structure"]
impl crate::Writable for FUNCTION_REG_0 {}
#[doc = "Function configuration register 0"]
pub mod function_reg_0;
#[doc = "Function configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [function_reg_1](function_reg_1) module"]
pub type FUNCTION_REG_1 = crate::Reg<u32, _FUNCTION_REG_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FUNCTION_REG_1;
#[doc = "`read()` method returns [function_reg_1::R](function_reg_1::R) reader structure"]
impl crate::Readable for FUNCTION_REG_1 {}
#[doc = "`write(|w| ..)` method takes [function_reg_1::W](function_reg_1::W) writer structure"]
impl crate::Writable for FUNCTION_REG_1 {}
#[doc = "Function configuration register 1"]
pub mod function_reg_1;
