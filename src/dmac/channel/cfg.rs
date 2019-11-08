#[doc = "Reader of register cfg"]
pub type R = crate::R<u64, super::CFG>;
#[doc = "Writer for register cfg"]
pub type W = crate::W<u64, super::CFG>;
#[doc = "Register cfg `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u64;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Source multi-block transfer type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_MULTBLK_TYPE_A {
    #[doc = "0: Continuous multi-block type"]
    CONTIGUOUS,
    #[doc = "1: Reload multi-block type"]
    RELOAD,
    #[doc = "2: Shadow register based multi-block type"]
    SHADOW_REGISTER,
    #[doc = "3: Linked list based multi-block type"]
    LINKED_LIST,
}
impl From<SRC_MULTBLK_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_MULTBLK_TYPE_A) -> Self {
        match variant {
            SRC_MULTBLK_TYPE_A::CONTIGUOUS => 0,
            SRC_MULTBLK_TYPE_A::RELOAD => 1,
            SRC_MULTBLK_TYPE_A::SHADOW_REGISTER => 2,
            SRC_MULTBLK_TYPE_A::LINKED_LIST => 3,
        }
    }
}
#[doc = "Reader of field `src_multblk_type`"]
pub type SRC_MULTBLK_TYPE_R = crate::R<u8, SRC_MULTBLK_TYPE_A>;
impl SRC_MULTBLK_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_MULTBLK_TYPE_A {
        match self.bits {
            0 => SRC_MULTBLK_TYPE_A::CONTIGUOUS,
            1 => SRC_MULTBLK_TYPE_A::RELOAD,
            2 => SRC_MULTBLK_TYPE_A::SHADOW_REGISTER,
            3 => SRC_MULTBLK_TYPE_A::LINKED_LIST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTIGUOUS`"]
    #[inline(always)]
    pub fn is_contiguous(&self) -> bool {
        *self == SRC_MULTBLK_TYPE_A::CONTIGUOUS
    }
    #[doc = "Checks if the value of the field is `RELOAD`"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        *self == SRC_MULTBLK_TYPE_A::RELOAD
    }
    #[doc = "Checks if the value of the field is `SHADOW_REGISTER`"]
    #[inline(always)]
    pub fn is_shadow_register(&self) -> bool {
        *self == SRC_MULTBLK_TYPE_A::SHADOW_REGISTER
    }
    #[doc = "Checks if the value of the field is `LINKED_LIST`"]
    #[inline(always)]
    pub fn is_linked_list(&self) -> bool {
        *self == SRC_MULTBLK_TYPE_A::LINKED_LIST
    }
}
#[doc = "Write proxy for field `src_multblk_type`"]
pub struct SRC_MULTBLK_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_MULTBLK_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_MULTBLK_TYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Continuous multi-block type"]
    #[inline(always)]
    pub fn contiguous(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPE_A::CONTIGUOUS)
    }
    #[doc = "Reload multi-block type"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPE_A::RELOAD)
    }
    #[doc = "Shadow register based multi-block type"]
    #[inline(always)]
    pub fn shadow_register(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPE_A::SHADOW_REGISTER)
    }
    #[doc = "Linked list based multi-block type"]
    #[inline(always)]
    pub fn linked_list(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPE_A::LINKED_LIST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u64) & 0x03);
        self.w
    }
}
#[doc = "Destination multi-block transfer type"]
pub type DST_MULTBLK_TYPE_A = SRC_MULTBLK_TYPE_A;
#[doc = "Reader of field `dst_multblk_type`"]
pub type DST_MULTBLK_TYPE_R = crate::R<u8, SRC_MULTBLK_TYPE_A>;
#[doc = "Write proxy for field `dst_multblk_type`"]
pub struct DST_MULTBLK_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_MULTBLK_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_MULTBLK_TYPE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Continuous multi-block type"]
    #[inline(always)]
    pub fn contiguous(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPE_A::CONTIGUOUS)
    }
    #[doc = "Reload multi-block type"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPE_A::RELOAD)
    }
    #[doc = "Shadow register based multi-block type"]
    #[inline(always)]
    pub fn shadow_register(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPE_A::SHADOW_REGISTER)
    }
    #[doc = "Linked list based multi-block type"]
    #[inline(always)]
    pub fn linked_list(self) -> &'a mut W {
        self.variant(SRC_MULTBLK_TYPE_A::LINKED_LIST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u64) & 0x03) << 2);
        self.w
    }
}
#[doc = "Transfer type and flow control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TT_FC_A {
    #[doc = "0: Transfer memory to memory and flow controller is DMAC"]
    MEM2MEM_DMA,
    #[doc = "1: Transfer memory to peripheral and flow controller is DMAC"]
    MEM2PRF_DMA,
    #[doc = "2: Transfer peripheral to memory and flow controller is DMAC"]
    PRF2MEM_DMA,
    #[doc = "3: Transfer peripheral to peripheral and flow controller is DMAC"]
    PRF2PRF_DMA,
    #[doc = "4: Transfer peripheral to memory and flow controller is source peripheral"]
    PRF2MEM_PRF,
    #[doc = "5: Transfer peripheral to peripheral and flow controller is source peripheral"]
    PRF2PRF_SRCPRF,
    #[doc = "6: Transfer memory to peripheral and flow controller is destination peripheral"]
    MEM2PRF_PRF,
    #[doc = "7: Transfer peripheral to peripheral and flow controller is destination peripheral"]
    PRF2PRF_DSTPRF,
}
impl From<TT_FC_A> for u8 {
    #[inline(always)]
    fn from(variant: TT_FC_A) -> Self {
        match variant {
            TT_FC_A::MEM2MEM_DMA => 0,
            TT_FC_A::MEM2PRF_DMA => 1,
            TT_FC_A::PRF2MEM_DMA => 2,
            TT_FC_A::PRF2PRF_DMA => 3,
            TT_FC_A::PRF2MEM_PRF => 4,
            TT_FC_A::PRF2PRF_SRCPRF => 5,
            TT_FC_A::MEM2PRF_PRF => 6,
            TT_FC_A::PRF2PRF_DSTPRF => 7,
        }
    }
}
#[doc = "Reader of field `tt_fc`"]
pub type TT_FC_R = crate::R<u8, TT_FC_A>;
impl TT_FC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TT_FC_A {
        match self.bits {
            0 => TT_FC_A::MEM2MEM_DMA,
            1 => TT_FC_A::MEM2PRF_DMA,
            2 => TT_FC_A::PRF2MEM_DMA,
            3 => TT_FC_A::PRF2PRF_DMA,
            4 => TT_FC_A::PRF2MEM_PRF,
            5 => TT_FC_A::PRF2PRF_SRCPRF,
            6 => TT_FC_A::MEM2PRF_PRF,
            7 => TT_FC_A::PRF2PRF_DSTPRF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEM2MEM_DMA`"]
    #[inline(always)]
    pub fn is_mem2mem_dma(&self) -> bool {
        *self == TT_FC_A::MEM2MEM_DMA
    }
    #[doc = "Checks if the value of the field is `MEM2PRF_DMA`"]
    #[inline(always)]
    pub fn is_mem2prf_dma(&self) -> bool {
        *self == TT_FC_A::MEM2PRF_DMA
    }
    #[doc = "Checks if the value of the field is `PRF2MEM_DMA`"]
    #[inline(always)]
    pub fn is_prf2mem_dma(&self) -> bool {
        *self == TT_FC_A::PRF2MEM_DMA
    }
    #[doc = "Checks if the value of the field is `PRF2PRF_DMA`"]
    #[inline(always)]
    pub fn is_prf2prf_dma(&self) -> bool {
        *self == TT_FC_A::PRF2PRF_DMA
    }
    #[doc = "Checks if the value of the field is `PRF2MEM_PRF`"]
    #[inline(always)]
    pub fn is_prf2mem_prf(&self) -> bool {
        *self == TT_FC_A::PRF2MEM_PRF
    }
    #[doc = "Checks if the value of the field is `PRF2PRF_SRCPRF`"]
    #[inline(always)]
    pub fn is_prf2prf_srcprf(&self) -> bool {
        *self == TT_FC_A::PRF2PRF_SRCPRF
    }
    #[doc = "Checks if the value of the field is `MEM2PRF_PRF`"]
    #[inline(always)]
    pub fn is_mem2prf_prf(&self) -> bool {
        *self == TT_FC_A::MEM2PRF_PRF
    }
    #[doc = "Checks if the value of the field is `PRF2PRF_DSTPRF`"]
    #[inline(always)]
    pub fn is_prf2prf_dstprf(&self) -> bool {
        *self == TT_FC_A::PRF2PRF_DSTPRF
    }
}
#[doc = "Write proxy for field `tt_fc`"]
pub struct TT_FC_W<'a> {
    w: &'a mut W,
}
impl<'a> TT_FC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TT_FC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Transfer memory to memory and flow controller is DMAC"]
    #[inline(always)]
    pub fn mem2mem_dma(self) -> &'a mut W {
        self.variant(TT_FC_A::MEM2MEM_DMA)
    }
    #[doc = "Transfer memory to peripheral and flow controller is DMAC"]
    #[inline(always)]
    pub fn mem2prf_dma(self) -> &'a mut W {
        self.variant(TT_FC_A::MEM2PRF_DMA)
    }
    #[doc = "Transfer peripheral to memory and flow controller is DMAC"]
    #[inline(always)]
    pub fn prf2mem_dma(self) -> &'a mut W {
        self.variant(TT_FC_A::PRF2MEM_DMA)
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is DMAC"]
    #[inline(always)]
    pub fn prf2prf_dma(self) -> &'a mut W {
        self.variant(TT_FC_A::PRF2PRF_DMA)
    }
    #[doc = "Transfer peripheral to memory and flow controller is source peripheral"]
    #[inline(always)]
    pub fn prf2mem_prf(self) -> &'a mut W {
        self.variant(TT_FC_A::PRF2MEM_PRF)
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is source peripheral"]
    #[inline(always)]
    pub fn prf2prf_srcprf(self) -> &'a mut W {
        self.variant(TT_FC_A::PRF2PRF_SRCPRF)
    }
    #[doc = "Transfer memory to peripheral and flow controller is destination peripheral"]
    #[inline(always)]
    pub fn mem2prf_prf(self) -> &'a mut W {
        self.variant(TT_FC_A::MEM2PRF_PRF)
    }
    #[doc = "Transfer peripheral to peripheral and flow controller is destination peripheral"]
    #[inline(always)]
    pub fn prf2prf_dstprf(self) -> &'a mut W {
        self.variant(TT_FC_A::PRF2PRF_DSTPRF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 32)) | (((value as u64) & 0x07) << 32);
        self.w
    }
}
#[doc = "Source software or hardware handshaking select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_SEL_SRC_A {
    #[doc = "0: Hardware handshaking is used"]
    HARDWARE,
    #[doc = "1: Software handshaking is used"]
    SOFTWARE,
}
impl From<HS_SEL_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: HS_SEL_SRC_A) -> Self {
        match variant {
            HS_SEL_SRC_A::HARDWARE => false,
            HS_SEL_SRC_A::SOFTWARE => true,
        }
    }
}
#[doc = "Reader of field `hs_sel_src`"]
pub type HS_SEL_SRC_R = crate::R<bool, HS_SEL_SRC_A>;
impl HS_SEL_SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_SEL_SRC_A {
        match self.bits {
            false => HS_SEL_SRC_A::HARDWARE,
            true => HS_SEL_SRC_A::SOFTWARE,
        }
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == HS_SEL_SRC_A::HARDWARE
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == HS_SEL_SRC_A::SOFTWARE
    }
}
#[doc = "Write proxy for field `hs_sel_src`"]
pub struct HS_SEL_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_SEL_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_SEL_SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware handshaking is used"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(HS_SEL_SRC_A::HARDWARE)
    }
    #[doc = "Software handshaking is used"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(HS_SEL_SRC_A::SOFTWARE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 35)) | (((value as u64) & 0x01) << 35);
        self.w
    }
}
#[doc = "Destination software or hardware handshaking select"]
pub type HS_SEL_DST_A = HS_SEL_SRC_A;
#[doc = "Reader of field `hs_sel_dst`"]
pub type HS_SEL_DST_R = crate::R<bool, HS_SEL_SRC_A>;
#[doc = "Write proxy for field `hs_sel_dst`"]
pub struct HS_SEL_DST_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_SEL_DST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_SEL_DST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware handshaking is used"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(HS_SEL_SRC_A::HARDWARE)
    }
    #[doc = "Software handshaking is used"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(HS_SEL_SRC_A::SOFTWARE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 36)) | (((value as u64) & 0x01) << 36);
        self.w
    }
}
#[doc = "Source hardware handshaking interface polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_HWHS_POL_A {
    #[doc = "0: Active high"]
    ACTIVE_HIGH,
    #[doc = "1: Active low"]
    ACTIVE_LOW,
}
impl From<SRC_HWHS_POL_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_HWHS_POL_A) -> Self {
        match variant {
            SRC_HWHS_POL_A::ACTIVE_HIGH => false,
            SRC_HWHS_POL_A::ACTIVE_LOW => true,
        }
    }
}
#[doc = "Reader of field `src_hwhs_pol`"]
pub type SRC_HWHS_POL_R = crate::R<bool, SRC_HWHS_POL_A>;
impl SRC_HWHS_POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_HWHS_POL_A {
        match self.bits {
            false => SRC_HWHS_POL_A::ACTIVE_HIGH,
            true => SRC_HWHS_POL_A::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == SRC_HWHS_POL_A::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == SRC_HWHS_POL_A::ACTIVE_LOW
    }
}
#[doc = "Write proxy for field `src_hwhs_pol`"]
pub struct SRC_HWHS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_HWHS_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_HWHS_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(SRC_HWHS_POL_A::ACTIVE_HIGH)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(SRC_HWHS_POL_A::ACTIVE_LOW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 37)) | (((value as u64) & 0x01) << 37);
        self.w
    }
}
#[doc = "Destination hardware handshaking interface polarity"]
pub type DST_HWHS_POL_A = SRC_HWHS_POL_A;
#[doc = "Reader of field `dst_hwhs_pol`"]
pub type DST_HWHS_POL_R = crate::R<bool, SRC_HWHS_POL_A>;
#[doc = "Write proxy for field `dst_hwhs_pol`"]
pub struct DST_HWHS_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_HWHS_POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_HWHS_POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(SRC_HWHS_POL_A::ACTIVE_HIGH)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(SRC_HWHS_POL_A::ACTIVE_LOW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 38)) | (((value as u64) & 0x01) << 38);
        self.w
    }
}
#[doc = "Reader of field `src_per`"]
pub type SRC_PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `src_per`"]
pub struct SRC_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 39)) | (((value as u64) & 0x0f) << 39);
        self.w
    }
}
#[doc = "Reader of field `dst_per`"]
pub type DST_PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dst_per`"]
pub struct DST_PER_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 44)) | (((value as u64) & 0x0f) << 44);
        self.w
    }
}
#[doc = "Reader of field `ch_prior`"]
pub type CH_PRIOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ch_prior`"]
pub struct CH_PRIOR_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_PRIOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 49)) | (((value as u64) & 0x07) << 49);
        self.w
    }
}
#[doc = "Reader of field `lock_ch`"]
pub type LOCK_CH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `lock_ch`"]
pub struct LOCK_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_CH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 52)) | (((value as u64) & 0x01) << 52);
        self.w
    }
}
#[doc = "Channel lock level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_CH_L_A {
    #[doc = "0: Duration of channel is locked for entire DMA transfer"]
    DMA_TRANSFER,
    #[doc = "1: Duration of channel is locked for current block transfer"]
    BLOCK_TRANSFER,
    #[doc = "2: Duration of channel is locked for current transaction"]
    TRANSACTION,
}
impl From<LOCK_CH_L_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_CH_L_A) -> Self {
        match variant {
            LOCK_CH_L_A::DMA_TRANSFER => 0,
            LOCK_CH_L_A::BLOCK_TRANSFER => 1,
            LOCK_CH_L_A::TRANSACTION => 2,
        }
    }
}
#[doc = "Reader of field `lock_ch_l`"]
pub type LOCK_CH_L_R = crate::R<u8, LOCK_CH_L_A>;
impl LOCK_CH_L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LOCK_CH_L_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOCK_CH_L_A::DMA_TRANSFER),
            1 => Val(LOCK_CH_L_A::BLOCK_TRANSFER),
            2 => Val(LOCK_CH_L_A::TRANSACTION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DMA_TRANSFER`"]
    #[inline(always)]
    pub fn is_dma_transfer(&self) -> bool {
        *self == LOCK_CH_L_A::DMA_TRANSFER
    }
    #[doc = "Checks if the value of the field is `BLOCK_TRANSFER`"]
    #[inline(always)]
    pub fn is_block_transfer(&self) -> bool {
        *self == LOCK_CH_L_A::BLOCK_TRANSFER
    }
    #[doc = "Checks if the value of the field is `TRANSACTION`"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        *self == LOCK_CH_L_A::TRANSACTION
    }
}
#[doc = "Write proxy for field `lock_ch_l`"]
pub struct LOCK_CH_L_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_CH_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_CH_L_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Duration of channel is locked for entire DMA transfer"]
    #[inline(always)]
    pub fn dma_transfer(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::DMA_TRANSFER)
    }
    #[doc = "Duration of channel is locked for current block transfer"]
    #[inline(always)]
    pub fn block_transfer(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::BLOCK_TRANSFER)
    }
    #[doc = "Duration of channel is locked for current transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut W {
        self.variant(LOCK_CH_L_A::TRANSACTION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 53)) | (((value as u64) & 0x03) << 53);
        self.w
    }
}
#[doc = "Reader of field `src_osr_lmt`"]
pub type SRC_OSR_LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `src_osr_lmt`"]
pub struct SRC_OSR_LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_OSR_LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 55)) | (((value as u64) & 0x0f) << 55);
        self.w
    }
}
#[doc = "Reader of field `dst_osr_lmt`"]
pub type DST_OSR_LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `dst_osr_lmt`"]
pub struct DST_OSR_LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_OSR_LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 59)) | (((value as u64) & 0x0f) << 59);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Source multi-block transfer type"]
    #[inline(always)]
    pub fn src_multblk_type(&self) -> SRC_MULTBLK_TYPE_R {
        SRC_MULTBLK_TYPE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Destination multi-block transfer type"]
    #[inline(always)]
    pub fn dst_multblk_type(&self) -> DST_MULTBLK_TYPE_R {
        DST_MULTBLK_TYPE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 32:34 - Transfer type and flow control"]
    #[inline(always)]
    pub fn tt_fc(&self) -> TT_FC_R {
        TT_FC_R::new(((self.bits >> 32) & 0x07) as u8)
    }
    #[doc = "Bit 35 - Source software or hardware handshaking select"]
    #[inline(always)]
    pub fn hs_sel_src(&self) -> HS_SEL_SRC_R {
        HS_SEL_SRC_R::new(((self.bits >> 35) & 0x01) != 0)
    }
    #[doc = "Bit 36 - Destination software or hardware handshaking select"]
    #[inline(always)]
    pub fn hs_sel_dst(&self) -> HS_SEL_DST_R {
        HS_SEL_DST_R::new(((self.bits >> 36) & 0x01) != 0)
    }
    #[doc = "Bit 37 - Source hardware handshaking interface polarity"]
    #[inline(always)]
    pub fn src_hwhs_pol(&self) -> SRC_HWHS_POL_R {
        SRC_HWHS_POL_R::new(((self.bits >> 37) & 0x01) != 0)
    }
    #[doc = "Bit 38 - Destination hardware handshaking interface polarity"]
    #[inline(always)]
    pub fn dst_hwhs_pol(&self) -> DST_HWHS_POL_R {
        DST_HWHS_POL_R::new(((self.bits >> 38) & 0x01) != 0)
    }
    #[doc = "Bits 39:42 - Assign a hardware handshaking interface to source of channel"]
    #[inline(always)]
    pub fn src_per(&self) -> SRC_PER_R {
        SRC_PER_R::new(((self.bits >> 39) & 0x0f) as u8)
    }
    #[doc = "Bits 44:47 - Assign a hardware handshaking interface to destination of channel"]
    #[inline(always)]
    pub fn dst_per(&self) -> DST_PER_R {
        DST_PER_R::new(((self.bits >> 44) & 0x0f) as u8)
    }
    #[doc = "Bits 49:51 - Channel priority (7 is highest, 0 is lowest)"]
    #[inline(always)]
    pub fn ch_prior(&self) -> CH_PRIOR_R {
        CH_PRIOR_R::new(((self.bits >> 49) & 0x07) as u8)
    }
    #[doc = "Bit 52 - Channel lock bit"]
    #[inline(always)]
    pub fn lock_ch(&self) -> LOCK_CH_R {
        LOCK_CH_R::new(((self.bits >> 52) & 0x01) != 0)
    }
    #[doc = "Bits 53:54 - Channel lock level"]
    #[inline(always)]
    pub fn lock_ch_l(&self) -> LOCK_CH_L_R {
        LOCK_CH_L_R::new(((self.bits >> 53) & 0x03) as u8)
    }
    #[doc = "Bits 55:58 - Source outstanding request limit"]
    #[inline(always)]
    pub fn src_osr_lmt(&self) -> SRC_OSR_LMT_R {
        SRC_OSR_LMT_R::new(((self.bits >> 55) & 0x0f) as u8)
    }
    #[doc = "Bits 59:62 - Destination outstanding request limit"]
    #[inline(always)]
    pub fn dst_osr_lmt(&self) -> DST_OSR_LMT_R {
        DST_OSR_LMT_R::new(((self.bits >> 59) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Source multi-block transfer type"]
    #[inline(always)]
    pub fn src_multblk_type(&mut self) -> SRC_MULTBLK_TYPE_W {
        SRC_MULTBLK_TYPE_W { w: self }
    }
    #[doc = "Bits 2:3 - Destination multi-block transfer type"]
    #[inline(always)]
    pub fn dst_multblk_type(&mut self) -> DST_MULTBLK_TYPE_W {
        DST_MULTBLK_TYPE_W { w: self }
    }
    #[doc = "Bits 32:34 - Transfer type and flow control"]
    #[inline(always)]
    pub fn tt_fc(&mut self) -> TT_FC_W {
        TT_FC_W { w: self }
    }
    #[doc = "Bit 35 - Source software or hardware handshaking select"]
    #[inline(always)]
    pub fn hs_sel_src(&mut self) -> HS_SEL_SRC_W {
        HS_SEL_SRC_W { w: self }
    }
    #[doc = "Bit 36 - Destination software or hardware handshaking select"]
    #[inline(always)]
    pub fn hs_sel_dst(&mut self) -> HS_SEL_DST_W {
        HS_SEL_DST_W { w: self }
    }
    #[doc = "Bit 37 - Source hardware handshaking interface polarity"]
    #[inline(always)]
    pub fn src_hwhs_pol(&mut self) -> SRC_HWHS_POL_W {
        SRC_HWHS_POL_W { w: self }
    }
    #[doc = "Bit 38 - Destination hardware handshaking interface polarity"]
    #[inline(always)]
    pub fn dst_hwhs_pol(&mut self) -> DST_HWHS_POL_W {
        DST_HWHS_POL_W { w: self }
    }
    #[doc = "Bits 39:42 - Assign a hardware handshaking interface to source of channel"]
    #[inline(always)]
    pub fn src_per(&mut self) -> SRC_PER_W {
        SRC_PER_W { w: self }
    }
    #[doc = "Bits 44:47 - Assign a hardware handshaking interface to destination of channel"]
    #[inline(always)]
    pub fn dst_per(&mut self) -> DST_PER_W {
        DST_PER_W { w: self }
    }
    #[doc = "Bits 49:51 - Channel priority (7 is highest, 0 is lowest)"]
    #[inline(always)]
    pub fn ch_prior(&mut self) -> CH_PRIOR_W {
        CH_PRIOR_W { w: self }
    }
    #[doc = "Bit 52 - Channel lock bit"]
    #[inline(always)]
    pub fn lock_ch(&mut self) -> LOCK_CH_W {
        LOCK_CH_W { w: self }
    }
    #[doc = "Bits 53:54 - Channel lock level"]
    #[inline(always)]
    pub fn lock_ch_l(&mut self) -> LOCK_CH_L_W {
        LOCK_CH_L_W { w: self }
    }
    #[doc = "Bits 55:58 - Source outstanding request limit"]
    #[inline(always)]
    pub fn src_osr_lmt(&mut self) -> SRC_OSR_LMT_W {
        SRC_OSR_LMT_W { w: self }
    }
    #[doc = "Bits 59:62 - Destination outstanding request limit"]
    #[inline(always)]
    pub fn dst_osr_lmt(&mut self) -> DST_OSR_LMT_W {
        DST_OSR_LMT_W { w: self }
    }
}
