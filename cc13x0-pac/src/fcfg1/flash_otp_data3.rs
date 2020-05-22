#[doc = "Reader of register FLASH_OTP_DATA3"]
pub type R = crate::R<u32, super::FLASH_OTP_DATA3>;
#[doc = "Reader of field `EC_STEP_SIZE`"]
pub type EC_STEP_SIZE_R = crate::R<u16, u16>;
#[doc = "Reader of field `DO_PRECOND`"]
pub type DO_PRECOND_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAX_EC_LEVEL`"]
pub type MAX_EC_LEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIM_1P7`"]
pub type TRIM_1P7_R = crate::R<u8, u8>;
#[doc = "Reader of field `FLASH_SIZE`"]
pub type FLASH_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `WAIT_SYSCODE`"]
pub type WAIT_SYSCODE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 23:31 - EC_STEP_SIZE"]
    #[inline(always)]
    pub fn ec_step_size(&self) -> EC_STEP_SIZE_R {
        EC_STEP_SIZE_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bit 22 - DO_PRECOND"]
    #[inline(always)]
    pub fn do_precond(&self) -> DO_PRECOND_R {
        DO_PRECOND_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 18:21 - MAX_EC_LEVEL"]
    #[inline(always)]
    pub fn max_ec_level(&self) -> MAX_EC_LEVEL_R {
        MAX_EC_LEVEL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - TRIM_1P7"]
    #[inline(always)]
    pub fn trim_1p7(&self) -> TRIM_1P7_R {
        TRIM_1P7_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - FLASH_SIZE"]
    #[inline(always)]
    pub fn flash_size(&self) -> FLASH_SIZE_R {
        FLASH_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - WAIT_SYSCODE"]
    #[inline(always)]
    pub fn wait_syscode(&self) -> WAIT_SYSCODE_R {
        WAIT_SYSCODE_R::new((self.bits & 0xff) as u8)
    }
}
