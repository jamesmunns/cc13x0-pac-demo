#[doc = "Reader of register OUT1"]
pub type R = crate::R<u32, super::OUT1>;
#[doc = "Reader of field `VALUE_63_32`"]
pub type VALUE_63_32_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - VALUE_63_32"]
    #[inline(always)]
    pub fn value_63_32(&self) -> VALUE_63_32_R {
        VALUE_63_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
