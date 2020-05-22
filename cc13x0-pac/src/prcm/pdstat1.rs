#[doc = "Reader of register PDSTAT1"]
pub type R = crate::R<u32, super::PDSTAT1>;
#[doc = "Reader of field `BUS_ON`"]
pub type BUS_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `VIMS_MODE`"]
pub type VIMS_MODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFC_ON`"]
pub type RFC_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `CPU_ON`"]
pub type CPU_ON_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - BUS_ON"]
    #[inline(always)]
    pub fn bus_on(&self) -> BUS_ON_R {
        BUS_ON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VIMS_MODE"]
    #[inline(always)]
    pub fn vims_mode(&self) -> VIMS_MODE_R {
        VIMS_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RFC_ON"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RFC_ON_R {
        RFC_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU_ON"]
    #[inline(always)]
    pub fn cpu_on(&self) -> CPU_ON_R {
        CPU_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
