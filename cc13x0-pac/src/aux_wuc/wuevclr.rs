#[doc = "Reader of register WUEVCLR"]
pub type R = crate::R<u32, super::WUEVCLR>;
#[doc = "Writer for register WUEVCLR"]
pub type W = crate::W<u32, super::WUEVCLR>;
#[doc = "Register WUEVCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::WUEVCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AON_RTC_CH2`"]
pub type AON_RTC_CH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AON_RTC_CH2`"]
pub struct AON_RTC_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_RTC_CH2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `AON_SW`"]
pub type AON_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AON_SW`"]
pub struct AON_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_SW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AON_PROG_WU`"]
pub type AON_PROG_WU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AON_PROG_WU`"]
pub struct AON_PROG_WU_W<'a> {
    w: &'a mut W,
}
impl<'a> AON_PROG_WU_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
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
impl W {
    #[doc = "Bit 2 - AON_RTC_CH2"]
    #[inline(always)]
    pub fn aon_rtc_ch2(&mut self) -> AON_RTC_CH2_W {
        AON_RTC_CH2_W { w: self }
    }
    #[doc = "Bit 1 - AON_SW"]
    #[inline(always)]
    pub fn aon_sw(&mut self) -> AON_SW_W {
        AON_SW_W { w: self }
    }
    #[doc = "Bit 0 - AON_PROG_WU"]
    #[inline(always)]
    pub fn aon_prog_wu(&mut self) -> AON_PROG_WU_W {
        AON_PROG_WU_W { w: self }
    }
}
