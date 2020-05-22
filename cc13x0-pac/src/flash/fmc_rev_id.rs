#[doc = "Reader of register FMC_REV_ID"]
pub type R = crate::R<u32, super::FMC_REV_ID>;
#[doc = "Reader of field `MOD_VERSION`"]
pub type MOD_VERSION_R = crate::R<u32, u32>;
#[doc = "Reader of field `CONFIG_CRC`"]
pub type CONFIG_CRC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 12:31 - MOD_VERSION"]
    #[inline(always)]
    pub fn mod_version(&self) -> MOD_VERSION_R {
        MOD_VERSION_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 0:11 - CONFIG_CRC"]
    #[inline(always)]
    pub fn config_crc(&self) -> CONFIG_CRC_R {
        CONFIG_CRC_R::new((self.bits & 0x0fff) as u16)
    }
}
