#[doc = "Reader of register ADCFIFOSTAT"]
pub type R = crate::R<u32, super::ADCFIFOSTAT>;
#[doc = "Reader of field `OVERFLOW`"]
pub type OVERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNDERFLOW`"]
pub type UNDERFLOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `FULL`"]
pub type FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALMOST_FULL`"]
pub type ALMOST_FULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EMPTY`"]
pub type EMPTY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FULL"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ALMOST_FULL"]
    #[inline(always)]
    pub fn almost_full(&self) -> ALMOST_FULL_R {
        ALMOST_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - EMPTY"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new((self.bits & 0x01) != 0)
    }
}
