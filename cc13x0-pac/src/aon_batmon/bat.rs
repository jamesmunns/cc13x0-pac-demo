#[doc = "Reader of register BAT"]
pub type R = crate::R<u32, super::BAT>;
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<u8, u8>;
#[doc = "Reader of field `FRAC`"]
pub type FRAC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:10 - INT"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:7 - FRAC"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0xff) as u8)
    }
}
