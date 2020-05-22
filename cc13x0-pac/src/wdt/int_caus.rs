#[doc = "Reader of register INT_CAUS"]
pub type R = crate::R<u32, super::INT_CAUS>;
#[doc = "Reader of field `CAUSE_RESET`"]
pub type CAUSE_RESET_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAUSE_INTR`"]
pub type CAUSE_INTR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - CAUSE_RESET"]
    #[inline(always)]
    pub fn cause_reset(&self) -> CAUSE_RESET_R {
        CAUSE_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CAUSE_INTR"]
    #[inline(always)]
    pub fn cause_intr(&self) -> CAUSE_INTR_R {
        CAUSE_INTR_R::new((self.bits & 0x01) != 0)
    }
}
