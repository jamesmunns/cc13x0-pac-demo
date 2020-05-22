#[doc = "Reader of register SOC_ADC_REL_GAIN"]
pub type R = crate::R<u32, super::SOC_ADC_REL_GAIN>;
#[doc = "Reader of field `SOC_ADC_REL_GAIN_TEMP1`"]
pub type SOC_ADC_REL_GAIN_TEMP1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - SOC_ADC_REL_GAIN_TEMP1"]
    #[inline(always)]
    pub fn soc_adc_rel_gain_temp1(&self) -> SOC_ADC_REL_GAIN_TEMP1_R {
        SOC_ADC_REL_GAIN_TEMP1_R::new((self.bits & 0xffff) as u16)
    }
}
