#[doc = "Reader of register MCUBUSSTAT"]
pub type R = crate::R<u32, super::MCUBUSSTAT>;
#[doc = "Reader of field `DISCONNECTED`"]
pub type DISCONNECTED_R = crate::R<bool, bool>;
#[doc = "Reader of field `DISCONNECT_ACK`"]
pub type DISCONNECT_ACK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - DISCONNECTED"]
    #[inline(always)]
    pub fn disconnected(&self) -> DISCONNECTED_R {
        DISCONNECTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DISCONNECT_ACK"]
    #[inline(always)]
    pub fn disconnect_ack(&self) -> DISCONNECT_ACK_R {
        DISCONNECT_ACK_R::new((self.bits & 0x01) != 0)
    }
}
