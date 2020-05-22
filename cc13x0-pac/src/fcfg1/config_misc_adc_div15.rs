#[doc = "Reader of register CONFIG_MISC_ADC_DIV15"]
pub type R = crate::R<u32, super::CONFIG_MISC_ADC_DIV15>;
#[doc = "Reader of field `RSSI_OFFSET`"]
pub type RSSI_OFFSET_R = crate::R<u8, u8>;
#[doc = "Reader of field `QUANTCTLTHRES`"]
pub type QUANTCTLTHRES_R = crate::R<u8, u8>;
#[doc = "Reader of field `DACTRIM`"]
pub type DACTRIM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 9:16 - RSSI_OFFSET"]
    #[inline(always)]
    pub fn rssi_offset(&self) -> RSSI_OFFSET_R {
        RSSI_OFFSET_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 6:8 - QUANTCTLTHRES"]
    #[inline(always)]
    pub fn quantctlthres(&self) -> QUANTCTLTHRES_R {
        QUANTCTLTHRES_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 0:5 - DACTRIM"]
    #[inline(always)]
    pub fn dactrim(&self) -> DACTRIM_R {
        DACTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
