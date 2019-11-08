#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel Config Register"]
    pub ch_cfg: CH_CFG,
    #[doc = "0x04 - Control Register"]
    pub ctl: CTL,
    #[doc = "0x08 - Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)"]
    pub dir_bidx: [DIR_BIDX; 32],
    #[doc = "0x88 - FIR0 pre-filter coefficients"]
    pub pre_fir0_coef: [PRE_FIR0_COEF; 9],
    #[doc = "0xac - FIR0 post-filter coefficients"]
    pub post_fir0_coef: [POST_FIR0_COEF; 9],
    #[doc = "0xd0 - FIR1 pre-filter coeffecients"]
    pub pre_fir1_coef: [PRE_FIR1_COEF; 9],
    #[doc = "0xf4 - FIR1 post-filter coefficients"]
    pub post_fir1_coef: [POST_FIR1_COEF; 9],
    #[doc = "0x118 - Downsize Config Register"]
    pub dwsz_cfg: DWSZ_CFG,
    #[doc = "0x11c - FFT Config Register"]
    pub fft_cfg: FFT_CFG,
    #[doc = "0x120 - Read register for DMA to sample-out buffers"]
    pub sobuf_dma_rdata: SOBUF_DMA_RDATA,
    #[doc = "0x124 - Read register for DMA to voice-out buffers"]
    pub vobuf_dma_rdata: VOBUF_DMA_RDATA,
    #[doc = "0x128 - Interrupt Status Register"]
    pub int_stat: INT_STAT,
    #[doc = "0x12c - Interrupt Mask Register"]
    pub int_mask: INT_MASK,
    #[doc = "0x130 - Saturation Counter"]
    pub sat_counter: SAT_COUNTER,
    #[doc = "0x134 - Saturation Limits"]
    pub sat_limits: SAT_LIMITS,
}
#[doc = "Channel Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ch_cfg](ch_cfg) module"]
pub type CH_CFG = crate::Reg<u32, _CH_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH_CFG;
#[doc = "`read()` method returns [ch_cfg::R](ch_cfg::R) reader structure"]
impl crate::Readable for CH_CFG {}
#[doc = "`write(|w| ..)` method takes [ch_cfg::W](ch_cfg::W) writer structure"]
impl crate::Writable for CH_CFG {}
#[doc = "Channel Config Register"]
pub mod ch_cfg;
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "Control Register"]
pub mod ctl;
#[doc = "Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dir_bidx](dir_bidx) module"]
pub type DIR_BIDX = crate::Reg<u32, _DIR_BIDX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR_BIDX;
#[doc = "`read()` method returns [dir_bidx::R](dir_bidx::R) reader structure"]
impl crate::Readable for DIR_BIDX {}
#[doc = "`write(|w| ..)` method takes [dir_bidx::W](dir_bidx::W) writer structure"]
impl crate::Writable for DIR_BIDX {}
#[doc = "Direction Sample Buffer Read Index Configure Register (16 directions * 2 values * 4 indices)"]
pub mod dir_bidx;
#[doc = "FIR0 pre-filter coefficients\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pre_fir0_coef](pre_fir0_coef) module"]
pub type PRE_FIR0_COEF = crate::Reg<u32, _PRE_FIR0_COEF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRE_FIR0_COEF;
#[doc = "`read()` method returns [pre_fir0_coef::R](pre_fir0_coef::R) reader structure"]
impl crate::Readable for PRE_FIR0_COEF {}
#[doc = "`write(|w| ..)` method takes [pre_fir0_coef::W](pre_fir0_coef::W) writer structure"]
impl crate::Writable for PRE_FIR0_COEF {}
#[doc = "FIR0 pre-filter coefficients"]
pub mod pre_fir0_coef;
#[doc = "FIR0 post-filter coefficients\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [post_fir0_coef](post_fir0_coef) module"]
pub type POST_FIR0_COEF = crate::Reg<u32, _POST_FIR0_COEF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POST_FIR0_COEF;
#[doc = "`read()` method returns [post_fir0_coef::R](post_fir0_coef::R) reader structure"]
impl crate::Readable for POST_FIR0_COEF {}
#[doc = "`write(|w| ..)` method takes [post_fir0_coef::W](post_fir0_coef::W) writer structure"]
impl crate::Writable for POST_FIR0_COEF {}
#[doc = "FIR0 post-filter coefficients"]
pub mod post_fir0_coef;
#[doc = "FIR1 pre-filter coeffecients\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pre_fir1_coef](pre_fir1_coef) module"]
pub type PRE_FIR1_COEF = crate::Reg<u32, _PRE_FIR1_COEF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRE_FIR1_COEF;
#[doc = "`read()` method returns [pre_fir1_coef::R](pre_fir1_coef::R) reader structure"]
impl crate::Readable for PRE_FIR1_COEF {}
#[doc = "`write(|w| ..)` method takes [pre_fir1_coef::W](pre_fir1_coef::W) writer structure"]
impl crate::Writable for PRE_FIR1_COEF {}
#[doc = "FIR1 pre-filter coeffecients"]
pub mod pre_fir1_coef;
#[doc = "FIR1 post-filter coefficients\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [post_fir1_coef](post_fir1_coef) module"]
pub type POST_FIR1_COEF = crate::Reg<u32, _POST_FIR1_COEF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POST_FIR1_COEF;
#[doc = "`read()` method returns [post_fir1_coef::R](post_fir1_coef::R) reader structure"]
impl crate::Readable for POST_FIR1_COEF {}
#[doc = "`write(|w| ..)` method takes [post_fir1_coef::W](post_fir1_coef::W) writer structure"]
impl crate::Writable for POST_FIR1_COEF {}
#[doc = "FIR1 post-filter coefficients"]
pub mod post_fir1_coef;
#[doc = "Downsize Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dwsz_cfg](dwsz_cfg) module"]
pub type DWSZ_CFG = crate::Reg<u32, _DWSZ_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DWSZ_CFG;
#[doc = "`read()` method returns [dwsz_cfg::R](dwsz_cfg::R) reader structure"]
impl crate::Readable for DWSZ_CFG {}
#[doc = "`write(|w| ..)` method takes [dwsz_cfg::W](dwsz_cfg::W) writer structure"]
impl crate::Writable for DWSZ_CFG {}
#[doc = "Downsize Config Register"]
pub mod dwsz_cfg;
#[doc = "FFT Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fft_cfg](fft_cfg) module"]
pub type FFT_CFG = crate::Reg<u32, _FFT_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFT_CFG;
#[doc = "`read()` method returns [fft_cfg::R](fft_cfg::R) reader structure"]
impl crate::Readable for FFT_CFG {}
#[doc = "`write(|w| ..)` method takes [fft_cfg::W](fft_cfg::W) writer structure"]
impl crate::Writable for FFT_CFG {}
#[doc = "FFT Config Register"]
pub mod fft_cfg;
#[doc = "Read register for DMA to sample-out buffers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sobuf_dma_rdata](sobuf_dma_rdata) module"]
pub type SOBUF_DMA_RDATA = crate::Reg<u32, _SOBUF_DMA_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOBUF_DMA_RDATA;
#[doc = "`read()` method returns [sobuf_dma_rdata::R](sobuf_dma_rdata::R) reader structure"]
impl crate::Readable for SOBUF_DMA_RDATA {}
#[doc = "`write(|w| ..)` method takes [sobuf_dma_rdata::W](sobuf_dma_rdata::W) writer structure"]
impl crate::Writable for SOBUF_DMA_RDATA {}
#[doc = "Read register for DMA to sample-out buffers"]
pub mod sobuf_dma_rdata;
#[doc = "Read register for DMA to voice-out buffers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vobuf_dma_rdata](vobuf_dma_rdata) module"]
pub type VOBUF_DMA_RDATA = crate::Reg<u32, _VOBUF_DMA_RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VOBUF_DMA_RDATA;
#[doc = "`read()` method returns [vobuf_dma_rdata::R](vobuf_dma_rdata::R) reader structure"]
impl crate::Readable for VOBUF_DMA_RDATA {}
#[doc = "`write(|w| ..)` method takes [vobuf_dma_rdata::W](vobuf_dma_rdata::W) writer structure"]
impl crate::Writable for VOBUF_DMA_RDATA {}
#[doc = "Read register for DMA to voice-out buffers"]
pub mod vobuf_dma_rdata;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_stat](int_stat) module"]
pub type INT_STAT = crate::Reg<u32, _INT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STAT;
#[doc = "`read()` method returns [int_stat::R](int_stat::R) reader structure"]
impl crate::Readable for INT_STAT {}
#[doc = "`write(|w| ..)` method takes [int_stat::W](int_stat::W) writer structure"]
impl crate::Writable for INT_STAT {}
#[doc = "Interrupt Status Register"]
pub mod int_stat;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_mask](int_mask) module"]
pub type INT_MASK = crate::Reg<u32, _INT_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_MASK;
#[doc = "`read()` method returns [int_mask::R](int_mask::R) reader structure"]
impl crate::Readable for INT_MASK {}
#[doc = "`write(|w| ..)` method takes [int_mask::W](int_mask::W) writer structure"]
impl crate::Writable for INT_MASK {}
#[doc = "Interrupt Mask Register"]
pub mod int_mask;
#[doc = "Saturation Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sat_counter](sat_counter) module"]
pub type SAT_COUNTER = crate::Reg<u32, _SAT_COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAT_COUNTER;
#[doc = "`read()` method returns [sat_counter::R](sat_counter::R) reader structure"]
impl crate::Readable for SAT_COUNTER {}
#[doc = "`write(|w| ..)` method takes [sat_counter::W](sat_counter::W) writer structure"]
impl crate::Writable for SAT_COUNTER {}
#[doc = "Saturation Counter"]
pub mod sat_counter;
#[doc = "Saturation Limits\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sat_limits](sat_limits) module"]
pub type SAT_LIMITS = crate::Reg<u32, _SAT_LIMITS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAT_LIMITS;
#[doc = "`read()` method returns [sat_limits::R](sat_limits::R) reader structure"]
impl crate::Readable for SAT_LIMITS {}
#[doc = "`write(|w| ..)` method takes [sat_limits::W](sat_limits::W) writer structure"]
impl crate::Writable for SAT_LIMITS {}
#[doc = "Saturation Limits"]
pub mod sat_limits;
