#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `SAT`"]
pub type SAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 7 - SAT"]
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DONE"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - STATE"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x3f) as u8)
    }
}
