#[doc = "Reader of register FBPRDY"]
pub type R = crate::R<u32, super::FBPRDY>;
#[doc = "Reader of field `BANKBUSY`"]
pub type BANKBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `PUMPRDY`"]
pub type PUMPRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BANKRDY`"]
pub type BANKRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 16 - BANKBUSY"]
    #[inline(always)]
    pub fn bankbusy(&self) -> BANKBUSY_R {
        BANKBUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - PUMPRDY"]
    #[inline(always)]
    pub fn pumprdy(&self) -> PUMPRDY_R {
        PUMPRDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 0 - BANKRDY"]
    #[inline(always)]
    pub fn bankrdy(&self) -> BANKRDY_R {
        BANKRDY_R::new((self.bits & 0x01) != 0)
    }
}
