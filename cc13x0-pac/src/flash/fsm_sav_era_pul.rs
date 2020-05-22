#[doc = "Reader of register FSM_SAV_ERA_PUL"]
pub type R = crate::R<u32, super::FSM_SAV_ERA_PUL>;
#[doc = "Reader of field `SAV_ERA_PUL`"]
pub type SAV_ERA_PUL_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - SAV_ERA_PUL"]
    #[inline(always)]
    pub fn sav_era_pul(&self) -> SAV_ERA_PUL_R {
        SAV_ERA_PUL_R::new((self.bits & 0x0fff) as u16)
    }
}
