#[doc = "Reader of register CCFG_TAP_DAP_0"]
pub type R = crate::R<u32, super::CCFG_TAP_DAP_0>;
#[doc = "Reader of field `CPU_DAP_ENABLE`"]
pub type CPU_DAP_ENABLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PRCM_TAP_ENABLE`"]
pub type PRCM_TAP_ENABLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `TEST_TAP_ENABLE`"]
pub type TEST_TAP_ENABLE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:23 - CPU_DAP_ENABLE"]
    #[inline(always)]
    pub fn cpu_dap_enable(&self) -> CPU_DAP_ENABLE_R {
        CPU_DAP_ENABLE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PRCM_TAP_ENABLE"]
    #[inline(always)]
    pub fn prcm_tap_enable(&self) -> PRCM_TAP_ENABLE_R {
        PRCM_TAP_ENABLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - TEST_TAP_ENABLE"]
    #[inline(always)]
    pub fn test_tap_enable(&self) -> TEST_TAP_ENABLE_R {
        TEST_TAP_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
