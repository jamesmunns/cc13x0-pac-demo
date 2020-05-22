#[doc = "Reader of register FMSTAT"]
pub type R = crate::R<u32, super::FMSTAT>;
#[doc = "Reader of field `RVSUSP`"]
pub type RVSUSP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDVER`"]
pub type RDVER_R = crate::R<bool, bool>;
#[doc = "Reader of field `RVF`"]
pub type RVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ILA`"]
pub type ILA_R = crate::R<bool, bool>;
#[doc = "Reader of field `DBF`"]
pub type DBF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PGV`"]
pub type PGV_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCV`"]
pub type PCV_R = crate::R<bool, bool>;
#[doc = "Reader of field `EV`"]
pub type EV_R = crate::R<bool, bool>;
#[doc = "Reader of field `CV`"]
pub type CV_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERS`"]
pub type ERS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PGM`"]
pub type PGM_R = crate::R<bool, bool>;
#[doc = "Reader of field `INVDAT`"]
pub type INVDAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSTAT`"]
pub type CSTAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `VOLSTAT`"]
pub type VOLSTAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ESUSP`"]
pub type ESUSP_R = crate::R<bool, bool>;
#[doc = "Reader of field `PSUSP`"]
pub type PSUSP_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLOCK`"]
pub type SLOCK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 17 - RVSUSP"]
    #[inline(always)]
    pub fn rvsusp(&self) -> RVSUSP_R {
        RVSUSP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RDVER"]
    #[inline(always)]
    pub fn rdver(&self) -> RDVER_R {
        RDVER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - RVF"]
    #[inline(always)]
    pub fn rvf(&self) -> RVF_R {
        RVF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ILA"]
    #[inline(always)]
    pub fn ila(&self) -> ILA_R {
        ILA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DBF"]
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PGV"]
    #[inline(always)]
    pub fn pgv(&self) -> PGV_R {
        PGV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PCV"]
    #[inline(always)]
    pub fn pcv(&self) -> PCV_R {
        PCV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EV"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CV"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ERS"]
    #[inline(always)]
    pub fn ers(&self) -> ERS_R {
        ERS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PGM"]
    #[inline(always)]
    pub fn pgm(&self) -> PGM_R {
        PGM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - INVDAT"]
    #[inline(always)]
    pub fn invdat(&self) -> INVDAT_R {
        INVDAT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSTAT"]
    #[inline(always)]
    pub fn cstat(&self) -> CSTAT_R {
        CSTAT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VOLSTAT"]
    #[inline(always)]
    pub fn volstat(&self) -> VOLSTAT_R {
        VOLSTAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ESUSP"]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - PSUSP"]
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SLOCK"]
    #[inline(always)]
    pub fn slock(&self) -> SLOCK_R {
        SLOCK_R::new((self.bits & 0x01) != 0)
    }
}
