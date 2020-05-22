#[doc = "Reader of register FLASH_E_P"]
pub type R = crate::R<u32, super::FLASH_E_P>;
#[doc = "Reader of field `PSU`"]
pub type PSU_R = crate::R<u8, u8>;
#[doc = "Reader of field `ESU`"]
pub type ESU_R = crate::R<u8, u8>;
#[doc = "Reader of field `PVSU`"]
pub type PVSU_R = crate::R<u8, u8>;
#[doc = "Reader of field `EVSU`"]
pub type EVSU_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - PSU"]
    #[inline(always)]
    pub fn psu(&self) -> PSU_R {
        PSU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ESU"]
    #[inline(always)]
    pub fn esu(&self) -> ESU_R {
        ESU_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PVSU"]
    #[inline(always)]
    pub fn pvsu(&self) -> PVSU_R {
        PVSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - EVSU"]
    #[inline(always)]
    pub fn evsu(&self) -> EVSU_R {
        EVSU_R::new((self.bits & 0xff) as u8)
    }
}
