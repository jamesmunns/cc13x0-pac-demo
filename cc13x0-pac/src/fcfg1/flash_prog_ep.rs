#[doc = "Reader of register FLASH_PROG_EP"]
pub type R = crate::R<u32, super::FLASH_PROG_EP>;
#[doc = "Reader of field `MAX_EP`"]
pub type MAX_EP_R = crate::R<u16, u16>;
#[doc = "Reader of field `PROGRAM_PW`"]
pub type PROGRAM_PW_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - MAX_EP"]
    #[inline(always)]
    pub fn max_ep(&self) -> MAX_EP_R {
        MAX_EP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - PROGRAM_PW"]
    #[inline(always)]
    pub fn program_pw(&self) -> PROGRAM_PW_R {
        PROGRAM_PW_R::new((self.bits & 0xffff) as u16)
    }
}
