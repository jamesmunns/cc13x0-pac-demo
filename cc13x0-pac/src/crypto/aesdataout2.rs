#[doc = "Reader of register AESDATAOUT2"]
pub type R = crate::R<u32, super::AESDATAOUT2>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
