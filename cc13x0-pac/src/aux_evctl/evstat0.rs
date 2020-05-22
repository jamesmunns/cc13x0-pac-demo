#[doc = "Reader of register EVSTAT0"]
pub type R = crate::R<u32, super::EVSTAT0>;
#[doc = "Reader of field `AUXIO2`"]
pub type AUXIO2_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO1`"]
pub type AUXIO1_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXIO0`"]
pub type AUXIO0_R = crate::R<bool, bool>;
#[doc = "Reader of field `AON_PROG_WU`"]
pub type AON_PROG_WU_R = crate::R<bool, bool>;
#[doc = "Reader of field `AON_SW`"]
pub type AON_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `OBSMUX1`"]
pub type OBSMUX1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OBSMUX0`"]
pub type OBSMUX0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_FIFO_ALMOST_FULL`"]
pub type ADC_FIFO_ALMOST_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC_DONE`"]
pub type ADC_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SMPH_AUTOTAKE_DONE`"]
pub type SMPH_AUTOTAKE_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMER1_EV`"]
pub type TIMER1_EV_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMER0_EV`"]
pub type TIMER0_EV_R = crate::R<bool, bool>;
#[doc = "Reader of field `TDC_DONE`"]
pub type TDC_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_COMPB`"]
pub type AUX_COMPB_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUX_COMPA`"]
pub type AUX_COMPA_R = crate::R<bool, bool>;
#[doc = "Reader of field `AON_RTC_CH2`"]
pub type AON_RTC_CH2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - AUXIO2"]
    #[inline(always)]
    pub fn auxio2(&self) -> AUXIO2_R {
        AUXIO2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AUXIO1"]
    #[inline(always)]
    pub fn auxio1(&self) -> AUXIO1_R {
        AUXIO1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AUXIO0"]
    #[inline(always)]
    pub fn auxio0(&self) -> AUXIO0_R {
        AUXIO0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(&self) -> AON_PROG_WU_R {
        AON_PROG_WU_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AON_SW"]
    #[inline(always)]
    pub fn aon_sw(&self) -> AON_SW_R {
        AON_SW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - OBSMUX1"]
    #[inline(always)]
    pub fn obsmux1(&self) -> OBSMUX1_R {
        OBSMUX1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - OBSMUX0"]
    #[inline(always)]
    pub fn obsmux0(&self) -> OBSMUX0_R {
        OBSMUX0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC_FIFO_ALMOST_FULL"]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&self) -> ADC_FIFO_ALMOST_FULL_R {
        ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC_DONE"]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SMPH_AUTOTAKE_DONE"]
    #[inline(always)]
    pub fn smph_autotake_done(&self) -> SMPH_AUTOTAKE_DONE_R {
        SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIMER1_EV"]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIMER0_EV"]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TDC_DONE"]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AUX_COMPB"]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AUX_COMPA"]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2_R {
        AON_RTC_CH2_R::new((self.bits & 0x01) != 0)
    }
}
