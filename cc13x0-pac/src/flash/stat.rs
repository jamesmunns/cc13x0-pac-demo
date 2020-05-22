#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `EFUSE_BLANK`"]
pub type EFUSE_BLANK_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_TIMEOUT`"]
pub type EFUSE_TIMEOUT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_CRC_ERROR`"]
pub type EFUSE_CRC_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_ERRCODE`"]
pub type EFUSE_ERRCODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `SAMHOLD_DIS`"]
pub type SAMHOLD_DIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `POWER_MODE`"]
pub type POWER_MODE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 15 - EFUSE_BLANK"]
    #[inline(always)]
    pub fn efuse_blank(&self) -> EFUSE_BLANK_R {
        EFUSE_BLANK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EFUSE_TIMEOUT"]
    #[inline(always)]
    pub fn efuse_timeout(&self) -> EFUSE_TIMEOUT_R {
        EFUSE_TIMEOUT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EFUSE_CRC_ERROR"]
    #[inline(always)]
    pub fn efuse_crc_error(&self) -> EFUSE_CRC_ERROR_R {
        EFUSE_CRC_ERROR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - EFUSE_ERRCODE"]
    #[inline(always)]
    pub fn efuse_errcode(&self) -> EFUSE_ERRCODE_R {
        EFUSE_ERRCODE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 2 - SAMHOLD_DIS"]
    #[inline(always)]
    pub fn samhold_dis(&self) -> SAMHOLD_DIS_R {
        SAMHOLD_DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - POWER_MODE"]
    #[inline(always)]
    pub fn power_mode(&self) -> POWER_MODE_R {
        POWER_MODE_R::new((self.bits & 0x01) != 0)
    }
}
