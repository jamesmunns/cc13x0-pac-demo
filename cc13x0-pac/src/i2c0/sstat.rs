#[doc = "Reader of register SSTAT"]
pub type R = crate::R<u32, super::SSTAT>;
#[doc = "Reader of field `FBR`"]
pub type FBR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TREQ`"]
pub type TREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `RREQ`"]
pub type RREQ_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - FBR"]
    #[inline(always)]
    pub fn fbr(&self) -> FBR_R {
        FBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TREQ"]
    #[inline(always)]
    pub fn treq(&self) -> TREQ_R {
        TREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RREQ"]
    #[inline(always)]
    pub fn rreq(&self) -> RREQ_R {
        RREQ_R::new((self.bits & 0x01) != 0)
    }
}
