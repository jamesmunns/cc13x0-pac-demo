#[doc = "Reader of register SIZE_AND_DIS_FLAGS"]
pub type R = crate::R<u32, super::SIZE_AND_DIS_FLAGS>;
#[doc = "Reader of field `SIZE_OF_CCFG`"]
pub type SIZE_OF_CCFG_R = crate::R<u16, u16>;
#[doc = "Reader of field `DISABLE_FLAGS`"]
pub type DISABLE_FLAGS_R = crate::R<u16, u16>;
#[doc = "Reader of field `DIS_GPRAM`"]
pub type DIS_GPRAM_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_ALT_DCDC_SETTING`"]
pub type DIS_ALT_DCDC_SETTING_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_XOSC_OVR`"]
pub type DIS_XOSC_OVR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 16:31 - SIZE_OF_CCFG"]
    #[inline(always)]
    pub fn size_of_ccfg(&self) -> SIZE_OF_CCFG_R {
        SIZE_OF_CCFG_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 3:15 - DISABLE_FLAGS"]
    #[inline(always)]
    pub fn disable_flags(&self) -> DISABLE_FLAGS_R {
        DISABLE_FLAGS_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bit 2 - DIS_GPRAM"]
    #[inline(always)]
    pub fn dis_gpram(&self) -> DIS_GPRAM_R {
        DIS_GPRAM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIS_ALT_DCDC_SETTING"]
    #[inline(always)]
    pub fn dis_alt_dcdc_setting(&self) -> DIS_ALT_DCDC_SETTING_R {
        DIS_ALT_DCDC_SETTING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIS_XOSC_OVR"]
    #[inline(always)]
    pub fn dis_xosc_ovr(&self) -> DIS_XOSC_OVR_R {
        DIS_XOSC_OVR_R::new((self.bits & 0x01) != 0)
    }
}
