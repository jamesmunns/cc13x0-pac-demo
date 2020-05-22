#[doc = "Reader of register MAC_BLE_0"]
pub type R = crate::R<u32, super::MAC_BLE_0>;
#[doc = "Reader of field `ADDR_0_31`"]
pub type ADDR_0_31_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADDR_0_31"]
    #[inline(always)]
    pub fn addr_0_31(&self) -> ADDR_0_31_R {
        ADDR_0_31_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
