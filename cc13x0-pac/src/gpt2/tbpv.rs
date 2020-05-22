#[doc = "Reader of register TBPV"]
pub type R = crate::R<u32, super::TBPV>;
#[doc = "Reader of field `PSV`"]
pub type PSV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PSV"]
    #[inline(always)]
    pub fn psv(&self) -> PSV_R {
        PSV_R::new((self.bits & 0xff) as u8)
    }
}
