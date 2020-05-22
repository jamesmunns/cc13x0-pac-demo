#[doc = "Reader of register FLASH_P_R_PV"]
pub type R = crate::R<u32, super::FLASH_P_R_PV>;
#[doc = "Reader of field `PH`"]
pub type PH_R = crate::R<u8, u8>;
#[doc = "Reader of field `RH`"]
pub type RH_R = crate::R<u8, u8>;
#[doc = "Reader of field `PVH`"]
pub type PVH_R = crate::R<u8, u8>;
#[doc = "Reader of field `PVH2`"]
pub type PVH2_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - PH"]
    #[inline(always)]
    pub fn ph(&self) -> PH_R {
        PH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RH"]
    #[inline(always)]
    pub fn rh(&self) -> RH_R {
        RH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PVH"]
    #[inline(always)]
    pub fn pvh(&self) -> PVH_R {
        PVH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - PVH2"]
    #[inline(always)]
    pub fn pvh2(&self) -> PVH2_R {
        PVH2_R::new((self.bits & 0xff) as u8)
    }
}
