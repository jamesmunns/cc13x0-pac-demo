#[doc = "Reader of register SHDW_ANA_TRIM"]
pub type R = crate::R<u32, super::SHDW_ANA_TRIM>;
#[doc = "Reader of field `BOD_BANDGAP_TRIM_CNF`"]
pub type BOD_BANDGAP_TRIM_CNF_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_ENABLE_PG1`"]
pub type VDDR_ENABLE_PG1_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDDR_OK_HYS`"]
pub type VDDR_OK_HYS_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPTAT_TRIM`"]
pub type IPTAT_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_TRIM`"]
pub type VDDR_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIMBOD_INTMODE`"]
pub type TRIMBOD_INTMODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIMBOD_EXTMODE`"]
pub type TRIMBOD_EXTMODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIMTEMP`"]
pub type TRIMTEMP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 25:26 - BOD_BANDGAP_TRIM_CNF"]
    #[inline(always)]
    pub fn bod_bandgap_trim_cnf(&self) -> BOD_BANDGAP_TRIM_CNF_R {
        BOD_BANDGAP_TRIM_CNF_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 24 - VDDR_ENABLE_PG1"]
    #[inline(always)]
    pub fn vddr_enable_pg1(&self) -> VDDR_ENABLE_PG1_R {
        VDDR_ENABLE_PG1_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - VDDR_OK_HYS"]
    #[inline(always)]
    pub fn vddr_ok_hys(&self) -> VDDR_OK_HYS_R {
        VDDR_OK_HYS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - IPTAT_TRIM"]
    #[inline(always)]
    pub fn iptat_trim(&self) -> IPTAT_TRIM_R {
        IPTAT_TRIM_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - VDDR_TRIM"]
    #[inline(always)]
    pub fn vddr_trim(&self) -> VDDR_TRIM_R {
        VDDR_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - TRIMBOD_INTMODE"]
    #[inline(always)]
    pub fn trimbod_intmode(&self) -> TRIMBOD_INTMODE_R {
        TRIMBOD_INTMODE_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - TRIMBOD_EXTMODE"]
    #[inline(always)]
    pub fn trimbod_extmode(&self) -> TRIMBOD_EXTMODE_R {
        TRIMBOD_EXTMODE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:5 - TRIMTEMP"]
    #[inline(always)]
    pub fn trimtemp(&self) -> TRIMTEMP_R {
        TRIMTEMP_R::new((self.bits & 0x3f) as u8)
    }
}
