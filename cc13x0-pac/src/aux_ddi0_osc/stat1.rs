#[doc = "Reader of register STAT1"]
pub type R = crate::R<u32, super::STAT1>;
#[doc = "Reader of field `RAMPSTATE`"]
pub type RAMPSTATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `HMP_UPDATE_AMP`"]
pub type HMP_UPDATE_AMP_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPM_UPDATE_AMP`"]
pub type LPM_UPDATE_AMP_R = crate::R<u8, u8>;
#[doc = "Reader of field `FORCE_RCOSC_HF`"]
pub type FORCE_RCOSC_HF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCLK_HF_EN`"]
pub type SCLK_HF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCLK_MF_EN`"]
pub type SCLK_MF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACLK_ADC_EN`"]
pub type ACLK_ADC_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACLK_TDC_EN`"]
pub type ACLK_TDC_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACLK_REF_EN`"]
pub type ACLK_REF_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLK_CHP_EN`"]
pub type CLK_CHP_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLK_DCDC_EN`"]
pub type CLK_DCDC_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCLK_HF_GOOD`"]
pub type SCLK_HF_GOOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCLK_MF_GOOD`"]
pub type SCLK_MF_GOOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCLK_LF_GOOD`"]
pub type SCLK_LF_GOOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACLK_ADC_GOOD`"]
pub type ACLK_ADC_GOOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACLK_TDC_GOOD`"]
pub type ACLK_TDC_GOOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACLK_REF_GOOD`"]
pub type ACLK_REF_GOOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLK_CHP_GOOD`"]
pub type CLK_CHP_GOOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLK_DCDC_GOOD`"]
pub type CLK_DCDC_GOOD_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 28:31 - RAMPSTATE"]
    #[inline(always)]
    pub fn rampstate(&self) -> RAMPSTATE_R {
        RAMPSTATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 22:27 - HMP_UPDATE_AMP"]
    #[inline(always)]
    pub fn hmp_update_amp(&self) -> HMP_UPDATE_AMP_R {
        HMP_UPDATE_AMP_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - LPM_UPDATE_AMP"]
    #[inline(always)]
    pub fn lpm_update_amp(&self) -> LPM_UPDATE_AMP_R {
        LPM_UPDATE_AMP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - FORCE_RCOSC_HF"]
    #[inline(always)]
    pub fn force_rcosc_hf(&self) -> FORCE_RCOSC_HF_R {
        FORCE_RCOSC_HF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SCLK_HF_EN"]
    #[inline(always)]
    pub fn sclk_hf_en(&self) -> SCLK_HF_EN_R {
        SCLK_HF_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SCLK_MF_EN"]
    #[inline(always)]
    pub fn sclk_mf_en(&self) -> SCLK_MF_EN_R {
        SCLK_MF_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ACLK_ADC_EN"]
    #[inline(always)]
    pub fn aclk_adc_en(&self) -> ACLK_ADC_EN_R {
        ACLK_ADC_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ACLK_TDC_EN"]
    #[inline(always)]
    pub fn aclk_tdc_en(&self) -> ACLK_TDC_EN_R {
        ACLK_TDC_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ACLK_REF_EN"]
    #[inline(always)]
    pub fn aclk_ref_en(&self) -> ACLK_REF_EN_R {
        ACLK_REF_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CLK_CHP_EN"]
    #[inline(always)]
    pub fn clk_chp_en(&self) -> CLK_CHP_EN_R {
        CLK_CHP_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CLK_DCDC_EN"]
    #[inline(always)]
    pub fn clk_dcdc_en(&self) -> CLK_DCDC_EN_R {
        CLK_DCDC_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SCLK_HF_GOOD"]
    #[inline(always)]
    pub fn sclk_hf_good(&self) -> SCLK_HF_GOOD_R {
        SCLK_HF_GOOD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SCLK_MF_GOOD"]
    #[inline(always)]
    pub fn sclk_mf_good(&self) -> SCLK_MF_GOOD_R {
        SCLK_MF_GOOD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SCLK_LF_GOOD"]
    #[inline(always)]
    pub fn sclk_lf_good(&self) -> SCLK_LF_GOOD_R {
        SCLK_LF_GOOD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ACLK_ADC_GOOD"]
    #[inline(always)]
    pub fn aclk_adc_good(&self) -> ACLK_ADC_GOOD_R {
        ACLK_ADC_GOOD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ACLK_TDC_GOOD"]
    #[inline(always)]
    pub fn aclk_tdc_good(&self) -> ACLK_TDC_GOOD_R {
        ACLK_TDC_GOOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ACLK_REF_GOOD"]
    #[inline(always)]
    pub fn aclk_ref_good(&self) -> ACLK_REF_GOOD_R {
        ACLK_REF_GOOD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CLK_CHP_GOOD"]
    #[inline(always)]
    pub fn clk_chp_good(&self) -> CLK_CHP_GOOD_R {
        CLK_CHP_GOOD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CLK_DCDC_GOOD"]
    #[inline(always)]
    pub fn clk_dcdc_good(&self) -> CLK_DCDC_GOOD_R {
        CLK_DCDC_GOOD_R::new((self.bits & 0x01) != 0)
    }
}
