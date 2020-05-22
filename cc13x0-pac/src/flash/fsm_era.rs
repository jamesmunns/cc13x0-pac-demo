#[doc = "Reader of register FSM_ERA"]
pub type R = crate::R<u32, super::FSM_ERA>;
#[doc = "Reader of field `ERA_BANK`"]
pub type ERA_BANK_R = crate::R<u8, u8>;
#[doc = "Reader of field `ERA_ADDR`"]
pub type ERA_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 23:25 - ERA_BANK"]
    #[inline(always)]
    pub fn era_bank(&self) -> ERA_BANK_R {
        ERA_BANK_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 0:22 - ERA_ADDR"]
    #[inline(always)]
    pub fn era_addr(&self) -> ERA_ADDR_R {
        ERA_ADDR_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
