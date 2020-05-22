#[doc = "Reader of register TEMP"]
pub type R = crate::R<u32, super::TEMP>;
#[doc = "Reader of field `INT`"]
pub type INT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 8:16 - INT"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
