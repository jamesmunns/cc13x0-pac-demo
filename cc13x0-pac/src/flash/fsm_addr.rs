#[doc = "Reader of register FSM_ADDR"]
pub type R = crate::R<u32, super::FSM_ADDR>;
#[doc = "Reader of field `BANK`"]
pub type BANK_R = crate::R<u8, u8>;
#[doc = "Reader of field `CUR_ADDR`"]
pub type CUR_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 28:30 - BANK"]
    #[inline(always)]
    pub fn bank(&self) -> BANK_R {
        BANK_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 0:27 - CUR_ADDR"]
    #[inline(always)]
    pub fn cur_addr(&self) -> CUR_ADDR_R {
        CUR_ADDR_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
