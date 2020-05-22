#[doc = "Reader of register PDSTAT0"]
pub type R = crate::R<u32, super::PDSTAT0>;
#[doc = "Reader of field `PERIPH_ON`"]
pub type PERIPH_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `SERIAL_ON`"]
pub type SERIAL_ON_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFC_ON`"]
pub type RFC_ON_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - PERIPH_ON"]
    #[inline(always)]
    pub fn periph_on(&self) -> PERIPH_ON_R {
        PERIPH_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERIAL_ON"]
    #[inline(always)]
    pub fn serial_on(&self) -> SERIAL_ON_R {
        SERIAL_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RFC_ON"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RFC_ON_R {
        RFC_ON_R::new((self.bits & 0x01) != 0)
    }
}
