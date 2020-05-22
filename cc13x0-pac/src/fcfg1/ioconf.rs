#[doc = "Reader of register IOCONF"]
pub type R = crate::R<u32, super::IOCONF>;
#[doc = "Reader of field `GPIO_CNT`"]
pub type GPIO_CNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - GPIO_CNT"]
    #[inline(always)]
    pub fn gpio_cnt(&self) -> GPIO_CNT_R {
        GPIO_CNT_R::new((self.bits & 0x7f) as u8)
    }
}
