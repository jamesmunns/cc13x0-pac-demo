#[doc = "Reader of register FSM_TIMER"]
pub type R = crate::R<u32, super::FSM_TIMER>;
#[doc = "Reader of field `FSM_TIMER`"]
pub type FSM_TIMER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - FSM_TIMER"]
    #[inline(always)]
    pub fn fsm_timer(&self) -> FSM_TIMER_R {
        FSM_TIMER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
