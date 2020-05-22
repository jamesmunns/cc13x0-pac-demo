#[doc = "Reader of register OSC_CONF"]
pub type R = crate::R<u32, super::OSC_CONF>;
#[doc = "Reader of field `ADC_SH_VBUF_EN`"]
pub type ADC_SH_VBUF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_SH_MODE_EN`"]
pub type ADC_SH_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ATESTLF_RCOSCLF_IBIAS_TRIM`"]
pub type ATESTLF_RCOSCLF_IBIAS_TRIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSCLF_REGULATOR_TRIM`"]
pub type XOSCLF_REGULATOR_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `XOSCLF_CMIRRWR_RATIO`"]
pub type XOSCLF_CMIRRWR_RATIO_R = crate::R<u8, u8>;
#[doc = "Reader of field `XOSC_HF_FAST_START`"]
pub type XOSC_HF_FAST_START_R = crate::R<u8, u8>;
#[doc = "Reader of field `XOSC_OPTION`"]
pub type XOSC_OPTION_R = crate::R<bool, bool>;
#[doc = "Reader of field `BAW_OPTION`"]
pub type BAW_OPTION_R = crate::R<bool, bool>;
#[doc = "Reader of field `BAW_BIAS_HOLD_MODE_EN`"]
pub type BAW_BIAS_HOLD_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BAW_CURRMIRR_RATIO`"]
pub type BAW_CURRMIRR_RATIO_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAW_BIAS_RES_SET`"]
pub type BAW_BIAS_RES_SET_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAW_FILTER_EN`"]
pub type BAW_FILTER_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BAW_BIAS_RECHARGE_DELAY`"]
pub type BAW_BIAS_RECHARGE_DELAY_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAW_SERIES_CAP`"]
pub type BAW_SERIES_CAP_R = crate::R<u8, u8>;
#[doc = "Reader of field `BAW_DIV3_BYPASS`"]
pub type BAW_DIV3_BYPASS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 29 - ADC_SH_VBUF_EN"]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&self) -> ADC_SH_VBUF_EN_R {
        ADC_SH_VBUF_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADC_SH_MODE_EN"]
    #[inline(always)]
    pub fn adc_sh_mode_en(&self) -> ADC_SH_MODE_EN_R {
        ADC_SH_MODE_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ATESTLF_RCOSCLF_IBIAS_TRIM"]
    #[inline(always)]
    pub fn atestlf_rcosclf_ibias_trim(&self) -> ATESTLF_RCOSCLF_IBIAS_TRIM_R {
        ATESTLF_RCOSCLF_IBIAS_TRIM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - XOSCLF_REGULATOR_TRIM"]
    #[inline(always)]
    pub fn xosclf_regulator_trim(&self) -> XOSCLF_REGULATOR_TRIM_R {
        XOSCLF_REGULATOR_TRIM_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 21:24 - XOSCLF_CMIRRWR_RATIO"]
    #[inline(always)]
    pub fn xosclf_cmirrwr_ratio(&self) -> XOSCLF_CMIRRWR_RATIO_R {
        XOSCLF_CMIRRWR_RATIO_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 19:20 - XOSC_HF_FAST_START"]
    #[inline(always)]
    pub fn xosc_hf_fast_start(&self) -> XOSC_HF_FAST_START_R {
        XOSC_HF_FAST_START_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 18 - XOSC_OPTION"]
    #[inline(always)]
    pub fn xosc_option(&self) -> XOSC_OPTION_R {
        XOSC_OPTION_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - BAW_OPTION"]
    #[inline(always)]
    pub fn baw_option(&self) -> BAW_OPTION_R {
        BAW_OPTION_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BAW_BIAS_HOLD_MODE_EN"]
    #[inline(always)]
    pub fn baw_bias_hold_mode_en(&self) -> BAW_BIAS_HOLD_MODE_EN_R {
        BAW_BIAS_HOLD_MODE_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - BAW_CURRMIRR_RATIO"]
    #[inline(always)]
    pub fn baw_currmirr_ratio(&self) -> BAW_CURRMIRR_RATIO_R {
        BAW_CURRMIRR_RATIO_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - BAW_BIAS_RES_SET"]
    #[inline(always)]
    pub fn baw_bias_res_set(&self) -> BAW_BIAS_RES_SET_R {
        BAW_BIAS_RES_SET_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - BAW_FILTER_EN"]
    #[inline(always)]
    pub fn baw_filter_en(&self) -> BAW_FILTER_EN_R {
        BAW_FILTER_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - BAW_BIAS_RECHARGE_DELAY"]
    #[inline(always)]
    pub fn baw_bias_recharge_delay(&self) -> BAW_BIAS_RECHARGE_DELAY_R {
        BAW_BIAS_RECHARGE_DELAY_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - BAW_SERIES_CAP"]
    #[inline(always)]
    pub fn baw_series_cap(&self) -> BAW_SERIES_CAP_R {
        BAW_SERIES_CAP_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - BAW_DIV3_BYPASS"]
    #[inline(always)]
    pub fn baw_div3_bypass(&self) -> BAW_DIV3_BYPASS_R {
        BAW_DIV3_BYPASS_R::new((self.bits & 0x01) != 0)
    }
}
