#[doc = "Reader of register MISC_CONF_1"]
pub type R = crate::R<u32, super::MISC_CONF_1>;
#[doc = "Reader of field `DEVICE_MINOR_REV`"]
pub type DEVICE_MINOR_REV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - DEVICE_MINOR_REV"]
    #[inline(always)]
    pub fn device_minor_rev(&self) -> DEVICE_MINOR_REV_R {
        DEVICE_MINOR_REV_R::new((self.bits & 0xff) as u8)
    }
}
