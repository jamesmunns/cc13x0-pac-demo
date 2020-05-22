#[doc = "Reader of register CONFIG_SYNTH_DIV30"]
pub type R = crate::R<u32, super::CONFIG_SYNTH_DIV30>;
#[doc = "Reader of field `RFC_MDM_DEMIQMC0`"]
pub type RFC_MDM_DEMIQMC0_R = crate::R<u16, u16>;
#[doc = "Reader of field `LDOVCO_TRIM_OUTPUT`"]
pub type LDOVCO_TRIM_OUTPUT_R = crate::R<u8, u8>;
#[doc = "Reader of field `SLDO_TRIM_OUTPUT`"]
pub type SLDO_TRIM_OUTPUT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 12:27 - RFC_MDM_DEMIQMC0"]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0(&self) -> RFC_MDM_DEMIQMC0_R {
        RFC_MDM_DEMIQMC0_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 6:11 - LDOVCO_TRIM_OUTPUT"]
    #[inline(always)]
    pub fn ldovco_trim_output(&self) -> LDOVCO_TRIM_OUTPUT_R {
        LDOVCO_TRIM_OUTPUT_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5 - SLDO_TRIM_OUTPUT"]
    #[inline(always)]
    pub fn sldo_trim_output(&self) -> SLDO_TRIM_OUTPUT_R {
        SLDO_TRIM_OUTPUT_R::new((self.bits & 0x3f) as u8)
    }
}
