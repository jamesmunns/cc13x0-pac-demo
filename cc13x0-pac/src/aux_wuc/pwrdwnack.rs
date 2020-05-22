#[doc = "Reader of register PWRDWNACK"]
pub type R = crate::R<u32, super::PWRDWNACK>;
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new((self.bits & 0x01) != 0)
    }
}
