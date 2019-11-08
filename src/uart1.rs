#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Receive Buffer Register / Divisor Latch (Low) / Transmit Holding Register (depending on context and R/W)"]
    pub rbr_dll_thr: RBR_DLL_THR,
    #[doc = "0x04 - Divisor Latch (High) / Interrupt Enable Register"]
    pub dlh_ier: DLH_IER,
    #[doc = "0x08 - FIFO Control Register / Interrupt Identification Register"]
    pub fcr_iir: FCR_IIR,
    #[doc = "0x0c - Line Control Register"]
    pub lcr: LCR,
    #[doc = "0x10 - Modem Control Register"]
    pub mcr: MCR,
    #[doc = "0x14 - Line Status Register"]
    pub lsr: LSR,
    #[doc = "0x18 - Modem Status Register"]
    pub msr: MSR,
    #[doc = "0x1c - Scratchpad Register"]
    pub scr: SCR,
    #[doc = "0x20 - Low Power Divisor Latch (Low) Register"]
    pub lpdll: LPDLL,
    #[doc = "0x24 - Low Power Divisor Latch (High) Register"]
    pub lpdlh: LPDLH,
    _reserved10: [u8; 8usize],
    #[doc = "0x30 - Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)"]
    pub srbr_sthr: [SRBR_STHR; 16],
    #[doc = "0x70 - FIFO Access Register"]
    pub far: FAR,
    #[doc = "0x74 - Transmit FIFO Read Register"]
    pub tfr: TFR,
    #[doc = "0x78 - Receive FIFO Write Register"]
    pub rfw: RFW,
    #[doc = "0x7c - UART Status Register"]
    pub usr: USR,
    #[doc = "0x80 - Transmit FIFO Level"]
    pub tfl: TFL,
    #[doc = "0x84 - Receive FIFO Level"]
    pub rfl: RFL,
    #[doc = "0x88 - Software Reset Register"]
    pub srr: SRR,
    #[doc = "0x8c - Shadow Request to Send Register"]
    pub srts: SRTS,
    #[doc = "0x90 - Shadow Break Control Register"]
    pub sbcr: SBCR,
    #[doc = "0x94 - Shadow DMA Mode"]
    pub sdmam: SDMAM,
    #[doc = "0x98 - Shadow FIFO Enable"]
    pub sfe: SFE,
    #[doc = "0x9c - Shadow RCVR Trigger Register"]
    pub srt: SRT,
    #[doc = "0xa0 - Shadow TX Empty Trigger Register"]
    pub stet: STET,
    #[doc = "0xa4 - Halt TX Regster"]
    pub htx: HTX,
    #[doc = "0xa8 - DMA Software Acknowledge Register"]
    pub dmasa: DMASA,
    #[doc = "0xac - Transfer Control Register"]
    pub tcr: TCR,
    #[doc = "0xb0 - DE Enable Register"]
    pub de_en: DE_EN,
    #[doc = "0xb4 - RE Enable Register"]
    pub re_en: RE_EN,
    #[doc = "0xb8 - DE Assertion Time Register"]
    pub det: DET,
    #[doc = "0xbc - Turn-Around Time Register"]
    pub tat: TAT,
    #[doc = "0xc0 - Divisor Latch (Fractional) Register"]
    pub dlf: DLF,
    #[doc = "0xc4 - Receive-Mode Address Register"]
    pub rar: RAR,
    #[doc = "0xc8 - Transmit-Mode Address Register"]
    pub tar: TAR,
    #[doc = "0xcc - Line Control Register (Extended)"]
    pub lcr_ext: LCR_EXT,
    _reserved35: [u8; 36usize],
    #[doc = "0xf4 - Component Parameter Register"]
    pub cpr: CPR,
    #[doc = "0xf8 - UART Component Version"]
    pub ucv: UCV,
    #[doc = "0xfc - Component Type Register"]
    pub ctr: CTR,
}
#[doc = "Receive Buffer Register / Divisor Latch (Low) / Transmit Holding Register (depending on context and R/W)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rbr_dll_thr](rbr_dll_thr) module"]
pub type RBR_DLL_THR = crate::Reg<u32, _RBR_DLL_THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBR_DLL_THR;
#[doc = "`read()` method returns [rbr_dll_thr::R](rbr_dll_thr::R) reader structure"]
impl crate::Readable for RBR_DLL_THR {}
#[doc = "`write(|w| ..)` method takes [rbr_dll_thr::W](rbr_dll_thr::W) writer structure"]
impl crate::Writable for RBR_DLL_THR {}
#[doc = "Receive Buffer Register / Divisor Latch (Low) / Transmit Holding Register (depending on context and R/W)"]
pub mod rbr_dll_thr;
#[doc = "Divisor Latch (High) / Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dlh_ier](dlh_ier) module"]
pub type DLH_IER = crate::Reg<u32, _DLH_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLH_IER;
#[doc = "`read()` method returns [dlh_ier::R](dlh_ier::R) reader structure"]
impl crate::Readable for DLH_IER {}
#[doc = "`write(|w| ..)` method takes [dlh_ier::W](dlh_ier::W) writer structure"]
impl crate::Writable for DLH_IER {}
#[doc = "Divisor Latch (High) / Interrupt Enable Register"]
pub mod dlh_ier;
#[doc = "FIFO Control Register / Interrupt Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcr_iir](fcr_iir) module"]
pub type FCR_IIR = crate::Reg<u32, _FCR_IIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR_IIR;
#[doc = "`read()` method returns [fcr_iir::R](fcr_iir::R) reader structure"]
impl crate::Readable for FCR_IIR {}
#[doc = "`write(|w| ..)` method takes [fcr_iir::W](fcr_iir::W) writer structure"]
impl crate::Writable for FCR_IIR {}
#[doc = "FIFO Control Register / Interrupt Identification Register"]
pub mod fcr_iir;
#[doc = "Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lcr](lcr) module"]
pub type LCR = crate::Reg<u32, _LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCR;
#[doc = "`read()` method returns [lcr::R](lcr::R) reader structure"]
impl crate::Readable for LCR {}
#[doc = "`write(|w| ..)` method takes [lcr::W](lcr::W) writer structure"]
impl crate::Writable for LCR {}
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "Modem Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Modem Control Register"]
pub mod mcr;
#[doc = "Line Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lsr](lsr) module"]
pub type LSR = crate::Reg<u32, _LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSR;
#[doc = "`read()` method returns [lsr::R](lsr::R) reader structure"]
impl crate::Readable for LSR {}
#[doc = "`write(|w| ..)` method takes [lsr::W](lsr::W) writer structure"]
impl crate::Writable for LSR {}
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "Modem Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [msr](msr) module"]
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
#[doc = "`read()` method returns [msr::R](msr::R) reader structure"]
impl crate::Readable for MSR {}
#[doc = "`write(|w| ..)` method takes [msr::W](msr::W) writer structure"]
impl crate::Writable for MSR {}
#[doc = "Modem Status Register"]
pub mod msr;
#[doc = "Scratchpad Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "Scratchpad Register"]
pub mod scr;
#[doc = "Low Power Divisor Latch (Low) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpdll](lpdll) module"]
pub type LPDLL = crate::Reg<u32, _LPDLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPDLL;
#[doc = "`read()` method returns [lpdll::R](lpdll::R) reader structure"]
impl crate::Readable for LPDLL {}
#[doc = "`write(|w| ..)` method takes [lpdll::W](lpdll::W) writer structure"]
impl crate::Writable for LPDLL {}
#[doc = "Low Power Divisor Latch (Low) Register"]
pub mod lpdll;
#[doc = "Low Power Divisor Latch (High) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lpdlh](lpdlh) module"]
pub type LPDLH = crate::Reg<u32, _LPDLH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPDLH;
#[doc = "`read()` method returns [lpdlh::R](lpdlh::R) reader structure"]
impl crate::Readable for LPDLH {}
#[doc = "`write(|w| ..)` method takes [lpdlh::W](lpdlh::W) writer structure"]
impl crate::Writable for LPDLH {}
#[doc = "Low Power Divisor Latch (High) Register"]
pub mod lpdlh;
#[doc = "Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srbr_sthr](srbr_sthr) module"]
pub type SRBR_STHR = crate::Reg<u32, _SRBR_STHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRBR_STHR;
#[doc = "`read()` method returns [srbr_sthr::R](srbr_sthr::R) reader structure"]
impl crate::Readable for SRBR_STHR {}
#[doc = "`write(|w| ..)` method takes [srbr_sthr::W](srbr_sthr::W) writer structure"]
impl crate::Writable for SRBR_STHR {}
#[doc = "Shadow Receive Buffer Register / Shadow Transmit Holding Register (depending on R/W)"]
pub mod srbr_sthr;
#[doc = "FIFO Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [far](far) module"]
pub type FAR = crate::Reg<u32, _FAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAR;
#[doc = "`read()` method returns [far::R](far::R) reader structure"]
impl crate::Readable for FAR {}
#[doc = "`write(|w| ..)` method takes [far::W](far::W) writer structure"]
impl crate::Writable for FAR {}
#[doc = "FIFO Access Register"]
pub mod far;
#[doc = "Transmit FIFO Read Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tfr](tfr) module"]
pub type TFR = crate::Reg<u32, _TFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFR;
#[doc = "`read()` method returns [tfr::R](tfr::R) reader structure"]
impl crate::Readable for TFR {}
#[doc = "`write(|w| ..)` method takes [tfr::W](tfr::W) writer structure"]
impl crate::Writable for TFR {}
#[doc = "Transmit FIFO Read Register"]
pub mod tfr;
#[doc = "Receive FIFO Write Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfw](rfw) module"]
pub type RFW = crate::Reg<u32, _RFW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFW;
#[doc = "`read()` method returns [rfw::R](rfw::R) reader structure"]
impl crate::Readable for RFW {}
#[doc = "`write(|w| ..)` method takes [rfw::W](rfw::W) writer structure"]
impl crate::Writable for RFW {}
#[doc = "Receive FIFO Write Register"]
pub mod rfw;
#[doc = "UART Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usr](usr) module"]
pub type USR = crate::Reg<u32, _USR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USR;
#[doc = "`read()` method returns [usr::R](usr::R) reader structure"]
impl crate::Readable for USR {}
#[doc = "`write(|w| ..)` method takes [usr::W](usr::W) writer structure"]
impl crate::Writable for USR {}
#[doc = "UART Status Register"]
pub mod usr;
#[doc = "Transmit FIFO Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tfl](tfl) module"]
pub type TFL = crate::Reg<u32, _TFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFL;
#[doc = "`read()` method returns [tfl::R](tfl::R) reader structure"]
impl crate::Readable for TFL {}
#[doc = "`write(|w| ..)` method takes [tfl::W](tfl::W) writer structure"]
impl crate::Writable for TFL {}
#[doc = "Transmit FIFO Level"]
pub mod tfl;
#[doc = "Receive FIFO Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rfl](rfl) module"]
pub type RFL = crate::Reg<u32, _RFL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RFL;
#[doc = "`read()` method returns [rfl::R](rfl::R) reader structure"]
impl crate::Readable for RFL {}
#[doc = "`write(|w| ..)` method takes [rfl::W](rfl::W) writer structure"]
impl crate::Writable for RFL {}
#[doc = "Receive FIFO Level"]
pub mod rfl;
#[doc = "Software Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srr](srr) module"]
pub type SRR = crate::Reg<u32, _SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRR;
#[doc = "`read()` method returns [srr::R](srr::R) reader structure"]
impl crate::Readable for SRR {}
#[doc = "`write(|w| ..)` method takes [srr::W](srr::W) writer structure"]
impl crate::Writable for SRR {}
#[doc = "Software Reset Register"]
pub mod srr;
#[doc = "Shadow Request to Send Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srts](srts) module"]
pub type SRTS = crate::Reg<u32, _SRTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRTS;
#[doc = "`read()` method returns [srts::R](srts::R) reader structure"]
impl crate::Readable for SRTS {}
#[doc = "`write(|w| ..)` method takes [srts::W](srts::W) writer structure"]
impl crate::Writable for SRTS {}
#[doc = "Shadow Request to Send Register"]
pub mod srts;
#[doc = "Shadow Break Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sbcr](sbcr) module"]
pub type SBCR = crate::Reg<u32, _SBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SBCR;
#[doc = "`read()` method returns [sbcr::R](sbcr::R) reader structure"]
impl crate::Readable for SBCR {}
#[doc = "`write(|w| ..)` method takes [sbcr::W](sbcr::W) writer structure"]
impl crate::Writable for SBCR {}
#[doc = "Shadow Break Control Register"]
pub mod sbcr;
#[doc = "Shadow DMA Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdmam](sdmam) module"]
pub type SDMAM = crate::Reg<u32, _SDMAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMAM;
#[doc = "`read()` method returns [sdmam::R](sdmam::R) reader structure"]
impl crate::Readable for SDMAM {}
#[doc = "`write(|w| ..)` method takes [sdmam::W](sdmam::W) writer structure"]
impl crate::Writable for SDMAM {}
#[doc = "Shadow DMA Mode"]
pub mod sdmam;
#[doc = "Shadow FIFO Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sfe](sfe) module"]
pub type SFE = crate::Reg<u32, _SFE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFE;
#[doc = "`read()` method returns [sfe::R](sfe::R) reader structure"]
impl crate::Readable for SFE {}
#[doc = "`write(|w| ..)` method takes [sfe::W](sfe::W) writer structure"]
impl crate::Writable for SFE {}
#[doc = "Shadow FIFO Enable"]
pub mod sfe;
#[doc = "Shadow RCVR Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [srt](srt) module"]
pub type SRT = crate::Reg<u32, _SRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRT;
#[doc = "`read()` method returns [srt::R](srt::R) reader structure"]
impl crate::Readable for SRT {}
#[doc = "`write(|w| ..)` method takes [srt::W](srt::W) writer structure"]
impl crate::Writable for SRT {}
#[doc = "Shadow RCVR Trigger Register"]
pub mod srt;
#[doc = "Shadow TX Empty Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stet](stet) module"]
pub type STET = crate::Reg<u32, _STET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STET;
#[doc = "`read()` method returns [stet::R](stet::R) reader structure"]
impl crate::Readable for STET {}
#[doc = "`write(|w| ..)` method takes [stet::W](stet::W) writer structure"]
impl crate::Writable for STET {}
#[doc = "Shadow TX Empty Trigger Register"]
pub mod stet;
#[doc = "Halt TX Regster\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [htx](htx) module"]
pub type HTX = crate::Reg<u32, _HTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HTX;
#[doc = "`read()` method returns [htx::R](htx::R) reader structure"]
impl crate::Readable for HTX {}
#[doc = "`write(|w| ..)` method takes [htx::W](htx::W) writer structure"]
impl crate::Writable for HTX {}
#[doc = "Halt TX Regster"]
pub mod htx;
#[doc = "DMA Software Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmasa](dmasa) module"]
pub type DMASA = crate::Reg<u32, _DMASA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASA;
#[doc = "`read()` method returns [dmasa::R](dmasa::R) reader structure"]
impl crate::Readable for DMASA {}
#[doc = "`write(|w| ..)` method takes [dmasa::W](dmasa::W) writer structure"]
impl crate::Writable for DMASA {}
#[doc = "DMA Software Acknowledge Register"]
pub mod dmasa;
#[doc = "Transfer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "Transfer Control Register"]
pub mod tcr;
#[doc = "DE Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [de_en](de_en) module"]
pub type DE_EN = crate::Reg<u32, _DE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DE_EN;
#[doc = "`read()` method returns [de_en::R](de_en::R) reader structure"]
impl crate::Readable for DE_EN {}
#[doc = "`write(|w| ..)` method takes [de_en::W](de_en::W) writer structure"]
impl crate::Writable for DE_EN {}
#[doc = "DE Enable Register"]
pub mod de_en;
#[doc = "RE Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [re_en](re_en) module"]
pub type RE_EN = crate::Reg<u32, _RE_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RE_EN;
#[doc = "`read()` method returns [re_en::R](re_en::R) reader structure"]
impl crate::Readable for RE_EN {}
#[doc = "`write(|w| ..)` method takes [re_en::W](re_en::W) writer structure"]
impl crate::Writable for RE_EN {}
#[doc = "RE Enable Register"]
pub mod re_en;
#[doc = "DE Assertion Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [det](det) module"]
pub type DET = crate::Reg<u32, _DET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DET;
#[doc = "`read()` method returns [det::R](det::R) reader structure"]
impl crate::Readable for DET {}
#[doc = "`write(|w| ..)` method takes [det::W](det::W) writer structure"]
impl crate::Writable for DET {}
#[doc = "DE Assertion Time Register"]
pub mod det;
#[doc = "Turn-Around Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tat](tat) module"]
pub type TAT = crate::Reg<u32, _TAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAT;
#[doc = "`read()` method returns [tat::R](tat::R) reader structure"]
impl crate::Readable for TAT {}
#[doc = "`write(|w| ..)` method takes [tat::W](tat::W) writer structure"]
impl crate::Writable for TAT {}
#[doc = "Turn-Around Time Register"]
pub mod tat;
#[doc = "Divisor Latch (Fractional) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dlf](dlf) module"]
pub type DLF = crate::Reg<u32, _DLF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLF;
#[doc = "`read()` method returns [dlf::R](dlf::R) reader structure"]
impl crate::Readable for DLF {}
#[doc = "`write(|w| ..)` method takes [dlf::W](dlf::W) writer structure"]
impl crate::Writable for DLF {}
#[doc = "Divisor Latch (Fractional) Register"]
pub mod dlf;
#[doc = "Receive-Mode Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rar](rar) module"]
pub type RAR = crate::Reg<u32, _RAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAR;
#[doc = "`read()` method returns [rar::R](rar::R) reader structure"]
impl crate::Readable for RAR {}
#[doc = "`write(|w| ..)` method takes [rar::W](rar::W) writer structure"]
impl crate::Writable for RAR {}
#[doc = "Receive-Mode Address Register"]
pub mod rar;
#[doc = "Transmit-Mode Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tar](tar) module"]
pub type TAR = crate::Reg<u32, _TAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TAR;
#[doc = "`read()` method returns [tar::R](tar::R) reader structure"]
impl crate::Readable for TAR {}
#[doc = "`write(|w| ..)` method takes [tar::W](tar::W) writer structure"]
impl crate::Writable for TAR {}
#[doc = "Transmit-Mode Address Register"]
pub mod tar;
#[doc = "Line Control Register (Extended)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lcr_ext](lcr_ext) module"]
pub type LCR_EXT = crate::Reg<u32, _LCR_EXT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCR_EXT;
#[doc = "`read()` method returns [lcr_ext::R](lcr_ext::R) reader structure"]
impl crate::Readable for LCR_EXT {}
#[doc = "`write(|w| ..)` method takes [lcr_ext::W](lcr_ext::W) writer structure"]
impl crate::Writable for LCR_EXT {}
#[doc = "Line Control Register (Extended)"]
pub mod lcr_ext;
#[doc = "Component Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpr](cpr) module"]
pub type CPR = crate::Reg<u32, _CPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPR;
#[doc = "`read()` method returns [cpr::R](cpr::R) reader structure"]
impl crate::Readable for CPR {}
#[doc = "`write(|w| ..)` method takes [cpr::W](cpr::W) writer structure"]
impl crate::Writable for CPR {}
#[doc = "Component Parameter Register"]
pub mod cpr;
#[doc = "UART Component Version\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ucv](ucv) module"]
pub type UCV = crate::Reg<u32, _UCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCV;
#[doc = "`read()` method returns [ucv::R](ucv::R) reader structure"]
impl crate::Readable for UCV {}
#[doc = "`write(|w| ..)` method takes [ucv::W](ucv::W) writer structure"]
impl crate::Writable for UCV {}
#[doc = "UART Component Version"]
pub mod ucv;
#[doc = "Component Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctr](ctr) module"]
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
#[doc = "`read()` method returns [ctr::R](ctr::R) reader structure"]
impl crate::Readable for CTR {}
#[doc = "`write(|w| ..)` method takes [ctr::W](ctr::W) writer structure"]
impl crate::Writable for CTR {}
#[doc = "Component Type Register"]
pub mod ctr;
