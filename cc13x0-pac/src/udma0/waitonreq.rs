#[doc = "Reader of register WAITONREQ"]
pub type R = crate::R<u32, super::WAITONREQ>;
#[doc = "Reader of field `CHNLSTATUS`"]
pub type CHNLSTATUS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CHNLSTATUS"]
    #[inline(always)]
    pub fn chnlstatus(&self) -> CHNLSTATUS_R {
        CHNLSTATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
