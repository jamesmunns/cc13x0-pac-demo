#[doc = "Reader of register CAP_TRIM"]
pub type R = crate::R<u32, super::CAP_TRIM>;
#[doc = "Reader of field `FLUX_CAP_0P28_TRIM`"]
pub type FLUX_CAP_0P28_TRIM_R = crate::R<u16, u16>;
#[doc = "Reader of field `FLUX_CAP_0P4_TRIM`"]
pub type FLUX_CAP_0P4_TRIM_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - FLUX_CAP_0P28_TRIM"]
    #[inline(always)]
    pub fn flux_cap_0p28_trim(&self) -> FLUX_CAP_0P28_TRIM_R {
        FLUX_CAP_0P28_TRIM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - FLUX_CAP_0P4_TRIM"]
    #[inline(always)]
    pub fn flux_cap_0p4_trim(&self) -> FLUX_CAP_0P4_TRIM_R {
        FLUX_CAP_0P4_TRIM_R::new((self.bits & 0xffff) as u16)
    }
}
