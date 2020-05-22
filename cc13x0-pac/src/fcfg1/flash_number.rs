#[doc = "Reader of register FLASH_NUMBER"]
pub type R = crate::R<u32, super::FLASH_NUMBER>;
#[doc = "Reader of field `LOT_NUMBER`"]
pub type LOT_NUMBER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - LOT_NUMBER"]
    #[inline(always)]
    pub fn lot_number(&self) -> LOT_NUMBER_R {
        LOT_NUMBER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
