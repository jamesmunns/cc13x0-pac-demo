#[doc = "Reader of register FCFG_WRAPPER"]
pub type R = crate::R<u32, super::FCFG_WRAPPER>;
#[doc = "Reader of field `FAMILY_TYPE`"]
pub type FAMILY_TYPE_R = crate::R<u8, u8>;
#[doc = "Reader of field `MEM_MAP`"]
pub type MEM_MAP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CPU2`"]
pub type CPU2_R = crate::R<u8, u8>;
#[doc = "Reader of field `EE_IN_MAIN`"]
pub type EE_IN_MAIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `ROM`"]
pub type ROM_R = crate::R<bool, bool>;
#[doc = "Reader of field `IFLUSH`"]
pub type IFLUSH_R = crate::R<bool, bool>;
#[doc = "Reader of field `SIL3`"]
pub type SIL3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCA`"]
pub type ECCA_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUTO_SUSP`"]
pub type AUTO_SUSP_R = crate::R<u8, u8>;
#[doc = "Reader of field `UERR`"]
pub type UERR_R = crate::R<u8, u8>;
#[doc = "Reader of field `CPU_TYPE1`"]
pub type CPU_TYPE1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - FAMILY_TYPE"]
    #[inline(always)]
    pub fn family_type(&self) -> FAMILY_TYPE_R {
        FAMILY_TYPE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 20 - MEM_MAP"]
    #[inline(always)]
    pub fn mem_map(&self) -> MEM_MAP_R {
        MEM_MAP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - CPU2"]
    #[inline(always)]
    pub fn cpu2(&self) -> CPU2_R {
        CPU2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EE_IN_MAIN"]
    #[inline(always)]
    pub fn ee_in_main(&self) -> EE_IN_MAIN_R {
        EE_IN_MAIN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - ROM"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - IFLUSH"]
    #[inline(always)]
    pub fn iflush(&self) -> IFLUSH_R {
        IFLUSH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SIL3"]
    #[inline(always)]
    pub fn sil3(&self) -> SIL3_R {
        SIL3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ECCA"]
    #[inline(always)]
    pub fn ecca(&self) -> ECCA_R {
        ECCA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - AUTO_SUSP"]
    #[inline(always)]
    pub fn auto_susp(&self) -> AUTO_SUSP_R {
        AUTO_SUSP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - UERR"]
    #[inline(always)]
    pub fn uerr(&self) -> UERR_R {
        UERR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - CPU_TYPE1"]
    #[inline(always)]
    pub fn cpu_type1(&self) -> CPU_TYPE1_R {
        CPU_TYPE1_R::new((self.bits & 0x0f) as u8)
    }
}
