#[doc = "Reader of register FLASH_PP"]
pub type R = crate::R<u32, super::FLASH_PP>;
#[doc = "Reader of field `PUMP_SU`"]
pub type PUMP_SU_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAX_PP`"]
pub type MAX_PP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 24:31 - PUMP_SU"]
    #[inline(always)]
    pub fn pump_su(&self) -> PUMP_SU_R {
        PUMP_SU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - MAX_PP"]
    #[inline(always)]
    pub fn max_pp(&self) -> MAX_PP_R {
        MAX_PP_R::new((self.bits & 0xffff) as u16)
    }
}
