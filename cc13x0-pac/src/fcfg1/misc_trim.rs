#[doc = "Reader of register MISC_TRIM"]
pub type R = crate::R<u32, super::MISC_TRIM>;
#[doc = "Reader of field `TEMPVSLOPE`"]
pub type TEMPVSLOPE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - TEMPVSLOPE"]
    #[inline(always)]
    pub fn tempvslope(&self) -> TEMPVSLOPE_R {
        TEMPVSLOPE_R::new((self.bits & 0xff) as u8)
    }
}
