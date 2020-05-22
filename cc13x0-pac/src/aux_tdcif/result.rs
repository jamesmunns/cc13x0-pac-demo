#[doc = "Reader of register RESULT"]
pub type R = crate::R<u32, super::RESULT>;
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:24 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
