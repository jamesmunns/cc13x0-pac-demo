#[doc = "Reader of register FSM_GLBCTL"]
pub type R = crate::R<u32, super::FSM_GLBCTL>;
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CLKSEL"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0x01) != 0)
    }
}
