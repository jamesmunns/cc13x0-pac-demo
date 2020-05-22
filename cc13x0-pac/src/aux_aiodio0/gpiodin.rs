#[doc = "Reader of register GPIODIN"]
pub type R = crate::R<u32, super::GPIODIN>;
#[doc = "Reader of field `IO7_0`"]
pub type IO7_0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - IO7_0"]
    #[inline(always)]
    pub fn io7_0(&self) -> IO7_0_R {
        IO7_0_R::new((self.bits & 0xff) as u8)
    }
}
