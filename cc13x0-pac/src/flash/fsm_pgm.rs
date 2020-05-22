#[doc = "Reader of register FSM_PGM"]
pub type R = crate::R<u32, super::FSM_PGM>;
#[doc = "Reader of field `PGM_BANK`"]
pub type PGM_BANK_R = crate::R<u8, u8>;
#[doc = "Reader of field `PGM_ADDR`"]
pub type PGM_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 23:25 - PGM_BANK"]
    #[inline(always)]
    pub fn pgm_bank(&self) -> PGM_BANK_R {
        PGM_BANK_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 0:22 - PGM_ADDR"]
    #[inline(always)]
    pub fn pgm_addr(&self) -> PGM_ADDR_R {
        PGM_ADDR_R::new((self.bits & 0x007f_ffff) as u32)
    }
}
