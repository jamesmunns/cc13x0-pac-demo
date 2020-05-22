#[doc = "Reader of register ANA2_TRIM"]
pub type R = crate::R<u32, super::ANA2_TRIM>;
#[doc = "Reader of field `RCOSCHFCTRIMFRACT_EN`"]
pub type RCOSCHFCTRIMFRACT_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCOSCHFCTRIMFRACT`"]
pub type RCOSCHFCTRIMFRACT_R = crate::R<u8, u8>;
#[doc = "Reader of field `SET_RCOSC_HF_FINE_RESISTOR`"]
pub type SET_RCOSC_HF_FINE_RESISTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `ATESTLF_UDIGLDO_IBIAS_TRIM`"]
pub type ATESTLF_UDIGLDO_IBIAS_TRIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `NANOAMP_RES_TRIM`"]
pub type NANOAMP_RES_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `DITHER_EN`"]
pub type DITHER_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCDC_IPEAK`"]
pub type DCDC_IPEAK_R = crate::R<u8, u8>;
#[doc = "Reader of field `DEAD_TIME_TRIM`"]
pub type DEAD_TIME_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `DCDC_LOW_EN_SEL`"]
pub type DCDC_LOW_EN_SEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `DCDC_HIGH_EN_SEL`"]
pub type DCDC_HIGH_EN_SEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31 - RCOSCHFCTRIMFRACT_EN"]
    #[inline(always)]
    pub fn rcoschfctrimfract_en(&self) -> RCOSCHFCTRIMFRACT_EN_R {
        RCOSCHFCTRIMFRACT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - RCOSCHFCTRIMFRACT"]
    #[inline(always)]
    pub fn rcoschfctrimfract(&self) -> RCOSCHFCTRIMFRACT_R {
        RCOSCHFCTRIMFRACT_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 23:24 - SET_RCOSC_HF_FINE_RESISTOR"]
    #[inline(always)]
    pub fn set_rcosc_hf_fine_resistor(&self) -> SET_RCOSC_HF_FINE_RESISTOR_R {
        SET_RCOSC_HF_FINE_RESISTOR_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 22 - ATESTLF_UDIGLDO_IBIAS_TRIM"]
    #[inline(always)]
    pub fn atestlf_udigldo_ibias_trim(&self) -> ATESTLF_UDIGLDO_IBIAS_TRIM_R {
        ATESTLF_UDIGLDO_IBIAS_TRIM_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - NANOAMP_RES_TRIM"]
    #[inline(always)]
    pub fn nanoamp_res_trim(&self) -> NANOAMP_RES_TRIM_R {
        NANOAMP_RES_TRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 11 - DITHER_EN"]
    #[inline(always)]
    pub fn dither_en(&self) -> DITHER_EN_R {
        DITHER_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - DCDC_IPEAK"]
    #[inline(always)]
    pub fn dcdc_ipeak(&self) -> DCDC_IPEAK_R {
        DCDC_IPEAK_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 6:7 - DEAD_TIME_TRIM"]
    #[inline(always)]
    pub fn dead_time_trim(&self) -> DEAD_TIME_TRIM_R {
        DEAD_TIME_TRIM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5 - DCDC_LOW_EN_SEL"]
    #[inline(always)]
    pub fn dcdc_low_en_sel(&self) -> DCDC_LOW_EN_SEL_R {
        DCDC_LOW_EN_SEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - DCDC_HIGH_EN_SEL"]
    #[inline(always)]
    pub fn dcdc_high_en_sel(&self) -> DCDC_HIGH_EN_SEL_R {
        DCDC_HIGH_EN_SEL_R::new((self.bits & 0x07) as u8)
    }
}
