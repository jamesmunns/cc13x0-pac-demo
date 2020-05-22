#[doc = "Reader of register FLASH_OTP_DATA4"]
pub type R = crate::R<u32, super::FLASH_OTP_DATA4>;
#[doc = "Reader of field `STANDBY_MODE_SEL_INT_WRT`"]
pub type STANDBY_MODE_SEL_INT_WRT_R = crate::R<bool, bool>;
#[doc = "Reader of field `STANDBY_PW_SEL_INT_WRT`"]
pub type STANDBY_PW_SEL_INT_WRT_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIS_STANDBY_INT_WRT`"]
pub type DIS_STANDBY_INT_WRT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_IDLE_INT_WRT`"]
pub type DIS_IDLE_INT_WRT_R = crate::R<bool, bool>;
#[doc = "Reader of field `VIN_AT_X_INT_WRT`"]
pub type VIN_AT_X_INT_WRT_R = crate::R<u8, u8>;
#[doc = "Reader of field `STANDBY_MODE_SEL_EXT_WRT`"]
pub type STANDBY_MODE_SEL_EXT_WRT_R = crate::R<bool, bool>;
#[doc = "Reader of field `STANDBY_PW_SEL_EXT_WRT`"]
pub type STANDBY_PW_SEL_EXT_WRT_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIS_STANDBY_EXT_WRT`"]
pub type DIS_STANDBY_EXT_WRT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_IDLE_EXT_WRT`"]
pub type DIS_IDLE_EXT_WRT_R = crate::R<bool, bool>;
#[doc = "Reader of field `VIN_AT_X_EXT_WRT`"]
pub type VIN_AT_X_EXT_WRT_R = crate::R<u8, u8>;
#[doc = "Reader of field `STANDBY_MODE_SEL_INT_RD`"]
pub type STANDBY_MODE_SEL_INT_RD_R = crate::R<bool, bool>;
#[doc = "Reader of field `STANDBY_PW_SEL_INT_RD`"]
pub type STANDBY_PW_SEL_INT_RD_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIS_STANDBY_INT_RD`"]
pub type DIS_STANDBY_INT_RD_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_IDLE_INT_RD`"]
pub type DIS_IDLE_INT_RD_R = crate::R<bool, bool>;
#[doc = "Reader of field `VIN_AT_X_INT_RD`"]
pub type VIN_AT_X_INT_RD_R = crate::R<u8, u8>;
#[doc = "Reader of field `STANDBY_MODE_SEL_EXT_RD`"]
pub type STANDBY_MODE_SEL_EXT_RD_R = crate::R<bool, bool>;
#[doc = "Reader of field `STANDBY_PW_SEL_EXT_RD`"]
pub type STANDBY_PW_SEL_EXT_RD_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIS_STANDBY_EXT_RD`"]
pub type DIS_STANDBY_EXT_RD_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS_IDLE_EXT_RD`"]
pub type DIS_IDLE_EXT_RD_R = crate::R<bool, bool>;
#[doc = "Reader of field `VIN_AT_X_EXT_RD`"]
pub type VIN_AT_X_EXT_RD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 31 - STANDBY_MODE_SEL_INT_WRT"]
    #[inline(always)]
    pub fn standby_mode_sel_int_wrt(&self) -> STANDBY_MODE_SEL_INT_WRT_R {
        STANDBY_MODE_SEL_INT_WRT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - STANDBY_PW_SEL_INT_WRT"]
    #[inline(always)]
    pub fn standby_pw_sel_int_wrt(&self) -> STANDBY_PW_SEL_INT_WRT_R {
        STANDBY_PW_SEL_INT_WRT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bit 28 - DIS_STANDBY_INT_WRT"]
    #[inline(always)]
    pub fn dis_standby_int_wrt(&self) -> DIS_STANDBY_INT_WRT_R {
        DIS_STANDBY_INT_WRT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DIS_IDLE_INT_WRT"]
    #[inline(always)]
    pub fn dis_idle_int_wrt(&self) -> DIS_IDLE_INT_WRT_R {
        DIS_IDLE_INT_WRT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - VIN_AT_X_INT_WRT"]
    #[inline(always)]
    pub fn vin_at_x_int_wrt(&self) -> VIN_AT_X_INT_WRT_R {
        VIN_AT_X_INT_WRT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23 - STANDBY_MODE_SEL_EXT_WRT"]
    #[inline(always)]
    pub fn standby_mode_sel_ext_wrt(&self) -> STANDBY_MODE_SEL_EXT_WRT_R {
        STANDBY_MODE_SEL_EXT_WRT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - STANDBY_PW_SEL_EXT_WRT"]
    #[inline(always)]
    pub fn standby_pw_sel_ext_wrt(&self) -> STANDBY_PW_SEL_EXT_WRT_R {
        STANDBY_PW_SEL_EXT_WRT_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20 - DIS_STANDBY_EXT_WRT"]
    #[inline(always)]
    pub fn dis_standby_ext_wrt(&self) -> DIS_STANDBY_EXT_WRT_R {
        DIS_STANDBY_EXT_WRT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DIS_IDLE_EXT_WRT"]
    #[inline(always)]
    pub fn dis_idle_ext_wrt(&self) -> DIS_IDLE_EXT_WRT_R {
        DIS_IDLE_EXT_WRT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - VIN_AT_X_EXT_WRT"]
    #[inline(always)]
    pub fn vin_at_x_ext_wrt(&self) -> VIN_AT_X_EXT_WRT_R {
        VIN_AT_X_EXT_WRT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15 - STANDBY_MODE_SEL_INT_RD"]
    #[inline(always)]
    pub fn standby_mode_sel_int_rd(&self) -> STANDBY_MODE_SEL_INT_RD_R {
        STANDBY_MODE_SEL_INT_RD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - STANDBY_PW_SEL_INT_RD"]
    #[inline(always)]
    pub fn standby_pw_sel_int_rd(&self) -> STANDBY_PW_SEL_INT_RD_R {
        STANDBY_PW_SEL_INT_RD_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - DIS_STANDBY_INT_RD"]
    #[inline(always)]
    pub fn dis_standby_int_rd(&self) -> DIS_STANDBY_INT_RD_R {
        DIS_STANDBY_INT_RD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DIS_IDLE_INT_RD"]
    #[inline(always)]
    pub fn dis_idle_int_rd(&self) -> DIS_IDLE_INT_RD_R {
        DIS_IDLE_INT_RD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - VIN_AT_X_INT_RD"]
    #[inline(always)]
    pub fn vin_at_x_int_rd(&self) -> VIN_AT_X_INT_RD_R {
        VIN_AT_X_INT_RD_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - STANDBY_MODE_SEL_EXT_RD"]
    #[inline(always)]
    pub fn standby_mode_sel_ext_rd(&self) -> STANDBY_MODE_SEL_EXT_RD_R {
        STANDBY_MODE_SEL_EXT_RD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - STANDBY_PW_SEL_EXT_RD"]
    #[inline(always)]
    pub fn standby_pw_sel_ext_rd(&self) -> STANDBY_PW_SEL_EXT_RD_R {
        STANDBY_PW_SEL_EXT_RD_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - DIS_STANDBY_EXT_RD"]
    #[inline(always)]
    pub fn dis_standby_ext_rd(&self) -> DIS_STANDBY_EXT_RD_R {
        DIS_STANDBY_EXT_RD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIS_IDLE_EXT_RD"]
    #[inline(always)]
    pub fn dis_idle_ext_rd(&self) -> DIS_IDLE_EXT_RD_R {
        DIS_IDLE_EXT_RD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - VIN_AT_X_EXT_RD"]
    #[inline(always)]
    pub fn vin_at_x_ext_rd(&self) -> VIN_AT_X_EXT_RD_R {
        VIN_AT_X_EXT_RD_R::new((self.bits & 0x07) as u8)
    }
}
