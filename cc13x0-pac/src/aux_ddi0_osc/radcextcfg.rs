#[doc = "Reader of register RADCEXTCFG"]
pub type R = crate::R<u32, super::RADCEXTCFG>;
#[doc = "Writer for register RADCEXTCFG"]
pub type W = crate::W<u32, super::RADCEXTCFG>;
#[doc = "Register RADCEXTCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::RADCEXTCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HPM_IBIAS_WAIT_CNT`"]
pub type HPM_IBIAS_WAIT_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HPM_IBIAS_WAIT_CNT`"]
pub struct HPM_IBIAS_WAIT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_IBIAS_WAIT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | (((value as u32) & 0x03ff) << 22);
        self.w
    }
}
#[doc = "Reader of field `LPM_IBIAS_WAIT_CNT`"]
pub type LPM_IBIAS_WAIT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPM_IBIAS_WAIT_CNT`"]
pub struct LPM_IBIAS_WAIT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_IBIAS_WAIT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `IDAC_STEP`"]
pub type IDAC_STEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDAC_STEP`"]
pub struct IDAC_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RADC_DAC_TH`"]
pub type RADC_DAC_TH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RADC_DAC_TH`"]
pub struct RADC_DAC_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> RADC_DAC_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `RADC_MODE_IS_SAR`"]
pub type RADC_MODE_IS_SAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RADC_MODE_IS_SAR`"]
pub struct RADC_MODE_IS_SAR_W<'a> {
    w: &'a mut W,
}
impl<'a> RADC_MODE_IS_SAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:31 - HPM_IBIAS_WAIT_CNT"]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&self) -> HPM_IBIAS_WAIT_CNT_R {
        HPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:21 - LPM_IBIAS_WAIT_CNT"]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&self) -> LPM_IBIAS_WAIT_CNT_R {
        LPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - IDAC_STEP"]
    #[inline(always)]
    pub fn idac_step(&self) -> IDAC_STEP_R {
        IDAC_STEP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 6:11 - RADC_DAC_TH"]
    #[inline(always)]
    pub fn radc_dac_th(&self) -> RADC_DAC_TH_R {
        RADC_DAC_TH_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 5 - RADC_MODE_IS_SAR"]
    #[inline(always)]
    pub fn radc_mode_is_sar(&self) -> RADC_MODE_IS_SAR_R {
        RADC_MODE_IS_SAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 22:31 - HPM_IBIAS_WAIT_CNT"]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&mut self) -> HPM_IBIAS_WAIT_CNT_W {
        HPM_IBIAS_WAIT_CNT_W { w: self }
    }
    #[doc = "Bits 16:21 - LPM_IBIAS_WAIT_CNT"]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&mut self) -> LPM_IBIAS_WAIT_CNT_W {
        LPM_IBIAS_WAIT_CNT_W { w: self }
    }
    #[doc = "Bits 12:15 - IDAC_STEP"]
    #[inline(always)]
    pub fn idac_step(&mut self) -> IDAC_STEP_W {
        IDAC_STEP_W { w: self }
    }
    #[doc = "Bits 6:11 - RADC_DAC_TH"]
    #[inline(always)]
    pub fn radc_dac_th(&mut self) -> RADC_DAC_TH_W {
        RADC_DAC_TH_W { w: self }
    }
    #[doc = "Bit 5 - RADC_MODE_IS_SAR"]
    #[inline(always)]
    pub fn radc_mode_is_sar(&mut self) -> RADC_MODE_IS_SAR_W {
        RADC_MODE_IS_SAR_W { w: self }
    }
}
