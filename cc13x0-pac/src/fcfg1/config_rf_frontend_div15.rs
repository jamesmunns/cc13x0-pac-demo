#[doc = "Reader of register CONFIG_RF_FRONTEND_DIV15"]
pub type R = crate::R<u32, super::CONFIG_RF_FRONTEND_DIV15>;
#[doc = "Reader of field `IFAMP_IB`"]
pub type IFAMP_IB_R = crate::R<u8, u8>;
#[doc = "Reader of field `LNA_IB`"]
pub type LNA_IB_R = crate::R<u8, u8>;
#[doc = "Reader of field `IFAMP_TRIM`"]
pub type IFAMP_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `CTL_PA0_TRIM`"]
pub type CTL_PA0_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `RFLDO_TRIM_OUTPUT`"]
pub type RFLDO_TRIM_OUTPUT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31 - IFAMP_IB"]
    #[inline(always)]
    pub fn ifamp_ib(&self) -> IFAMP_IB_R {
        IFAMP_IB_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - LNA_IB"]
    #[inline(always)]
    pub fn lna_ib(&self) -> LNA_IB_R {
        LNA_IB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 19:23 - IFAMP_TRIM"]
    #[inline(always)]
    pub fn ifamp_trim(&self) -> IFAMP_TRIM_R {
        IFAMP_TRIM_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - CTL_PA0_TRIM"]
    #[inline(always)]
    pub fn ctl_pa0_trim(&self) -> CTL_PA0_TRIM_R {
        CTL_PA0_TRIM_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 0:6 - RFLDO_TRIM_OUTPUT"]
    #[inline(always)]
    pub fn rfldo_trim_output(&self) -> RFLDO_TRIM_OUTPUT_R {
        RFLDO_TRIM_OUTPUT_R::new((self.bits & 0x7f) as u8)
    }
}
