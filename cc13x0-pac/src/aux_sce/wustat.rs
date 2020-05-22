#[doc = "Reader of register WUSTAT"]
pub type R = crate::R<u32, super::WUSTAT>;
#[doc = "Reader of field `EXC_VECTOR`"]
pub type EXC_VECTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `WU_SIGNAL`"]
pub type WU_SIGNAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EV_SIGNALS`"]
pub type EV_SIGNALS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:17 - EXC_VECTOR"]
    #[inline(always)]
    pub fn exc_vector(&self) -> EXC_VECTOR_R {
        EXC_VECTOR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 8 - WU_SIGNAL"]
    #[inline(always)]
    pub fn wu_signal(&self) -> WU_SIGNAL_R {
        WU_SIGNAL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - EV_SIGNALS"]
    #[inline(always)]
    pub fn ev_signals(&self) -> EV_SIGNALS_R {
        EV_SIGNALS_R::new((self.bits & 0xff) as u8)
    }
}
