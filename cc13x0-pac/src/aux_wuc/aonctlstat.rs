#[doc = "Reader of register AONCTLSTAT"]
pub type R = crate::R<u32, super::AONCTLSTAT>;
#[doc = "Reader of field `AUX_FORCE_ON`"]
pub type AUX_FORCE_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCE_RUN_EN`"]
pub type SCE_RUN_EN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - AUX_FORCE_ON"]
    #[inline(always)]
    pub fn aux_force_on(&self) -> AUX_FORCE_ON_R {
        AUX_FORCE_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SCE_RUN_EN"]
    #[inline(always)]
    pub fn sce_run_en(&self) -> SCE_RUN_EN_R {
        SCE_RUN_EN_R::new((self.bits & 0x01) != 0)
    }
}
