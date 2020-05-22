#[doc = "Reader of register ADCDOUBLERNANOAMPCTL"]
pub type R = crate::R<u32, super::ADCDOUBLERNANOAMPCTL>;
#[doc = "Writer for register ADCDOUBLERNANOAMPCTL"]
pub type W = crate::W<u32, super::ADCDOUBLERNANOAMPCTL>;
#[doc = "Register ADCDOUBLERNANOAMPCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCDOUBLERNANOAMPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NANOAMP_BIAS_ENABLE`"]
pub type NANOAMP_BIAS_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NANOAMP_BIAS_ENABLE`"]
pub struct NANOAMP_BIAS_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NANOAMP_BIAS_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPARE23`"]
pub type SPARE23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPARE23`"]
pub struct SPARE23_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ADC_SH_MODE_EN`"]
pub type ADC_SH_MODE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SH_MODE_EN`"]
pub struct ADC_SH_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SH_MODE_EN_W<'a> {
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
#[doc = "Reader of field `ADC_SH_VBUF_EN`"]
pub type ADC_SH_VBUF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC_SH_VBUF_EN`"]
pub struct ADC_SH_VBUF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SH_VBUF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC_IREF_CTRL`"]
pub type ADC_IREF_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC_IREF_CTRL`"]
pub struct ADC_IREF_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_IREF_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - NANOAMP_BIAS_ENABLE"]
    #[inline(always)]
    pub fn nanoamp_bias_enable(&self) -> NANOAMP_BIAS_ENABLE_R {
        NANOAMP_BIAS_ENABLE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SPARE23"]
    #[inline(always)]
    pub fn spare23(&self) -> SPARE23_R {
        SPARE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC_SH_MODE_EN"]
    #[inline(always)]
    pub fn adc_sh_mode_en(&self) -> ADC_SH_MODE_EN_R {
        ADC_SH_MODE_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC_SH_VBUF_EN"]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&self) -> ADC_SH_VBUF_EN_R {
        ADC_SH_VBUF_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - ADC_IREF_CTRL"]
    #[inline(always)]
    pub fn adc_iref_ctrl(&self) -> ADC_IREF_CTRL_R {
        ADC_IREF_CTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - NANOAMP_BIAS_ENABLE"]
    #[inline(always)]
    pub fn nanoamp_bias_enable(&mut self) -> NANOAMP_BIAS_ENABLE_W {
        NANOAMP_BIAS_ENABLE_W { w: self }
    }
    #[doc = "Bit 23 - SPARE23"]
    #[inline(always)]
    pub fn spare23(&mut self) -> SPARE23_W {
        SPARE23_W { w: self }
    }
    #[doc = "Bit 5 - ADC_SH_MODE_EN"]
    #[inline(always)]
    pub fn adc_sh_mode_en(&mut self) -> ADC_SH_MODE_EN_W {
        ADC_SH_MODE_EN_W { w: self }
    }
    #[doc = "Bit 4 - ADC_SH_VBUF_EN"]
    #[inline(always)]
    pub fn adc_sh_vbuf_en(&mut self) -> ADC_SH_VBUF_EN_W {
        ADC_SH_VBUF_EN_W { w: self }
    }
    #[doc = "Bits 0:1 - ADC_IREF_CTRL"]
    #[inline(always)]
    pub fn adc_iref_ctrl(&mut self) -> ADC_IREF_CTRL_W {
        ADC_IREF_CTRL_W { w: self }
    }
}
