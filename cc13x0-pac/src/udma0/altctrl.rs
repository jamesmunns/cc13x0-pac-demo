#[doc = "Reader of register ALTCTRL"]
pub type R = crate::R<u32, super::ALTCTRL>;
#[doc = "Reader of field `BASEPTR`"]
pub type BASEPTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - BASEPTR"]
    #[inline(always)]
    pub fn baseptr(&self) -> BASEPTR_R {
        BASEPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
