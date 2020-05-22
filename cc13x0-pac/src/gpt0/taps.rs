#[doc = "Reader of register TAPS"]
pub type R = crate::R<u32, super::TAPS>;
#[doc = "Reader of field `PSS`"]
pub type PSS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PSS"]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new((self.bits & 0xff) as u8)
    }
}
