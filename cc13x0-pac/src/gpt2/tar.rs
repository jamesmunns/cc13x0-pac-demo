#[doc = "Reader of register TAR"]
pub type R = crate::R<u32, super::TAR>;
#[doc = "Reader of field `TAR`"]
pub type TAR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
