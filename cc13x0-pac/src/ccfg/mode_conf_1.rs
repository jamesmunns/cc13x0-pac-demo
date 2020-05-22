#[doc = "Reader of register MODE_CONF_1"]
pub type R = crate::R<u32, super::MODE_CONF_1>;
#[doc = "Reader of field `ALT_DCDC_VMIN`"]
pub type ALT_DCDC_VMIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `ALT_DCDC_DITHER_EN`"]
pub type ALT_DCDC_DITHER_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALT_DCDC_IPEAK`"]
pub type ALT_DCDC_IPEAK_R = crate::R<u8, u8>;
#[doc = "Reader of field `DELTA_IBIAS_INIT`"]
pub type DELTA_IBIAS_INIT_R = crate::R<u8, u8>;
#[doc = "Reader of field `DELTA_IBIAS_OFFSET`"]
pub type DELTA_IBIAS_OFFSET_R = crate::R<u8, u8>;
#[doc = "Reader of field `XOSC_MAX_START`"]
pub type XOSC_MAX_START_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 20:23 - ALT_DCDC_VMIN"]
    #[inline(always)]
    pub fn alt_dcdc_vmin(&self) -> ALT_DCDC_VMIN_R {
        ALT_DCDC_VMIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - ALT_DCDC_DITHER_EN"]
    #[inline(always)]
    pub fn alt_dcdc_dither_en(&self) -> ALT_DCDC_DITHER_EN_R {
        ALT_DCDC_DITHER_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - ALT_DCDC_IPEAK"]
    #[inline(always)]
    pub fn alt_dcdc_ipeak(&self) -> ALT_DCDC_IPEAK_R {
        ALT_DCDC_IPEAK_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:15 - DELTA_IBIAS_INIT"]
    #[inline(always)]
    pub fn delta_ibias_init(&self) -> DELTA_IBIAS_INIT_R {
        DELTA_IBIAS_INIT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DELTA_IBIAS_OFFSET"]
    #[inline(always)]
    pub fn delta_ibias_offset(&self) -> DELTA_IBIAS_OFFSET_R {
        DELTA_IBIAS_OFFSET_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - XOSC_MAX_START"]
    #[inline(always)]
    pub fn xosc_max_start(&self) -> XOSC_MAX_START_R {
        XOSC_MAX_START_R::new((self.bits & 0xff) as u8)
    }
}
