#[doc = "Reader of register IRQSTATMASK"]
pub type R = crate::R<u32, super::IRQSTATMASK>;
#[doc = "Reader of field `SHUTDOWN_OVF`"]
pub type SHUTDOWN_OVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDY`"]
pub type RDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - SHUTDOWN_OVF"]
    #[inline(always)]
    pub fn shutdown_ovf(&self) -> SHUTDOWN_OVF_R {
        SHUTDOWN_OVF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RDY"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 0x01) != 0)
    }
}
