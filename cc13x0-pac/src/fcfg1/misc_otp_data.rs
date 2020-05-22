#[doc = "Reader of register MISC_OTP_DATA"]
pub type R = crate::R<u32, super::MISC_OTP_DATA>;
#[doc = "Reader of field `RCOSC_HF_ITUNE`"]
pub type RCOSC_HF_ITUNE_R = crate::R<u8, u8>;
#[doc = "Reader of field `RCOSC_HF_CRIM`"]
pub type RCOSC_HF_CRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `PER_M`"]
pub type PER_M_R = crate::R<u8, u8>;
#[doc = "Reader of field `PER_E`"]
pub type PER_E_R = crate::R<u8, u8>;
#[doc = "Reader of field `PO_TAIL_RES_TRIM`"]
pub type PO_TAIL_RES_TRIM_R = crate::R<u8, u8>;
#[doc = "Reader of field `TEST_PROGRAM_REV`"]
pub type TEST_PROGRAM_REV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 28:31 - RCOSC_HF_ITUNE"]
    #[inline(always)]
    pub fn rcosc_hf_itune(&self) -> RCOSC_HF_ITUNE_R {
        RCOSC_HF_ITUNE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - RCOSC_HF_CRIM"]
    #[inline(always)]
    pub fn rcosc_hf_crim(&self) -> RCOSC_HF_CRIM_R {
        RCOSC_HF_CRIM_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 15:19 - PER_M"]
    #[inline(always)]
    pub fn per_m(&self) -> PER_M_R {
        PER_M_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - PER_E"]
    #[inline(always)]
    pub fn per_e(&self) -> PER_E_R {
        PER_E_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - PO_TAIL_RES_TRIM"]
    #[inline(always)]
    pub fn po_tail_res_trim(&self) -> PO_TAIL_RES_TRIM_R {
        PO_TAIL_RES_TRIM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - TEST_PROGRAM_REV"]
    #[inline(always)]
    pub fn test_program_rev(&self) -> TEST_PROGRAM_REV_R {
        TEST_PROGRAM_REV_R::new((self.bits & 0xff) as u8)
    }
}
