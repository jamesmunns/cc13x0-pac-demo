#[doc = "Reader of register STAT2"]
pub type R = crate::R<u32, super::STAT2>;
#[doc = "Reader of field `ADC_DCBIAS`"]
pub type ADC_DCBIAS_R = crate::R<u8, u8>;
#[doc = "Reader of field `HPM_RAMP1_THMET`"]
pub type HPM_RAMP1_THMET_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPM_RAMP2_THMET`"]
pub type HPM_RAMP2_THMET_R = crate::R<bool, bool>;
#[doc = "Reader of field `HPM_RAMP3_THMET`"]
pub type HPM_RAMP3_THMET_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAMPSTATE`"]
pub type RAMPSTATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `AMPCOMP_REQ`"]
pub type AMPCOMP_REQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC_HF_AMPGOOD`"]
pub type XOSC_HF_AMPGOOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC_HF_FREQGOOD`"]
pub type XOSC_HF_FREQGOOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC_HF_RF_FREQGOOD`"]
pub type XOSC_HF_RF_FREQGOOD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 26:31 - ADC_DCBIAS"]
    #[inline(always)]
    pub fn adc_dcbias(&self) -> ADC_DCBIAS_R {
        ADC_DCBIAS_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bit 25 - HPM_RAMP1_THMET"]
    #[inline(always)]
    pub fn hpm_ramp1_thmet(&self) -> HPM_RAMP1_THMET_R {
        HPM_RAMP1_THMET_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - HPM_RAMP2_THMET"]
    #[inline(always)]
    pub fn hpm_ramp2_thmet(&self) -> HPM_RAMP2_THMET_R {
        HPM_RAMP2_THMET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - HPM_RAMP3_THMET"]
    #[inline(always)]
    pub fn hpm_ramp3_thmet(&self) -> HPM_RAMP3_THMET_R {
        HPM_RAMP3_THMET_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - RAMPSTATE"]
    #[inline(always)]
    pub fn rampstate(&self) -> RAMPSTATE_R {
        RAMPSTATE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - AMPCOMP_REQ"]
    #[inline(always)]
    pub fn ampcomp_req(&self) -> AMPCOMP_REQ_R {
        AMPCOMP_REQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XOSC_HF_AMPGOOD"]
    #[inline(always)]
    pub fn xosc_hf_ampgood(&self) -> XOSC_HF_AMPGOOD_R {
        XOSC_HF_AMPGOOD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - XOSC_HF_FREQGOOD"]
    #[inline(always)]
    pub fn xosc_hf_freqgood(&self) -> XOSC_HF_FREQGOOD_R {
        XOSC_HF_FREQGOOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - XOSC_HF_RF_FREQGOOD"]
    #[inline(always)]
    pub fn xosc_hf_rf_freqgood(&self) -> XOSC_HF_RF_FREQGOOD_R {
        XOSC_HF_RF_FREQGOOD_R::new((self.bits & 0x01) != 0)
    }
}
