#[doc = "Reader of register CH1CAPT"]
pub type R = crate::R<u32, super::CH1CAPT>;
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u16, u16>;
#[doc = "Reader of field `SUBSEC`"]
pub type SUBSEC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - SEC"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - SUBSEC"]
    #[inline(always)]
    pub fn subsec(&self) -> SUBSEC_R {
        SUBSEC_R::new((self.bits & 0xffff) as u16)
    }
}
