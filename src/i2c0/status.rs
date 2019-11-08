#[doc = "Reader of register status"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `activity`"]
pub type ACTIVITY_R = crate::R<bool, bool>;
#[doc = "Reader of field `tfnf`"]
pub type TFNF_R = crate::R<bool, bool>;
#[doc = "Reader of field `tfe`"]
pub type TFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `rfne`"]
pub type RFNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `rff`"]
pub type RFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `mst_activity`"]
pub type MST_ACTIVITY_R = crate::R<bool, bool>;
#[doc = "Reader of field `slv_activity`"]
pub type SLV_ACTIVITY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ACTIVITY"]
    #[inline(always)]
    pub fn activity(&self) -> ACTIVITY_R {
        ACTIVITY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TFNF"]
    #[inline(always)]
    pub fn tfnf(&self) -> TFNF_R {
        TFNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TFE"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RFNE"]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RFF"]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MST_ACTIVITY"]
    #[inline(always)]
    pub fn mst_activity(&self) -> MST_ACTIVITY_R {
        MST_ACTIVITY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SLV_ACTIVITY"]
    #[inline(always)]
    pub fn slv_activity(&self) -> SLV_ACTIVITY_R {
        SLV_ACTIVITY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
