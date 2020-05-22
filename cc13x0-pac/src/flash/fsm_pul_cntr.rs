#[doc = "Reader of register FSM_PUL_CNTR"]
pub type R = crate::R<u32, super::FSM_PUL_CNTR>;
#[doc = "Reader of field `CUR_EC_LEVEL`"]
pub type CUR_EC_LEVEL_R = crate::R<u16, u16>;
#[doc = "Reader of field `PUL_CNTR`"]
pub type PUL_CNTR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:24 - CUR_EC_LEVEL"]
    #[inline(always)]
    pub fn cur_ec_level(&self) -> CUR_EC_LEVEL_R {
        CUR_EC_LEVEL_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:11 - PUL_CNTR"]
    #[inline(always)]
    pub fn pul_cntr(&self) -> PUL_CNTR_R {
        PUL_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
}
