#[doc = "Reader of register CONFIG_OSC_TOP"]
pub type R = crate::R<u32, super::CONFIG_OSC_TOP>;
#[doc = "Reader of field `XOSC_HF_ROW_Q12`"]
pub type XOSC_HF_ROW_Q12_R = crate::R<u8, u8>;
#[doc = "Reader of field `XOSC_HF_COLUMN_Q12`"]
pub type XOSC_HF_COLUMN_Q12_R = crate::R<u16, u16>;
#[doc = "Reader of field `RCOSCLF_CTUNE_TRIM`"]
pub type RCOSCLF_CTUNE_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `RCOSCLF_RTUNE_TRIM`"]
pub type RCOSCLF_RTUNE_TRIM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 26:29 - XOSC_HF_ROW_Q12"]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&self) -> XOSC_HF_ROW_Q12_R {
        XOSC_HF_ROW_Q12_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bits 10:25 - XOSC_HF_COLUMN_Q12"]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&self) -> XOSC_HF_COLUMN_Q12_R {
        XOSC_HF_COLUMN_Q12_R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bits 2:9 - RCOSCLF_CTUNE_TRIM"]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&self) -> RCOSCLF_CTUNE_TRIM_R {
        RCOSCLF_CTUNE_TRIM_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 0:1 - RCOSCLF_RTUNE_TRIM"]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&self) -> RCOSCLF_RTUNE_TRIM_R {
        RCOSCLF_RTUNE_TRIM_R::new((self.bits & 0x03) as u8)
    }
}
