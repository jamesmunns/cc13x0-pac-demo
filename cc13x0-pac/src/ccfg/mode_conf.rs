#[doc = "Reader of register MODE_CONF"]
pub type R = crate::R<u32, super::MODE_CONF>;
#[doc = "Reader of field `VDDR_TRIM_SLEEP_DELTA`"]
pub type VDDR_TRIM_SLEEP_DELTA_R = crate::R<u8, u8>;
#[doc = "Reader of field `DCDC_RECHARGE`"]
pub type DCDC_RECHARGE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDC_ACTIVE`"]
pub type DCDC_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDDR_EXT_LOAD`"]
pub type VDDR_EXT_LOAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `VDDS_BOD_LEVEL`"]
pub type VDDS_BOD_LEVEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCLK_LF_OPTION`"]
pub type SCLK_LF_OPTION_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_TRIM_SLEEP_TC`"]
pub type VDDR_TRIM_SLEEP_TC_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_COMP`"]
pub type RTC_COMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC_FREQ`"]
pub type XOSC_FREQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `XOSC_CAP_MOD`"]
pub type XOSC_CAP_MOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `HF_COMP`"]
pub type HF_COMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `XOSC_CAPARRAY_DELTA`"]
pub type XOSC_CAPARRAY_DELTA_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_CAP`"]
pub type VDDR_CAP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31 - VDDR_TRIM_SLEEP_DELTA"]
    #[inline(always)]
    pub fn vddr_trim_sleep_delta(&self) -> VDDR_TRIM_SLEEP_DELTA_R {
        VDDR_TRIM_SLEEP_DELTA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - DCDC_RECHARGE"]
    #[inline(always)]
    pub fn dcdc_recharge(&self) -> DCDC_RECHARGE_R {
        DCDC_RECHARGE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DCDC_ACTIVE"]
    #[inline(always)]
    pub fn dcdc_active(&self) -> DCDC_ACTIVE_R {
        DCDC_ACTIVE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - VDDR_EXT_LOAD"]
    #[inline(always)]
    pub fn vddr_ext_load(&self) -> VDDR_EXT_LOAD_R {
        VDDR_EXT_LOAD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - VDDS_BOD_LEVEL"]
    #[inline(always)]
    pub fn vdds_bod_level(&self) -> VDDS_BOD_LEVEL_R {
        VDDS_BOD_LEVEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - SCLK_LF_OPTION"]
    #[inline(always)]
    pub fn sclk_lf_option(&self) -> SCLK_LF_OPTION_R {
        SCLK_LF_OPTION_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 21 - VDDR_TRIM_SLEEP_TC"]
    #[inline(always)]
    pub fn vddr_trim_sleep_tc(&self) -> VDDR_TRIM_SLEEP_TC_R {
        VDDR_TRIM_SLEEP_TC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - RTC_COMP"]
    #[inline(always)]
    pub fn rtc_comp(&self) -> RTC_COMP_R {
        RTC_COMP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - XOSC_FREQ"]
    #[inline(always)]
    pub fn xosc_freq(&self) -> XOSC_FREQ_R {
        XOSC_FREQ_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - XOSC_CAP_MOD"]
    #[inline(always)]
    pub fn xosc_cap_mod(&self) -> XOSC_CAP_MOD_R {
        XOSC_CAP_MOD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HF_COMP"]
    #[inline(always)]
    pub fn hf_comp(&self) -> HF_COMP_R {
        HF_COMP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - XOSC_CAPARRAY_DELTA"]
    #[inline(always)]
    pub fn xosc_caparray_delta(&self) -> XOSC_CAPARRAY_DELTA_R {
        XOSC_CAPARRAY_DELTA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - VDDR_CAP"]
    #[inline(always)]
    pub fn vddr_cap(&self) -> VDDR_CAP_R {
        VDDR_CAP_R::new((self.bits & 0xff) as u8)
    }
}
