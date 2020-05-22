#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `WURIS`"]
pub type WURIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMABRIS`"]
pub type DMABRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TBMRIS`"]
pub type TBMRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBERIS`"]
pub type CBERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBMRIS`"]
pub type CBMRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TBTORIS`"]
pub type TBTORIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAARIS`"]
pub type DMAARIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMRIS`"]
pub type TAMRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTCRIS`"]
pub type RTCRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAERIS`"]
pub type CAERIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAMRIS`"]
pub type CAMRIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TATORIS`"]
pub type TATORIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16 - WURIS"]
    #[inline(always)]
    pub fn wuris(&self) -> WURIS_R {
        WURIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DMABRIS"]
    #[inline(always)]
    pub fn dmabris(&self) -> DMABRIS_R {
        DMABRIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TBMRIS"]
    #[inline(always)]
    pub fn tbmris(&self) -> TBMRIS_R {
        TBMRIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CBERIS"]
    #[inline(always)]
    pub fn cberis(&self) -> CBERIS_R {
        CBERIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CBMRIS"]
    #[inline(always)]
    pub fn cbmris(&self) -> CBMRIS_R {
        CBMRIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TBTORIS"]
    #[inline(always)]
    pub fn tbtoris(&self) -> TBTORIS_R {
        TBTORIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMAARIS"]
    #[inline(always)]
    pub fn dmaaris(&self) -> DMAARIS_R {
        DMAARIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TAMRIS"]
    #[inline(always)]
    pub fn tamris(&self) -> TAMRIS_R {
        TAMRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTCRIS"]
    #[inline(always)]
    pub fn rtcris(&self) -> RTCRIS_R {
        RTCRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAERIS"]
    #[inline(always)]
    pub fn caeris(&self) -> CAERIS_R {
        CAERIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CAMRIS"]
    #[inline(always)]
    pub fn camris(&self) -> CAMRIS_R {
        CAMRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TATORIS"]
    #[inline(always)]
    pub fn tatoris(&self) -> TATORIS_R {
        TATORIS_R::new((self.bits & 0x01) != 0)
    }
}
