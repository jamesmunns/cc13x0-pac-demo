#[doc = "Reader of register AMPCOMP_TH2"]
pub type R = crate::R<u32, super::AMPCOMP_TH2>;
#[doc = "Reader of field `LPMUPDATE_LTH`"]
pub type LPMUPDATE_LTH_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPMUPDATE_HTM`"]
pub type LPMUPDATE_HTM_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADC_COMP_AMPTH_LPM`"]
pub type ADC_COMP_AMPTH_LPM_R = crate::R<u8, u8>;
#[doc = "Reader of field `ADC_COMP_AMPTH_HPM`"]
pub type ADC_COMP_AMPTH_HPM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 26:31 - LPMUPDATE_LTH"]
    #[inline(always)]
    pub fn lpmupdate_lth(&self) -> LPMUPDATE_LTH_R {
        LPMUPDATE_LTH_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - LPMUPDATE_HTM"]
    #[inline(always)]
    pub fn lpmupdate_htm(&self) -> LPMUPDATE_HTM_R {
        LPMUPDATE_HTM_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - ADC_COMP_AMPTH_LPM"]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&self) -> ADC_COMP_AMPTH_LPM_R {
        ADC_COMP_AMPTH_LPM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 2:7 - ADC_COMP_AMPTH_HPM"]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&self) -> ADC_COMP_AMPTH_HPM_R {
        ADC_COMP_AMPTH_HPM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
