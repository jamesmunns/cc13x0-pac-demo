#[doc = "Reader of register FSM_STATE"]
pub type R = crate::R<u32, super::FSM_STATE>;
#[doc = "Reader of field `CTRLENZ`"]
pub type CTRLENZ_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXECUTEZ`"]
pub type EXECUTEZ_R = crate::R<bool, bool>;
#[doc = "Reader of field `FSM_ACT`"]
pub type FSM_ACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIOTP_ACT`"]
pub type TIOTP_ACT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OTP_ACT`"]
pub type OTP_ACT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 11 - CTRLENZ"]
    #[inline(always)]
    pub fn ctrlenz(&self) -> CTRLENZ_R {
        CTRLENZ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EXECUTEZ"]
    #[inline(always)]
    pub fn executez(&self) -> EXECUTEZ_R {
        EXECUTEZ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FSM_ACT"]
    #[inline(always)]
    pub fn fsm_act(&self) -> FSM_ACT_R {
        FSM_ACT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIOTP_ACT"]
    #[inline(always)]
    pub fn tiotp_act(&self) -> TIOTP_ACT_R {
        TIOTP_ACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OTP_ACT"]
    #[inline(always)]
    pub fn otp_act(&self) -> OTP_ACT_R {
        OTP_ACT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
