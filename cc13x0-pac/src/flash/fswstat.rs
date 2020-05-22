#[doc = "Reader of register FSWSTAT"]
pub type R = crate::R<u32, super::FSWSTAT>;
#[doc = "Reader of field `SAFELV`"]
pub type SAFELV_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SAFELV"]
    #[inline(always)]
    pub fn safelv(&self) -> SAFELV_R {
        SAFELV_R::new((self.bits & 0x01) != 0)
    }
}
