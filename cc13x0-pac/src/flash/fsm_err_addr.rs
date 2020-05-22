#[doc = "Reader of register FSM_ERR_ADDR"]
pub type R = crate::R<u32, super::FSM_ERR_ADDR>;
#[doc = "Reader of field `FSM_ERR_ADDR`"]
pub type FSM_ERR_ADDR_R = crate::R<u32, u32>;
#[doc = "Reader of field `FSM_ERR_BANK`"]
pub type FSM_ERR_BANK_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:31 - FSM_ERR_ADDR"]
    #[inline(always)]
    pub fn fsm_err_addr(&self) -> FSM_ERR_ADDR_R {
        FSM_ERR_ADDR_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - FSM_ERR_BANK"]
    #[inline(always)]
    pub fn fsm_err_bank(&self) -> FSM_ERR_BANK_R {
        FSM_ERR_BANK_R::new((self.bits & 0x0f) as u8)
    }
}
