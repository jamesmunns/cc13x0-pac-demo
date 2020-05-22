#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Reader of field `WUMIS`"]
pub type WUMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMABMIS`"]
pub type DMABMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TBMMIS`"]
pub type TBMMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBEMIS`"]
pub type CBEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBMMIS`"]
pub type CBMMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TBTOMIS`"]
pub type TBTOMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAAMIS`"]
pub type DMAAMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TAMMIS`"]
pub type TAMMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTCMIS`"]
pub type RTCMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAEMIS`"]
pub type CAEMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAMMIS`"]
pub type CAMMIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TATOMIS`"]
pub type TATOMIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16 - WUMIS"]
    #[inline(always)]
    pub fn wumis(&self) -> WUMIS_R {
        WUMIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DMABMIS"]
    #[inline(always)]
    pub fn dmabmis(&self) -> DMABMIS_R {
        DMABMIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TBMMIS"]
    #[inline(always)]
    pub fn tbmmis(&self) -> TBMMIS_R {
        TBMMIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CBEMIS"]
    #[inline(always)]
    pub fn cbemis(&self) -> CBEMIS_R {
        CBEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CBMMIS"]
    #[inline(always)]
    pub fn cbmmis(&self) -> CBMMIS_R {
        CBMMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TBTOMIS"]
    #[inline(always)]
    pub fn tbtomis(&self) -> TBTOMIS_R {
        TBTOMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMAAMIS"]
    #[inline(always)]
    pub fn dmaamis(&self) -> DMAAMIS_R {
        DMAAMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TAMMIS"]
    #[inline(always)]
    pub fn tammis(&self) -> TAMMIS_R {
        TAMMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTCMIS"]
    #[inline(always)]
    pub fn rtcmis(&self) -> RTCMIS_R {
        RTCMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CAEMIS"]
    #[inline(always)]
    pub fn caemis(&self) -> CAEMIS_R {
        CAEMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CAMMIS"]
    #[inline(always)]
    pub fn cammis(&self) -> CAMMIS_R {
        CAMMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TATOMIS"]
    #[inline(always)]
    pub fn tatomis(&self) -> TATOMIS_R {
        TATOMIS_R::new((self.bits & 0x01) != 0)
    }
}
