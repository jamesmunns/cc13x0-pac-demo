#[doc = "Reader of register FLASH_VHV"]
pub type R = crate::R<u32, super::FLASH_VHV>;
#[doc = "Reader of field `TRIM13_P`"]
pub type TRIM13_P_R = crate::R<u8, u8>;
#[doc = "Reader of field `VHV_P`"]
pub type VHV_P_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIM13_E`"]
pub type TRIM13_E_R = crate::R<u8, u8>;
#[doc = "Reader of field `VHV_E`"]
pub type VHV_E_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:27 - TRIM13_P"]
    #[inline(always)]
    pub fn trim13_p(&self) -> TRIM13_P_R {
        TRIM13_P_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - VHV_P"]
    #[inline(always)]
    pub fn vhv_p(&self) -> VHV_P_R {
        VHV_P_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TRIM13_E"]
    #[inline(always)]
    pub fn trim13_e(&self) -> TRIM13_E_R {
        TRIM13_E_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - VHV_E"]
    #[inline(always)]
    pub fn vhv_e(&self) -> VHV_E_R {
        VHV_E_R::new((self.bits & 0x0f) as u8)
    }
}
