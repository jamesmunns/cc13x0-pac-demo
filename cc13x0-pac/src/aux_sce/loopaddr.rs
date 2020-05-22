#[doc = "Reader of register LOOPADDR"]
pub type R = crate::R<u32, super::LOOPADDR>;
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<u16, u16>;
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 16:31 - STOP"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - START"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0xffff) as u16)
    }
}
