#[doc = "Reader of register FLASH_VHV_PV"]
pub type R = crate::R<u32, super::FLASH_VHV_PV>;
#[doc = "Reader of field `TRIM13_PV`"]
pub type TRIM13_PV_R = crate::R<u8, u8>;
#[doc = "Reader of field `VHV_PV`"]
pub type VHV_PV_R = crate::R<u8, u8>;
#[doc = "Reader of field `VCG2P5`"]
pub type VCG2P5_R = crate::R<u8, u8>;
#[doc = "Reader of field `VINH`"]
pub type VINH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:27 - TRIM13_PV"]
    #[inline(always)]
    pub fn trim13_pv(&self) -> TRIM13_PV_R {
        TRIM13_PV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - VHV_PV"]
    #[inline(always)]
    pub fn vhv_pv(&self) -> VHV_PV_R {
        VHV_PV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - VCG2P5"]
    #[inline(always)]
    pub fn vcg2p5(&self) -> VCG2P5_R {
        VCG2P5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - VINH"]
    #[inline(always)]
    pub fn vinh(&self) -> VINH_R {
        VINH_R::new((self.bits & 0xff) as u8)
    }
}
