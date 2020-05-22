#[doc = "Reader of register FLASH_ERA_PW"]
pub type R = crate::R<u32, super::FLASH_ERA_PW>;
#[doc = "Reader of field `ERASE_PW`"]
pub type ERASE_PW_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ERASE_PW"]
    #[inline(always)]
    pub fn erase_pw(&self) -> ERASE_PW_R {
        ERASE_PW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
