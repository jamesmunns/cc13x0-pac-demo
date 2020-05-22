#[doc = "Reader of register DMASTAT"]
pub type R = crate::R<u32, super::DMASTAT>;
#[doc = "Reader of field `PORT_ERR`"]
pub type PORT_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1_ACTIVE`"]
pub type CH1_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH0_ACTIVE`"]
pub type CH0_ACTIVE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 17 - PORT_ERR"]
    #[inline(always)]
    pub fn port_err(&self) -> PORT_ERR_R {
        PORT_ERR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH1_ACTIVE"]
    #[inline(always)]
    pub fn ch1_active(&self) -> CH1_ACTIVE_R {
        CH1_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - CH0_ACTIVE"]
    #[inline(always)]
    pub fn ch0_active(&self) -> CH0_ACTIVE_R {
        CH0_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
}
