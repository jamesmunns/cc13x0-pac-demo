#[doc = "Reader of register HWOPT"]
pub type R = crate::R<u32, super::HWOPT>;
#[doc = "Reader of field `NR_OF_FROS`"]
pub type NR_OF_FROS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 6:11 - NR_OF_FROS"]
    #[inline(always)]
    pub fn nr_of_fros(&self) -> NR_OF_FROS_R {
        NR_OF_FROS_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
