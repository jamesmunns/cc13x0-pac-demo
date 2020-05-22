#[doc = "Reader of register FLASH_C_E_P_R"]
pub type R = crate::R<u32, super::FLASH_C_E_P_R>;
#[doc = "Reader of field `RVSU`"]
pub type RVSU_R = crate::R<u8, u8>;
#[doc = "Reader of field `PV_ACCESS`"]
pub type PV_ACCESS_R = crate::R<u8, u8>;
#[doc = "Reader of field `A_EXEZ_SETUP`"]
pub type A_EXEZ_SETUP_R = crate::R<u8, u8>;
#[doc = "Reader of field `CVSU`"]
pub type CVSU_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 24:31 - RVSU"]
    #[inline(always)]
    pub fn rvsu(&self) -> RVSU_R {
        RVSU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PV_ACCESS"]
    #[inline(always)]
    pub fn pv_access(&self) -> PV_ACCESS_R {
        PV_ACCESS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - A_EXEZ_SETUP"]
    #[inline(always)]
    pub fn a_exez_setup(&self) -> A_EXEZ_SETUP_R {
        A_EXEZ_SETUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - CVSU"]
    #[inline(always)]
    pub fn cvsu(&self) -> CVSU_R {
        CVSU_R::new((self.bits & 0x0fff) as u16)
    }
}
