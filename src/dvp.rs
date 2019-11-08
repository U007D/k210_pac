#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Config Register"]
    pub dvp_cfg: DVP_CFG,
    #[doc = "0x04 - R_ADDR"]
    pub r_addr: R_ADDR,
    #[doc = "0x08 - G_ADDR"]
    pub g_addr: G_ADDR,
    #[doc = "0x0c - B_ADDR"]
    pub b_addr: B_ADDR,
    #[doc = "0x10 - CMOS Config Register"]
    pub cmos_cfg: CMOS_CFG,
    #[doc = "0x14 - SCCB Config Register"]
    pub sccb_cfg: SCCB_CFG,
    #[doc = "0x18 - SCCB Control Register"]
    pub sccb_ctl: SCCB_CTL,
    #[doc = "0x1c - AXI Register"]
    pub axi: AXI,
    #[doc = "0x20 - STS Register"]
    pub sts: STS,
    #[doc = "0x24 - REVERSE"]
    pub reverse: REVERSE,
    #[doc = "0x28 - RGB_ADDR"]
    pub rgb_addr: RGB_ADDR,
}
#[doc = "Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dvp_cfg](dvp_cfg) module"]
pub type DVP_CFG = crate::Reg<u32, _DVP_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVP_CFG;
#[doc = "`read()` method returns [dvp_cfg::R](dvp_cfg::R) reader structure"]
impl crate::Readable for DVP_CFG {}
#[doc = "`write(|w| ..)` method takes [dvp_cfg::W](dvp_cfg::W) writer structure"]
impl crate::Writable for DVP_CFG {}
#[doc = "Config Register"]
pub mod dvp_cfg;
#[doc = "R_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [r_addr](r_addr) module"]
pub type R_ADDR = crate::Reg<u32, _R_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _R_ADDR;
#[doc = "`read()` method returns [r_addr::R](r_addr::R) reader structure"]
impl crate::Readable for R_ADDR {}
#[doc = "`write(|w| ..)` method takes [r_addr::W](r_addr::W) writer structure"]
impl crate::Writable for R_ADDR {}
#[doc = "R_ADDR"]
pub mod r_addr;
#[doc = "G_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [g_addr](g_addr) module"]
pub type G_ADDR = crate::Reg<u32, _G_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _G_ADDR;
#[doc = "`read()` method returns [g_addr::R](g_addr::R) reader structure"]
impl crate::Readable for G_ADDR {}
#[doc = "`write(|w| ..)` method takes [g_addr::W](g_addr::W) writer structure"]
impl crate::Writable for G_ADDR {}
#[doc = "G_ADDR"]
pub mod g_addr;
#[doc = "B_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [b_addr](b_addr) module"]
pub type B_ADDR = crate::Reg<u32, _B_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B_ADDR;
#[doc = "`read()` method returns [b_addr::R](b_addr::R) reader structure"]
impl crate::Readable for B_ADDR {}
#[doc = "`write(|w| ..)` method takes [b_addr::W](b_addr::W) writer structure"]
impl crate::Writable for B_ADDR {}
#[doc = "B_ADDR"]
pub mod b_addr;
#[doc = "CMOS Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmos_cfg](cmos_cfg) module"]
pub type CMOS_CFG = crate::Reg<u32, _CMOS_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMOS_CFG;
#[doc = "`read()` method returns [cmos_cfg::R](cmos_cfg::R) reader structure"]
impl crate::Readable for CMOS_CFG {}
#[doc = "`write(|w| ..)` method takes [cmos_cfg::W](cmos_cfg::W) writer structure"]
impl crate::Writable for CMOS_CFG {}
#[doc = "CMOS Config Register"]
pub mod cmos_cfg;
#[doc = "SCCB Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sccb_cfg](sccb_cfg) module"]
pub type SCCB_CFG = crate::Reg<u32, _SCCB_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCCB_CFG;
#[doc = "`read()` method returns [sccb_cfg::R](sccb_cfg::R) reader structure"]
impl crate::Readable for SCCB_CFG {}
#[doc = "`write(|w| ..)` method takes [sccb_cfg::W](sccb_cfg::W) writer structure"]
impl crate::Writable for SCCB_CFG {}
#[doc = "SCCB Config Register"]
pub mod sccb_cfg;
#[doc = "SCCB Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sccb_ctl](sccb_ctl) module"]
pub type SCCB_CTL = crate::Reg<u32, _SCCB_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCCB_CTL;
#[doc = "`read()` method returns [sccb_ctl::R](sccb_ctl::R) reader structure"]
impl crate::Readable for SCCB_CTL {}
#[doc = "`write(|w| ..)` method takes [sccb_ctl::W](sccb_ctl::W) writer structure"]
impl crate::Writable for SCCB_CTL {}
#[doc = "SCCB Control Register"]
pub mod sccb_ctl;
#[doc = "AXI Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [axi](axi) module"]
pub type AXI = crate::Reg<u32, _AXI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AXI;
#[doc = "`read()` method returns [axi::R](axi::R) reader structure"]
impl crate::Readable for AXI {}
#[doc = "`write(|w| ..)` method takes [axi::W](axi::W) writer structure"]
impl crate::Writable for AXI {}
#[doc = "AXI Register"]
pub mod axi;
#[doc = "STS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sts](sts) module"]
pub type STS = crate::Reg<u32, _STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STS;
#[doc = "`read()` method returns [sts::R](sts::R) reader structure"]
impl crate::Readable for STS {}
#[doc = "`write(|w| ..)` method takes [sts::W](sts::W) writer structure"]
impl crate::Writable for STS {}
#[doc = "STS Register"]
pub mod sts;
#[doc = "REVERSE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reverse](reverse) module"]
pub type REVERSE = crate::Reg<u32, _REVERSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REVERSE;
#[doc = "`read()` method returns [reverse::R](reverse::R) reader structure"]
impl crate::Readable for REVERSE {}
#[doc = "`write(|w| ..)` method takes [reverse::W](reverse::W) writer structure"]
impl crate::Writable for REVERSE {}
#[doc = "REVERSE"]
pub mod reverse;
#[doc = "RGB_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rgb_addr](rgb_addr) module"]
pub type RGB_ADDR = crate::Reg<u32, _RGB_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RGB_ADDR;
#[doc = "`read()` method returns [rgb_addr::R](rgb_addr::R) reader structure"]
impl crate::Readable for RGB_ADDR {}
#[doc = "`write(|w| ..)` method takes [rgb_addr::W](rgb_addr::W) writer structure"]
impl crate::Writable for RGB_ADDR {}
#[doc = "RGB_ADDR"]
pub mod rgb_addr;
