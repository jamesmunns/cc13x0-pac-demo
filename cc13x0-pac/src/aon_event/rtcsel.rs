#[doc = "Reader of register RTCSEL"]
pub type R = crate::R<u32, super::RTCSEL>;
#[doc = "Writer for register RTCSEL"]
pub type W = crate::W<u32, super::RTCSEL>;
#[doc = "Register RTCSEL `reset()`'s with value 0x3f"]
impl crate::ResetValue for super::RTCSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f
    }
}
#[doc = "Reader of field `RTC_CH1_CAPT_EV`"]
pub type RTC_CH1_CAPT_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTC_CH1_CAPT_EV`"]
pub struct RTC_CH1_CAPT_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CH1_CAPT_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - RTC_CH1_CAPT_EV"]
    #[inline(always)]
    pub fn rtc_ch1_capt_ev(&self) -> RTC_CH1_CAPT_EV_R {
        RTC_CH1_CAPT_EV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - RTC_CH1_CAPT_EV"]
    #[inline(always)]
    pub fn rtc_ch1_capt_ev(&mut self) -> RTC_CH1_CAPT_EV_W {
        RTC_CH1_CAPT_EV_W { w: self }
    }
}
