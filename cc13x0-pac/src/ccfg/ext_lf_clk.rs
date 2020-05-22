#[doc = "Reader of register EXT_LF_CLK"]
pub type R = crate::R<u32, super::EXT_LF_CLK>;
#[doc = "Reader of field `DIO`"]
pub type DIO_R = crate::R<u8, u8>;
#[doc = "Reader of field `RTC_INCREMENT`"]
pub type RTC_INCREMENT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 24:31 - DIO"]
    #[inline(always)]
    pub fn dio(&self) -> DIO_R {
        DIO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:23 - RTC_INCREMENT"]
    #[inline(always)]
    pub fn rtc_increment(&self) -> RTC_INCREMENT_R {
        RTC_INCREMENT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
