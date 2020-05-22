#[doc = "Reader of register STMPWCNTCAPT0"]
pub type R = crate::R<u32, super::STMPWCNTCAPT0>;
#[doc = "Reader of field `CAPT_VALUE`"]
pub type CAPT_VALUE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CAPT_VALUE"]
    #[inline(always)]
    pub fn capt_value(&self) -> CAPT_VALUE_R {
        CAPT_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
