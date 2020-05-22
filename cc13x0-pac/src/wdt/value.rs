#[doc = "Reader of register VALUE"]
pub type R = crate::R<u32, super::VALUE>;
#[doc = "Reader of field `WDTVALUE`"]
pub type WDTVALUE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - WDTVALUE"]
    #[inline(always)]
    pub fn wdtvalue(&self) -> WDTVALUE_R {
        WDTVALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
