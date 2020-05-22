#[doc = "Reader of register BL_CONFIG"]
pub type R = crate::R<u32, super::BL_CONFIG>;
#[doc = "Reader of field `BOOTLOADER_ENABLE`"]
pub type BOOTLOADER_ENABLE_R = crate::R<u8, u8>;
#[doc = "Reader of field `BL_LEVEL`"]
pub type BL_LEVEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `BL_PIN_NUMBER`"]
pub type BL_PIN_NUMBER_R = crate::R<u8, u8>;
#[doc = "Reader of field `BL_ENABLE`"]
pub type BL_ENABLE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - BOOTLOADER_ENABLE"]
    #[inline(always)]
    pub fn bootloader_enable(&self) -> BOOTLOADER_ENABLE_R {
        BOOTLOADER_ENABLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 16 - BL_LEVEL"]
    #[inline(always)]
    pub fn bl_level(&self) -> BL_LEVEL_R {
        BL_LEVEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - BL_PIN_NUMBER"]
    #[inline(always)]
    pub fn bl_pin_number(&self) -> BL_PIN_NUMBER_R {
        BL_PIN_NUMBER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BL_ENABLE"]
    #[inline(always)]
    pub fn bl_enable(&self) -> BL_ENABLE_R {
        BL_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
