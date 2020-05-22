#[doc = "Reader of register AMPCOMP_TH1"]
pub type R = crate::R<u32, super::AMPCOMP_TH1>;
#[doc = "Reader of field `HPMRAMP3_LTH`"]
pub type HPMRAMP3_LTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `HPMRAMP3_HTH`"]
pub type HPMRAMP3_HTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `IBIASCAP_LPTOHP_OL_CNT`"]
pub type IBIASCAP_LPTOHP_OL_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `HPMRAMP1_TH`"]
pub type HPMRAMP1_TH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 18:23 - HPMRAMP3_LTH"]
    #[inline(always)]
    pub fn hpmramp3_lth(&self) -> HPMRAMP3_LTH_R {
        HPMRAMP3_LTH_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - HPMRAMP3_HTH"]
    #[inline(always)]
    pub fn hpmramp3_hth(&self) -> HPMRAMP3_HTH_R {
        HPMRAMP3_HTH_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - IBIASCAP_LPTOHP_OL_CNT"]
    #[inline(always)]
    pub fn ibiascap_lptohp_ol_cnt(&self) -> IBIASCAP_LPTOHP_OL_CNT_R {
        IBIASCAP_LPTOHP_OL_CNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 0:5 - HPMRAMP1_TH"]
    #[inline(always)]
    pub fn hpmramp1_th(&self) -> HPMRAMP1_TH_R {
        HPMRAMP1_TH_R::new((self.bits & 0x3f) as u8)
    }
}
