#[doc = "Reader of register SHDW_OSC_BIAS_LDO_TRIM"]
pub type R = crate::R<u32, super::SHDW_OSC_BIAS_LDO_TRIM>;
#[doc = "Reader of field `SET_RCOSC_HF_COARSE_RESISTOR`"]
pub type SET_RCOSC_HF_COARSE_RESISTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIMMAG`"]
pub type TRIMMAG_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIMIREF`"]
pub type TRIMIREF_R = crate::R<u8, u8>;
#[doc = "Reader of field `ITRIM_DIG_LDO`"]
pub type ITRIM_DIG_LDO_R = crate::R<u8, u8>;
#[doc = "Reader of field `VTRIM_DIG`"]
pub type VTRIM_DIG_R = crate::R<u8, u8>;
#[doc = "Reader of field `VTRIM_COARSE`"]
pub type VTRIM_COARSE_R = crate::R<u8, u8>;
#[doc = "Reader of field `RCOSCHF_CTRIM`"]
pub type RCOSCHF_CTRIM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 27:28 - SET_RCOSC_HF_COARSE_RESISTOR"]
    #[inline(always)]
    pub fn set_rcosc_hf_coarse_resistor(&self) -> SET_RCOSC_HF_COARSE_RESISTOR_R {
        SET_RCOSC_HF_COARSE_RESISTOR_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 23:26 - TRIMMAG"]
    #[inline(always)]
    pub fn trimmag(&self) -> TRIMMAG_R {
        TRIMMAG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 18:22 - TRIMIREF"]
    #[inline(always)]
    pub fn trimiref(&self) -> TRIMIREF_R {
        TRIMIREF_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - ITRIM_DIG_LDO"]
    #[inline(always)]
    pub fn itrim_dig_ldo(&self) -> ITRIM_DIG_LDO_R {
        ITRIM_DIG_LDO_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - VTRIM_DIG"]
    #[inline(always)]
    pub fn vtrim_dig(&self) -> VTRIM_DIG_R {
        VTRIM_DIG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - VTRIM_COARSE"]
    #[inline(always)]
    pub fn vtrim_coarse(&self) -> VTRIM_COARSE_R {
        VTRIM_COARSE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - RCOSCHF_CTRIM"]
    #[inline(always)]
    pub fn rcoschf_ctrim(&self) -> RCOSCHF_CTRIM_R {
        RCOSCHF_CTRIM_R::new((self.bits & 0xff) as u8)
    }
}
