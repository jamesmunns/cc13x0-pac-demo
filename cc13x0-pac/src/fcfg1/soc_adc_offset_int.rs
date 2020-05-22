#[doc = "Reader of register SOC_ADC_OFFSET_INT"]
pub type R = crate::R<u32, super::SOC_ADC_OFFSET_INT>;
#[doc = "Reader of field `SOC_ADC_REL_OFFSET_TEMP1`"]
pub type SOC_ADC_REL_OFFSET_TEMP1_R = crate::R<u8, u8>;
#[doc = "Reader of field `SOC_ADC_ABS_OFFSET_TEMP1`"]
pub type SOC_ADC_ABS_OFFSET_TEMP1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:23 - SOC_ADC_REL_OFFSET_TEMP1"]
    #[inline(always)]
    pub fn soc_adc_rel_offset_temp1(&self) -> SOC_ADC_REL_OFFSET_TEMP1_R {
        SOC_ADC_REL_OFFSET_TEMP1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - SOC_ADC_ABS_OFFSET_TEMP1"]
    #[inline(always)]
    pub fn soc_adc_abs_offset_temp1(&self) -> SOC_ADC_ABS_OFFSET_TEMP1_R {
        SOC_ADC_ABS_OFFSET_TEMP1_R::new((self.bits & 0xff) as u8)
    }
}
