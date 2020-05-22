#[doc = "Reader of register FSM_SAV_PPUL"]
pub type R = crate::R<u32, super::FSM_SAV_PPUL>;
#[doc = "Reader of field `SAV_P_PUL`"]
pub type SAV_P_PUL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - SAV_P_PUL"]
    #[inline(always)]
    pub fn sav_p_pul(&self) -> SAV_P_PUL_R {
        SAV_P_PUL_R::new((self.bits & 0x0fff) as u16)
    }
}
