#[doc = "Reader of register LOOPCNT"]
pub type R = crate::R<u32, super::LOOPCNT>;
#[doc = "Reader of field `ITER_LEFT`"]
pub type ITER_LEFT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - ITER_LEFT"]
    #[inline(always)]
    pub fn iter_left(&self) -> ITER_LEFT_R {
        ITER_LEFT_R::new((self.bits & 0xff) as u8)
    }
}
