#[doc = "Reader of register WUEVFLAGS"]
pub type R = crate::R<u32, super::WUEVFLAGS>;
#[doc = "Reader of field `AON_RTC_CH2`"]
pub type AON_RTC_CH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `AON_SW`"]
pub type AON_SW_R = crate::R<bool, bool>;
#[doc = "Reader of field `AON_PROG_WU`"]
pub type AON_PROG_WU_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AON_RTC_CH2_R {
        AON_RTC_CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - AON_SW"]
    #[inline(always)]
    pub fn aon_sw(&self) -> AON_SW_R {
        AON_SW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(&self) -> AON_PROG_WU_R {
        AON_PROG_WU_R::new((self.bits & 0x01) != 0)
    }
}
