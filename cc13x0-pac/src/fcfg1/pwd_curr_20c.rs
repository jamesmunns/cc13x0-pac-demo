#[doc = "Reader of register PWD_CURR_20C"]
pub type R = crate::R<u32, super::PWD_CURR_20C>;
#[doc = "Reader of field `DELTA_CACHE_REF`"]
pub type DELTA_CACHE_REF_R = crate::R<u8, u8>;
#[doc = "Reader of field `DELTA_RFMEM_RET`"]
pub type DELTA_RFMEM_RET_R = crate::R<u8, u8>;
#[doc = "Reader of field `DELTA_XOSC_LPM`"]
pub type DELTA_XOSC_LPM_R = crate::R<u8, u8>;
#[doc = "Reader of field `BASELINE`"]
pub type BASELINE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - DELTA_CACHE_REF"]
    #[inline(always)]
    pub fn delta_cache_ref(&self) -> DELTA_CACHE_REF_R {
        DELTA_CACHE_REF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DELTA_RFMEM_RET"]
    #[inline(always)]
    pub fn delta_rfmem_ret(&self) -> DELTA_RFMEM_RET_R {
        DELTA_RFMEM_RET_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DELTA_XOSC_LPM"]
    #[inline(always)]
    pub fn delta_xosc_lpm(&self) -> DELTA_XOSC_LPM_R {
        DELTA_XOSC_LPM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BASELINE"]
    #[inline(always)]
    pub fn baseline(&self) -> BASELINE_R {
        BASELINE_R::new((self.bits & 0xff) as u8)
    }
}
