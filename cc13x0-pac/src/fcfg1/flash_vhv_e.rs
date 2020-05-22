#[doc = "Reader of register FLASH_VHV_E"]
pub type R = crate::R<u32, super::FLASH_VHV_E>;
#[doc = "Reader of field `VHV_E_START`"]
pub type VHV_E_START_R = crate::R<u16, u16>;
#[doc = "Reader of field `VHV_E_STEP_HIGHT`"]
pub type VHV_E_STEP_HIGHT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - VHV_E_START"]
    #[inline(always)]
    pub fn vhv_e_start(&self) -> VHV_E_START_R {
        VHV_E_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - VHV_E_STEP_HIGHT"]
    #[inline(always)]
    pub fn vhv_e_step_hight(&self) -> VHV_E_STEP_HIGHT_R {
        VHV_E_STEP_HIGHT_R::new((self.bits & 0xffff) as u16)
    }
}
