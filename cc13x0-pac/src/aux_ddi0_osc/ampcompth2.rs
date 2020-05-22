#[doc = "Reader of register AMPCOMPTH2"]
pub type R = crate::R<u32, super::AMPCOMPTH2>;
#[doc = "Writer for register AMPCOMPTH2"]
pub type W = crate::W<u32, super::AMPCOMPTH2>;
#[doc = "Register AMPCOMPTH2 `reset()`'s with value 0"]
impl crate::ResetValue for super::AMPCOMPTH2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPMUPDATE_LTH`"]
pub type LPMUPDATE_LTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMUPDATE_LTH`"]
pub struct LPMUPDATE_LTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMUPDATE_LTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Reader of field `LPMUPDATE_HTH`"]
pub type LPMUPDATE_HTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPMUPDATE_HTH`"]
pub struct LPMUPDATE_HTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMUPDATE_HTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `ADC_COMP_AMPTH_LPM`"]
pub type ADC_COMP_AMPTH_LPM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_COMP_AMPTH_LPM`"]
pub struct ADC_COMP_AMPTH_LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_COMP_AMPTH_LPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 10)) | (((value as u32) & 0x3f) << 10);
        self.w
    }
}
#[doc = "Reader of field `ADC_COMP_AMPTH_HPM`"]
pub type ADC_COMP_AMPTH_HPM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_COMP_AMPTH_HPM`"]
pub struct ADC_COMP_AMPTH_HPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_COMP_AMPTH_HPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31 - LPMUPDATE_LTH"]
    #[inline(always)]
    pub fn lpmupdate_lth(&self) -> LPMUPDATE_LTH_R {
        LPMUPDATE_LTH_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - LPMUPDATE_HTH"]
    #[inline(always)]
    pub fn lpmupdate_hth(&self) -> LPMUPDATE_HTH_R {
        LPMUPDATE_HTH_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - ADC_COMP_AMPTH_LPM"]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&self) -> ADC_COMP_AMPTH_LPM_R {
        ADC_COMP_AMPTH_LPM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 2:7 - ADC_COMP_AMPTH_HPM"]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&self) -> ADC_COMP_AMPTH_HPM_R {
        ADC_COMP_AMPTH_HPM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31 - LPMUPDATE_LTH"]
    #[inline(always)]
    pub fn lpmupdate_lth(&mut self) -> LPMUPDATE_LTH_W {
        LPMUPDATE_LTH_W { w: self }
    }
    #[doc = "Bits 18:23 - LPMUPDATE_HTH"]
    #[inline(always)]
    pub fn lpmupdate_hth(&mut self) -> LPMUPDATE_HTH_W {
        LPMUPDATE_HTH_W { w: self }
    }
    #[doc = "Bits 10:15 - ADC_COMP_AMPTH_LPM"]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&mut self) -> ADC_COMP_AMPTH_LPM_W {
        ADC_COMP_AMPTH_LPM_W { w: self }
    }
    #[doc = "Bits 2:7 - ADC_COMP_AMPTH_HPM"]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&mut self) -> ADC_COMP_AMPTH_HPM_W {
        ADC_COMP_AMPTH_HPM_W { w: self }
    }
}
