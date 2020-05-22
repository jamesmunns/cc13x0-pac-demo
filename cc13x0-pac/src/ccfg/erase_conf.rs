#[doc = "Reader of register ERASE_CONF"]
pub type R = crate::R<u32, super::ERASE_CONF>;
#[doc = "Reader of field `CHIP_ERASE_DIS_N`"]
pub type CHIP_ERASE_DIS_N_R = crate::R<bool, bool>;
#[doc = "Reader of field `BANK_ERASE_DIS_N`"]
pub type BANK_ERASE_DIS_N_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 8 - CHIP_ERASE_DIS_N"]
    #[inline(always)]
    pub fn chip_erase_dis_n(&self) -> CHIP_ERASE_DIS_N_R {
        CHIP_ERASE_DIS_N_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BANK_ERASE_DIS_N"]
    #[inline(always)]
    pub fn bank_erase_dis_n(&self) -> BANK_ERASE_DIS_N_R {
        BANK_ERASE_DIS_N_R::new((self.bits & 0x01) != 0)
    }
}
