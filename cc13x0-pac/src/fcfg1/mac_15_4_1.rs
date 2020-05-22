#[doc = "Reader of register MAC_15_4_1"]
pub type R = crate::R<u32, super::MAC_15_4_1>;
#[doc = "Reader of field `ADDR_32_63`"]
pub type ADDR_32_63_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADDR_32_63"]
    #[inline(always)]
    pub fn addr_32_63(&self) -> ADDR_32_63_R {
        ADDR_32_63_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
