#[doc = "Reader of register AESTAGOUT%s"]
pub type R = crate::R<u32, super::AESTAGOUT>;
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TAG"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
