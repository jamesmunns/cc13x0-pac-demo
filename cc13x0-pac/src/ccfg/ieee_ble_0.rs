#[doc = "Reader of register IEEE_BLE_0"]
pub type R = crate::R<u32, super::IEEE_BLE_0>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
