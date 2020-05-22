#[doc = "Reader of register SOC_ADC_REF_TRIM_AND_OFFSET_EXT"]
pub type R = crate::R<u32, super::SOC_ADC_REF_TRIM_AND_OFFSET_EXT>;
#[doc = "Reader of field `SOC_ADC_REF_VOLTAGE_TRIM_TEMP1`"]
pub type SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - SOC_ADC_REF_VOLTAGE_TRIM_TEMP1"]
    #[inline(always)]
    pub fn soc_adc_ref_voltage_trim_temp1(&self) -> SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_R {
        SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_R::new((self.bits & 0x3f) as u8)
    }
}
