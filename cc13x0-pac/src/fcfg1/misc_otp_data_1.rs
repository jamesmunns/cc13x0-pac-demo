#[doc = "Reader of register MISC_OTP_DATA_1"]
pub type R = crate::R<u32, super::MISC_OTP_DATA_1>;
#[doc = "Reader of field `PEAK_DET_ITRIM`"]
pub type PEAK_DET_ITRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `HP_BUF_ITRIM`"]
pub type HP_BUF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `LP_BUF_ITRIM`"]
pub type LP_BUF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `DBLR_LOOP_FILTER_RESET_VOLTAGE`"]
pub type DBLR_LOOP_FILTER_RESET_VOLTAGE_R = crate::R<u8, u8>;
#[doc = "Reader of field `HPM_IBIAS_WAIT_CNT`"]
pub type HPM_IBIAS_WAIT_CNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `LPM_IBIAS_WAIT_CNT`"]
pub type LPM_IBIAS_WAIT_CNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `IDAC_STEP`"]
pub type IDAC_STEP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 27:28 - PEAK_DET_ITRIM"]
    #[inline(always)]
    pub fn peak_det_itrim(&self) -> PEAK_DET_ITRIM_R {
        PEAK_DET_ITRIM_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - HP_BUF_ITRIM"]
    #[inline(always)]
    pub fn hp_buf_itrim(&self) -> HP_BUF_ITRIM_R {
        HP_BUF_ITRIM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 22:23 - LP_BUF_ITRIM"]
    #[inline(always)]
    pub fn lp_buf_itrim(&self) -> LP_BUF_ITRIM_R {
        LP_BUF_ITRIM_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - DBLR_LOOP_FILTER_RESET_VOLTAGE"]
    #[inline(always)]
    pub fn dblr_loop_filter_reset_voltage(&self) -> DBLR_LOOP_FILTER_RESET_VOLTAGE_R {
        DBLR_LOOP_FILTER_RESET_VOLTAGE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 10:19 - HPM_IBIAS_WAIT_CNT"]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&self) -> HPM_IBIAS_WAIT_CNT_R {
        HPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 4:9 - LPM_IBIAS_WAIT_CNT"]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&self) -> LPM_IBIAS_WAIT_CNT_R {
        LPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 0:3 - IDAC_STEP"]
    #[inline(always)]
    pub fn idac_step(&self) -> IDAC_STEP_R {
        IDAC_STEP_R::new((self.bits & 0x0f) as u8)
    }
}
