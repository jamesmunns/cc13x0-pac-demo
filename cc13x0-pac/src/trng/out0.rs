#[doc = "Reader of register OUT0"]
pub type R = crate::R<u32, super::OUT0>;
#[doc = "Reader of field `VALUE_31_0`"]
pub type VALUE_31_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - VALUE_31_0"]
    #[inline(always)]
    pub fn value_31_0(&self) -> VALUE_31_0_R {
        VALUE_31_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
