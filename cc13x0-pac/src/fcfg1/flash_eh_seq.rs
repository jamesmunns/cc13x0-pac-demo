#[doc = "Reader of register FLASH_EH_SEQ"]
pub type R = crate::R<u32, super::FLASH_EH_SEQ>;
#[doc = "Reader of field `EH`"]
pub type EH_R = crate::R<u8, u8>;
#[doc = "Reader of field `SEQ`"]
pub type SEQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `VSTAT`"]
pub type VSTAT_R = crate::R<u8, u8>;
#[doc = "Reader of field `SM_FREQUENCY`"]
pub type SM_FREQUENCY_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 24:31 - EH"]
    #[inline(always)]
    pub fn eh(&self) -> EH_R {
        EH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SEQ"]
    #[inline(always)]
    pub fn seq(&self) -> SEQ_R {
        SEQ_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - VSTAT"]
    #[inline(always)]
    pub fn vstat(&self) -> VSTAT_R {
        VSTAT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:11 - SM_FREQUENCY"]
    #[inline(always)]
    pub fn sm_frequency(&self) -> SM_FREQUENCY_R {
        SM_FREQUENCY_R::new((self.bits & 0x0fff) as u16)
    }
}
