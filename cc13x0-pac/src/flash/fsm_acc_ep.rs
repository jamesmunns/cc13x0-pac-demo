#[doc = "Reader of register FSM_ACC_EP"]
pub type R = crate::R<u32, super::FSM_ACC_EP>;
#[doc = "Reader of field `ACC_EP`"]
pub type ACC_EP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - ACC_EP"]
    #[inline(always)]
    pub fn acc_ep(&self) -> ACC_EP_R {
        ACC_EP_R::new((self.bits & 0xffff) as u16)
    }
}
