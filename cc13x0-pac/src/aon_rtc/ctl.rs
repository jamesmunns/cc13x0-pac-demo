#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMB_EV_MASK`"]
pub type COMB_EV_MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMB_EV_MASK`"]
pub struct COMB_EV_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> COMB_EV_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `EV_DELAY`"]
pub type EV_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EV_DELAY`"]
pub struct EV_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTC_4KHZ_EN`"]
pub type RTC_4KHZ_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_4KHZ_EN`"]
pub struct RTC_4KHZ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_4KHZ_EN_W<'a> {
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
#[doc = "Reader of field `RTC_UPD_EN`"]
pub type RTC_UPD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_UPD_EN`"]
pub struct RTC_UPD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_UPD_EN_W<'a> {
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
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bits 16:18 - COMB_EV_MASK"]
    #[inline(always)]
    pub fn comb_ev_mask(&self) -> COMB_EV_MASK_R {
        COMB_EV_MASK_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - EV_DELAY"]
    #[inline(always)]
    pub fn ev_delay(&self) -> EV_DELAY_R {
        EV_DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC_4KHZ_EN"]
    #[inline(always)]
    pub fn rtc_4khz_en(&self) -> RTC_4KHZ_EN_R {
        RTC_4KHZ_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC_UPD_EN"]
    #[inline(always)]
    pub fn rtc_upd_en(&self) -> RTC_UPD_EN_R {
        RTC_UPD_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18 - COMB_EV_MASK"]
    #[inline(always)]
    pub fn comb_ev_mask(&mut self) -> COMB_EV_MASK_W {
        COMB_EV_MASK_W { w: self }
    }
    #[doc = "Bits 8:11 - EV_DELAY"]
    #[inline(always)]
    pub fn ev_delay(&mut self) -> EV_DELAY_W {
        EV_DELAY_W { w: self }
    }
    #[doc = "Bit 7 - RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 2 - RTC_4KHZ_EN"]
    #[inline(always)]
    pub fn rtc_4khz_en(&mut self) -> RTC_4KHZ_EN_W {
        RTC_4KHZ_EN_W { w: self }
    }
    #[doc = "Bit 1 - RTC_UPD_EN"]
    #[inline(always)]
    pub fn rtc_upd_en(&mut self) -> RTC_UPD_EN_W {
        RTC_UPD_EN_W { w: self }
    }
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
