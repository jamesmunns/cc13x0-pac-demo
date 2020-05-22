#[doc = "Reader of register RSR"]
pub type R = crate::R<u32, super::RSR>;
#[doc = "Reader of field `OE`"]
pub type OE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BE`"]
pub type BE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FE`"]
pub type FE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - OE"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BE"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - FE"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new((self.bits & 0x01) != 0)
    }
}
