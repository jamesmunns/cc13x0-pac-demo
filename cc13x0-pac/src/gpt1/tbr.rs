#[doc = "Reader of register TBR"]
pub type R = crate::R<u32, super::TBR>;
#[doc = "Reader of field `TBR`"]
pub type TBR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TBR"]
    #[inline(always)]
    pub fn tbr(&self) -> TBR_R {
        TBR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
