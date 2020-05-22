#[doc = "Reader of register FCFG_BANK"]
pub type R = crate::R<u32, super::FCFG_BANK>;
#[doc = "Reader of field `EE_BANK_WIDTH`"]
pub type EE_BANK_WIDTH_R = crate::R<u16, u16>;
#[doc = "Reader of field `EE_NUM_BANK`"]
pub type EE_NUM_BANK_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAIN_BANK_WIDTH`"]
pub type MAIN_BANK_WIDTH_R = crate::R<u16, u16>;
#[doc = "Reader of field `MAIN_NUM_BANK`"]
pub type MAIN_NUM_BANK_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 20:31 - EE_BANK_WIDTH"]
    #[inline(always)]
    pub fn ee_bank_width(&self) -> EE_BANK_WIDTH_R {
        EE_BANK_WIDTH_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - EE_NUM_BANK"]
    #[inline(always)]
    pub fn ee_num_bank(&self) -> EE_NUM_BANK_R {
        EE_NUM_BANK_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - MAIN_BANK_WIDTH"]
    #[inline(always)]
    pub fn main_bank_width(&self) -> MAIN_BANK_WIDTH_R {
        MAIN_BANK_WIDTH_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - MAIN_NUM_BANK"]
    #[inline(always)]
    pub fn main_num_bank(&self) -> MAIN_NUM_BANK_R {
        MAIN_NUM_BANK_R::new((self.bits & 0x0f) as u8)
    }
}
