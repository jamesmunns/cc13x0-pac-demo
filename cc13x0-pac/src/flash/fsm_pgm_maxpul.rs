#[doc = "Reader of register FSM_PGM_MAXPUL"]
pub type R = crate::R<u32, super::FSM_PGM_MAXPUL>;
#[doc = "Reader of field `FSM_PGM_MAXPUL`"]
pub type FSM_PGM_MAXPUL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - FSM_PGM_MAXPUL"]
    #[inline(always)]
    pub fn fsm_pgm_maxpul(&self) -> FSM_PGM_MAXPUL_R {
        FSM_PGM_MAXPUL_R::new((self.bits & 0x0fff) as u16)
    }
}
