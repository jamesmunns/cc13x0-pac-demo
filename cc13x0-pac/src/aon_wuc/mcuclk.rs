#[doc = "Reader of register MCUCLK"]
pub type R = crate::R<u32, super::MCUCLK>;
#[doc = "Writer for register MCUCLK"]
pub type W = crate::W<u32, super::MCUCLK>;
#[doc = "Register MCUCLK `reset()`'s with value 0"]
impl crate::ResetValue for super::MCUCLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCOSC_HF_CAL_DONE`"]
pub type RCOSC_HF_CAL_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSC_HF_CAL_DONE`"]
pub struct RCOSC_HF_CAL_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_HF_CAL_DONE_W<'a> {
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
#[doc = "Reader of field `PWR_DWN_SRC`"]
pub type PWR_DWN_SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PWR_DWN_SRC`"]
pub struct PWR_DWN_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DWN_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - RCOSC_HF_CAL_DONE"]
    #[inline(always)]
    pub fn rcosc_hf_cal_done(&self) -> RCOSC_HF_CAL_DONE_R {
        RCOSC_HF_CAL_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - PWR_DWN_SRC"]
    #[inline(always)]
    pub fn pwr_dwn_src(&self) -> PWR_DWN_SRC_R {
        PWR_DWN_SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - RCOSC_HF_CAL_DONE"]
    #[inline(always)]
    pub fn rcosc_hf_cal_done(&mut self) -> RCOSC_HF_CAL_DONE_W {
        RCOSC_HF_CAL_DONE_W { w: self }
    }
    #[doc = "Bits 0:1 - PWR_DWN_SRC"]
    #[inline(always)]
    pub fn pwr_dwn_src(&mut self) -> PWR_DWN_SRC_W {
        PWR_DWN_SRC_W { w: self }
    }
}
