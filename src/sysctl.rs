#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Git short commit id"]
    pub git_id: GIT_ID,
    #[doc = "0x04 - System clock base frequency"]
    pub clk_freq: CLK_FREQ,
    #[doc = "0x08 - PLL0 controller"]
    pub pll0: PLL0,
    #[doc = "0x0c - PLL1 controller"]
    pub pll1: PLL1,
    #[doc = "0x10 - PLL2 controller"]
    pub pll2: PLL2,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - PLL lock tester"]
    pub pll_lock: PLL_LOCK,
    #[doc = "0x1c - AXI ROM detector"]
    pub rom_error: ROM_ERROR,
    #[doc = "0x20 - Clock select controller 0"]
    pub clk_sel0: CLK_SEL0,
    #[doc = "0x24 - Clock select controller 1"]
    pub clk_sel1: CLK_SEL1,
    #[doc = "0x28 - Central clock enable"]
    pub clk_en_cent: CLK_EN_CENT,
    #[doc = "0x2c - Peripheral clock enable"]
    pub clk_en_peri: CLK_EN_PERI,
    #[doc = "0x30 - Soft reset ctrl"]
    pub soft_reset: SOFT_RESET,
    #[doc = "0x34 - Peripheral reset controller"]
    pub peri_reset: PERI_RESET,
    #[doc = "0x38 - Clock threshold controller 0"]
    pub clk_th0: CLK_TH0,
    #[doc = "0x3c - Clock threshold controller 1"]
    pub clk_th1: CLK_TH1,
    #[doc = "0x40 - Clock threshold controller 2"]
    pub clk_th2: CLK_TH2,
    #[doc = "0x44 - Clock threshold controller 3"]
    pub clk_th3: CLK_TH3,
    #[doc = "0x48 - Clock threshold controller 4"]
    pub clk_th4: CLK_TH4,
    #[doc = "0x4c - Clock threshold controller 5"]
    pub clk_th5: CLK_TH5,
    #[doc = "0x50 - Clock threshold controller 6"]
    pub clk_th6: CLK_TH6,
    #[doc = "0x54 - Miscellaneous controller"]
    pub misc: MISC,
    #[doc = "0x58 - Peripheral controller"]
    pub peri: PERI,
    #[doc = "0x5c - SPI sleep controller"]
    pub spi_sleep: SPI_SLEEP,
    #[doc = "0x60 - Reset source status"]
    pub reset_status: RESET_STATUS,
    #[doc = "0x64 - DMA handshake selector"]
    pub dma_sel0: DMA_SEL0,
    #[doc = "0x68 - DMA handshake selector"]
    pub dma_sel1: DMA_SEL1,
    #[doc = "0x6c - IO Power Mode Select controller"]
    pub power_sel: POWER_SEL,
}
#[doc = "Git short commit id\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [git_id](git_id) module"]
pub type GIT_ID = crate::Reg<u32, _GIT_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GIT_ID;
#[doc = "`read()` method returns [git_id::R](git_id::R) reader structure"]
impl crate::Readable for GIT_ID {}
#[doc = "`write(|w| ..)` method takes [git_id::W](git_id::W) writer structure"]
impl crate::Writable for GIT_ID {}
#[doc = "Git short commit id"]
pub mod git_id;
#[doc = "System clock base frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_freq](clk_freq) module"]
pub type CLK_FREQ = crate::Reg<u32, _CLK_FREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_FREQ;
#[doc = "`read()` method returns [clk_freq::R](clk_freq::R) reader structure"]
impl crate::Readable for CLK_FREQ {}
#[doc = "`write(|w| ..)` method takes [clk_freq::W](clk_freq::W) writer structure"]
impl crate::Writable for CLK_FREQ {}
#[doc = "System clock base frequency"]
pub mod clk_freq;
#[doc = "PLL0 controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll0](pll0) module"]
pub type PLL0 = crate::Reg<u32, _PLL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0;
#[doc = "`read()` method returns [pll0::R](pll0::R) reader structure"]
impl crate::Readable for PLL0 {}
#[doc = "`write(|w| ..)` method takes [pll0::W](pll0::W) writer structure"]
impl crate::Writable for PLL0 {}
#[doc = "PLL0 controller"]
pub mod pll0;
#[doc = "PLL1 controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll1](pll1) module"]
pub type PLL1 = crate::Reg<u32, _PLL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1;
#[doc = "`read()` method returns [pll1::R](pll1::R) reader structure"]
impl crate::Readable for PLL1 {}
#[doc = "`write(|w| ..)` method takes [pll1::W](pll1::W) writer structure"]
impl crate::Writable for PLL1 {}
#[doc = "PLL1 controller"]
pub mod pll1;
#[doc = "PLL2 controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll2](pll2) module"]
pub type PLL2 = crate::Reg<u32, _PLL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL2;
#[doc = "`read()` method returns [pll2::R](pll2::R) reader structure"]
impl crate::Readable for PLL2 {}
#[doc = "`write(|w| ..)` method takes [pll2::W](pll2::W) writer structure"]
impl crate::Writable for PLL2 {}
#[doc = "PLL2 controller"]
pub mod pll2;
#[doc = "PLL lock tester\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pll_lock](pll_lock) module"]
pub type PLL_LOCK = crate::Reg<u32, _PLL_LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL_LOCK;
#[doc = "`read()` method returns [pll_lock::R](pll_lock::R) reader structure"]
impl crate::Readable for PLL_LOCK {}
#[doc = "`write(|w| ..)` method takes [pll_lock::W](pll_lock::W) writer structure"]
impl crate::Writable for PLL_LOCK {}
#[doc = "PLL lock tester"]
pub mod pll_lock;
#[doc = "AXI ROM detector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rom_error](rom_error) module"]
pub type ROM_ERROR = crate::Reg<u32, _ROM_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROM_ERROR;
#[doc = "`read()` method returns [rom_error::R](rom_error::R) reader structure"]
impl crate::Readable for ROM_ERROR {}
#[doc = "`write(|w| ..)` method takes [rom_error::W](rom_error::W) writer structure"]
impl crate::Writable for ROM_ERROR {}
#[doc = "AXI ROM detector"]
pub mod rom_error;
#[doc = "Clock select controller 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_sel0](clk_sel0) module"]
pub type CLK_SEL0 = crate::Reg<u32, _CLK_SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SEL0;
#[doc = "`read()` method returns [clk_sel0::R](clk_sel0::R) reader structure"]
impl crate::Readable for CLK_SEL0 {}
#[doc = "`write(|w| ..)` method takes [clk_sel0::W](clk_sel0::W) writer structure"]
impl crate::Writable for CLK_SEL0 {}
#[doc = "Clock select controller 0"]
pub mod clk_sel0;
#[doc = "Clock select controller 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_sel1](clk_sel1) module"]
pub type CLK_SEL1 = crate::Reg<u32, _CLK_SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_SEL1;
#[doc = "`read()` method returns [clk_sel1::R](clk_sel1::R) reader structure"]
impl crate::Readable for CLK_SEL1 {}
#[doc = "`write(|w| ..)` method takes [clk_sel1::W](clk_sel1::W) writer structure"]
impl crate::Writable for CLK_SEL1 {}
#[doc = "Clock select controller 1"]
pub mod clk_sel1;
#[doc = "Central clock enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_en_cent](clk_en_cent) module"]
pub type CLK_EN_CENT = crate::Reg<u32, _CLK_EN_CENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_EN_CENT;
#[doc = "`read()` method returns [clk_en_cent::R](clk_en_cent::R) reader structure"]
impl crate::Readable for CLK_EN_CENT {}
#[doc = "`write(|w| ..)` method takes [clk_en_cent::W](clk_en_cent::W) writer structure"]
impl crate::Writable for CLK_EN_CENT {}
#[doc = "Central clock enable"]
pub mod clk_en_cent;
#[doc = "Peripheral clock enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_en_peri](clk_en_peri) module"]
pub type CLK_EN_PERI = crate::Reg<u32, _CLK_EN_PERI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_EN_PERI;
#[doc = "`read()` method returns [clk_en_peri::R](clk_en_peri::R) reader structure"]
impl crate::Readable for CLK_EN_PERI {}
#[doc = "`write(|w| ..)` method takes [clk_en_peri::W](clk_en_peri::W) writer structure"]
impl crate::Writable for CLK_EN_PERI {}
#[doc = "Peripheral clock enable"]
pub mod clk_en_peri;
#[doc = "Soft reset ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [soft_reset](soft_reset) module"]
pub type SOFT_RESET = crate::Reg<u32, _SOFT_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOFT_RESET;
#[doc = "`read()` method returns [soft_reset::R](soft_reset::R) reader structure"]
impl crate::Readable for SOFT_RESET {}
#[doc = "`write(|w| ..)` method takes [soft_reset::W](soft_reset::W) writer structure"]
impl crate::Writable for SOFT_RESET {}
#[doc = "Soft reset ctrl"]
pub mod soft_reset;
#[doc = "Peripheral reset controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [peri_reset](peri_reset) module"]
pub type PERI_RESET = crate::Reg<u32, _PERI_RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI_RESET;
#[doc = "`read()` method returns [peri_reset::R](peri_reset::R) reader structure"]
impl crate::Readable for PERI_RESET {}
#[doc = "`write(|w| ..)` method takes [peri_reset::W](peri_reset::W) writer structure"]
impl crate::Writable for PERI_RESET {}
#[doc = "Peripheral reset controller"]
pub mod peri_reset;
#[doc = "Clock threshold controller 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_th0](clk_th0) module"]
pub type CLK_TH0 = crate::Reg<u32, _CLK_TH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TH0;
#[doc = "`read()` method returns [clk_th0::R](clk_th0::R) reader structure"]
impl crate::Readable for CLK_TH0 {}
#[doc = "`write(|w| ..)` method takes [clk_th0::W](clk_th0::W) writer structure"]
impl crate::Writable for CLK_TH0 {}
#[doc = "Clock threshold controller 0"]
pub mod clk_th0;
#[doc = "Clock threshold controller 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_th1](clk_th1) module"]
pub type CLK_TH1 = crate::Reg<u32, _CLK_TH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TH1;
#[doc = "`read()` method returns [clk_th1::R](clk_th1::R) reader structure"]
impl crate::Readable for CLK_TH1 {}
#[doc = "`write(|w| ..)` method takes [clk_th1::W](clk_th1::W) writer structure"]
impl crate::Writable for CLK_TH1 {}
#[doc = "Clock threshold controller 1"]
pub mod clk_th1;
#[doc = "Clock threshold controller 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_th2](clk_th2) module"]
pub type CLK_TH2 = crate::Reg<u32, _CLK_TH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TH2;
#[doc = "`read()` method returns [clk_th2::R](clk_th2::R) reader structure"]
impl crate::Readable for CLK_TH2 {}
#[doc = "`write(|w| ..)` method takes [clk_th2::W](clk_th2::W) writer structure"]
impl crate::Writable for CLK_TH2 {}
#[doc = "Clock threshold controller 2"]
pub mod clk_th2;
#[doc = "Clock threshold controller 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_th3](clk_th3) module"]
pub type CLK_TH3 = crate::Reg<u32, _CLK_TH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TH3;
#[doc = "`read()` method returns [clk_th3::R](clk_th3::R) reader structure"]
impl crate::Readable for CLK_TH3 {}
#[doc = "`write(|w| ..)` method takes [clk_th3::W](clk_th3::W) writer structure"]
impl crate::Writable for CLK_TH3 {}
#[doc = "Clock threshold controller 3"]
pub mod clk_th3;
#[doc = "Clock threshold controller 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_th4](clk_th4) module"]
pub type CLK_TH4 = crate::Reg<u32, _CLK_TH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TH4;
#[doc = "`read()` method returns [clk_th4::R](clk_th4::R) reader structure"]
impl crate::Readable for CLK_TH4 {}
#[doc = "`write(|w| ..)` method takes [clk_th4::W](clk_th4::W) writer structure"]
impl crate::Writable for CLK_TH4 {}
#[doc = "Clock threshold controller 4"]
pub mod clk_th4;
#[doc = "Clock threshold controller 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_th5](clk_th5) module"]
pub type CLK_TH5 = crate::Reg<u32, _CLK_TH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TH5;
#[doc = "`read()` method returns [clk_th5::R](clk_th5::R) reader structure"]
impl crate::Readable for CLK_TH5 {}
#[doc = "`write(|w| ..)` method takes [clk_th5::W](clk_th5::W) writer structure"]
impl crate::Writable for CLK_TH5 {}
#[doc = "Clock threshold controller 5"]
pub mod clk_th5;
#[doc = "Clock threshold controller 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clk_th6](clk_th6) module"]
pub type CLK_TH6 = crate::Reg<u32, _CLK_TH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLK_TH6;
#[doc = "`read()` method returns [clk_th6::R](clk_th6::R) reader structure"]
impl crate::Readable for CLK_TH6 {}
#[doc = "`write(|w| ..)` method takes [clk_th6::W](clk_th6::W) writer structure"]
impl crate::Writable for CLK_TH6 {}
#[doc = "Clock threshold controller 6"]
pub mod clk_th6;
#[doc = "Miscellaneous controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [misc](misc) module"]
pub type MISC = crate::Reg<u32, _MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC;
#[doc = "`read()` method returns [misc::R](misc::R) reader structure"]
impl crate::Readable for MISC {}
#[doc = "`write(|w| ..)` method takes [misc::W](misc::W) writer structure"]
impl crate::Writable for MISC {}
#[doc = "Miscellaneous controller"]
pub mod misc;
#[doc = "Peripheral controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [peri](peri) module"]
pub type PERI = crate::Reg<u32, _PERI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERI;
#[doc = "`read()` method returns [peri::R](peri::R) reader structure"]
impl crate::Readable for PERI {}
#[doc = "`write(|w| ..)` method takes [peri::W](peri::W) writer structure"]
impl crate::Writable for PERI {}
#[doc = "Peripheral controller"]
pub mod peri;
#[doc = "SPI sleep controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spi_sleep](spi_sleep) module"]
pub type SPI_SLEEP = crate::Reg<u32, _SPI_SLEEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SLEEP;
#[doc = "`read()` method returns [spi_sleep::R](spi_sleep::R) reader structure"]
impl crate::Readable for SPI_SLEEP {}
#[doc = "`write(|w| ..)` method takes [spi_sleep::W](spi_sleep::W) writer structure"]
impl crate::Writable for SPI_SLEEP {}
#[doc = "SPI sleep controller"]
pub mod spi_sleep;
#[doc = "Reset source status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reset_status](reset_status) module"]
pub type RESET_STATUS = crate::Reg<u32, _RESET_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET_STATUS;
#[doc = "`read()` method returns [reset_status::R](reset_status::R) reader structure"]
impl crate::Readable for RESET_STATUS {}
#[doc = "`write(|w| ..)` method takes [reset_status::W](reset_status::W) writer structure"]
impl crate::Writable for RESET_STATUS {}
#[doc = "Reset source status"]
pub mod reset_status;
#[doc = "DMA handshake selector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_sel0](dma_sel0) module"]
pub type DMA_SEL0 = crate::Reg<u32, _DMA_SEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SEL0;
#[doc = "`read()` method returns [dma_sel0::R](dma_sel0::R) reader structure"]
impl crate::Readable for DMA_SEL0 {}
#[doc = "`write(|w| ..)` method takes [dma_sel0::W](dma_sel0::W) writer structure"]
impl crate::Writable for DMA_SEL0 {}
#[doc = "DMA handshake selector"]
pub mod dma_sel0;
#[doc = "DMA handshake selector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma_sel1](dma_sel1) module"]
pub type DMA_SEL1 = crate::Reg<u32, _DMA_SEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA_SEL1;
#[doc = "`read()` method returns [dma_sel1::R](dma_sel1::R) reader structure"]
impl crate::Readable for DMA_SEL1 {}
#[doc = "`write(|w| ..)` method takes [dma_sel1::W](dma_sel1::W) writer structure"]
impl crate::Writable for DMA_SEL1 {}
#[doc = "DMA handshake selector"]
pub mod dma_sel1;
#[doc = "IO Power Mode Select controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [power_sel](power_sel) module"]
pub type POWER_SEL = crate::Reg<u32, _POWER_SEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER_SEL;
#[doc = "`read()` method returns [power_sel::R](power_sel::R) reader structure"]
impl crate::Readable for POWER_SEL {}
#[doc = "`write(|w| ..)` method takes [power_sel::W](power_sel::W) writer structure"]
impl crate::Writable for POWER_SEL {}
#[doc = "IO Power Mode Select controller"]
pub mod power_sel;
