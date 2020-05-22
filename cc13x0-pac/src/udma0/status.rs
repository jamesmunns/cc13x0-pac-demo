#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `TEST`"]
pub type TEST_R = crate::R<u8, u8>;
#[doc = "Reader of field `TOTALCHANNELS`"]
pub type TOTALCHANNELS_R = crate::R<u8, u8>;
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, u8>;
#[doc = "Reader of field `MASTERENABLE`"]
pub type MASTERENABLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 28:31 - TEST"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - TOTALCHANNELS"]
    #[inline(always)]
    pub fn totalchannels(&self) -> TOTALCHANNELS_R {
        TOTALCHANNELS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 4:7 - STATE"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - MASTERENABLE"]
    #[inline(always)]
    pub fn masterenable(&self) -> MASTERENABLE_R {
        MASTERENABLE_R::new((self.bits & 0x01) != 0)
    }
}
