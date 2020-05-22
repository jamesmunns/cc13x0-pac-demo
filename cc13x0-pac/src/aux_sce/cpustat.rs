#[doc = "Reader of register CPUSTAT"]
pub type R = crate::R<u32, super::CPUSTAT>;
#[doc = "Reader of field `BUS_ERROR`"]
pub type BUS_ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, bool>;
#[doc = "Reader of field `WEV`"]
pub type WEV_R = crate::R<bool, bool>;
#[doc = "Reader of field `SELF_STOP`"]
pub type SELF_STOP_R = crate::R<bool, bool>;
#[doc = "Reader of field `V_FLAG`"]
pub type V_FLAG_R = crate::R<bool, bool>;
#[doc = "Reader of field `C_FLAG`"]
pub type C_FLAG_R = crate::R<bool, bool>;
#[doc = "Reader of field `N_FLAG`"]
pub type N_FLAG_R = crate::R<bool, bool>;
#[doc = "Reader of field `Z_FLAG`"]
pub type Z_FLAG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 11 - BUS_ERROR"]
    #[inline(always)]
    pub fn bus_error(&self) -> BUS_ERROR_R {
        BUS_ERROR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SLEEP"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WEV"]
    #[inline(always)]
    pub fn wev(&self) -> WEV_R {
        WEV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SELF_STOP"]
    #[inline(always)]
    pub fn self_stop(&self) -> SELF_STOP_R {
        SELF_STOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3 - V_FLAG"]
    #[inline(always)]
    pub fn v_flag(&self) -> V_FLAG_R {
        V_FLAG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - C_FLAG"]
    #[inline(always)]
    pub fn c_flag(&self) -> C_FLAG_R {
        C_FLAG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - N_FLAG"]
    #[inline(always)]
    pub fn n_flag(&self) -> N_FLAG_R {
        N_FLAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Z_FLAG"]
    #[inline(always)]
    pub fn z_flag(&self) -> Z_FLAG_R {
        Z_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
