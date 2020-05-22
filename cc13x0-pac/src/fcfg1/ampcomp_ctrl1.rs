#[doc = "Reader of register AMPCOMP_CTRL1"]
pub type R = crate::R<u32, super::AMPCOMP_CTRL1>;
#[doc = "Reader of field `AMPCOMP_REQ_MODE`"]
pub type AMPCOMP_REQ_MODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `IBIAS_OFFSET`"]
pub type IBIAS_OFFSET_R = crate::R<u8, u8>;
#[doc = "Reader of field `IBIAS_INIT`"]
pub type IBIAS_INIT_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPM_IBIAS_WAIT_CNT_FINAL`"]
pub type LPM_IBIAS_WAIT_CNT_FINAL_R = crate::R<u8, u8>;
#[doc = "Reader of field `CAP_STEP`"]
pub type CAP_STEP_R = crate::R<u8, u8>;
#[doc = "Reader of field `IBIASCAP_HPTOLP_OL_CNT`"]
pub type IBIASCAP_HPTOLP_OL_CNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 30 - AMPCOMP_REQ_MODE"]
    #[inline(always)]
    pub fn ampcomp_req_mode(&self) -> AMPCOMP_REQ_MODE_R {
        AMPCOMP_REQ_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - IBIAS_OFFSET"]
    #[inline(always)]
    pub fn ibias_offset(&self) -> IBIAS_OFFSET_R {
        IBIAS_OFFSET_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - IBIAS_INIT"]
    #[inline(always)]
    pub fn ibias_init(&self) -> IBIAS_INIT_R {
        IBIAS_INIT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - LPM_IBIAS_WAIT_CNT_FINAL"]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&self) -> LPM_IBIAS_WAIT_CNT_FINAL_R {
        LPM_IBIAS_WAIT_CNT_FINAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - CAP_STEP"]
    #[inline(always)]
    pub fn cap_step(&self) -> CAP_STEP_R {
        CAP_STEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - IBIASCAP_HPTOLP_OL_CNT"]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&self) -> IBIASCAP_HPTOLP_OL_CNT_R {
        IBIASCAP_HPTOLP_OL_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
