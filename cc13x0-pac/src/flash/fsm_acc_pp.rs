#[doc = "Reader of register FSM_ACC_PP"]
pub type R = crate::R<u32, super::FSM_ACC_PP>;
#[doc = "Reader of field `FSM_ACC_PP`"]
pub type FSM_ACC_PP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - FSM_ACC_PP"]
    #[inline(always)]
    pub fn fsm_acc_pp(&self) -> FSM_ACC_PP_R {
        FSM_ACC_PP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
