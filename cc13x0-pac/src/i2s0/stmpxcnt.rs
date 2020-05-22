#[doc = "Reader of register STMPXCNT"]
pub type R = crate::R<u32, super::STMPXCNT>;
#[doc = "Reader of field `CURR_VALUE`"]
pub type CURR_VALUE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - CURR_VALUE"]
    #[inline(always)]
    pub fn curr_value(&self) -> CURR_VALUE_R {
        CURR_VALUE_R::new((self.bits & 0xffff) as u16)
    }
}
