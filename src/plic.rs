#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Source Priority Register"]
    pub priority: [PRIORITY; 1024],
    #[doc = "0x1000 - Interrupt Pending Register"]
    pub pending: [PENDING; 32],
    _reserved2: [u8; 3968usize],
    #[doc = "0x2000 - Target Interrupt Enables"]
    pub target_enables: [TARGET_ENABLES; 4],
    _reserved3: [u8; 2_088_448usize],
    #[doc = "0x200000 - Target Configuration"]
    pub targets: [TARGETS; 4],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct TARGET_ENABLES {
    #[doc = "0x00 - Interrupt Enable Register"]
    pub enable: [self::target_enables::ENABLE; 32],
}
#[doc = r"Register block"]
#[doc = "Target Interrupt Enables"]
pub mod target_enables;
#[doc = r"Register block"]
#[repr(C)]
pub struct TARGETS {
    #[doc = "0x00 - Priority Threshold Register"]
    pub threshold: self::targets::THRESHOLD,
    #[doc = "0x04 - Claim/Complete Register"]
    pub claim: self::targets::CLAIM,
    _reserved2: [u8; 4084usize],
    #[doc = "0xffc - Padding to make sure targets is an array"]
    pub _reserved: self::targets::_RESERVED,
}
#[doc = r"Register block"]
#[doc = "Target Configuration"]
pub mod targets;
#[doc = "Interrupt Source Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [priority](priority) module"]
pub type PRIORITY = crate::Reg<u32, _PRIORITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRIORITY;
#[doc = "`read()` method returns [priority::R](priority::R) reader structure"]
impl crate::Readable for PRIORITY {}
#[doc = "`write(|w| ..)` method takes [priority::W](priority::W) writer structure"]
impl crate::Writable for PRIORITY {}
#[doc = "Interrupt Source Priority Register"]
pub mod priority;
#[doc = "Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pending](pending) module"]
pub type PENDING = crate::Reg<u32, _PENDING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PENDING;
#[doc = "`read()` method returns [pending::R](pending::R) reader structure"]
impl crate::Readable for PENDING {}
#[doc = "`write(|w| ..)` method takes [pending::W](pending::W) writer structure"]
impl crate::Writable for PENDING {}
#[doc = "Interrupt Pending Register"]
pub mod pending;
