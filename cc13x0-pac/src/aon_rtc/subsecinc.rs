#[doc = "Reader of register SUBSECINC"]
pub type R = crate::R<u32, super::SUBSECINC>;
#[doc = "Reader of field `VALUEINC`"]
pub type VALUEINC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - VALUEINC"]
    #[inline(always)]
    pub fn valueinc(&self) -> VALUEINC_R {
        VALUEINC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
