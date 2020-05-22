#[doc = "Reader of register FSM_STAT"]
pub type R = crate::R<u32, super::FSM_STAT>;
#[doc = "Reader of field `NON_OP`"]
pub type NON_OP_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVR_PUL_CNT`"]
pub type OVR_PUL_CNT_R = crate::R<bool, bool>;
#[doc = "Reader of field `INV_DAT`"]
pub type INV_DAT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - NON_OP"]
    #[inline(always)]
    pub fn non_op(&self) -> NON_OP_R {
        NON_OP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - OVR_PUL_CNT"]
    #[inline(always)]
    pub fn ovr_pul_cnt(&self) -> OVR_PUL_CNT_R {
        OVR_PUL_CNT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - INV_DAT"]
    #[inline(always)]
    pub fn inv_dat(&self) -> INV_DAT_R {
        INV_DAT_R::new((self.bits & 0x01) != 0)
    }
}
