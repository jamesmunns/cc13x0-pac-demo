#[doc = "Reader of register FCFG_B5_START"]
pub type R = crate::R<u32, super::FCFG_B5_START>;
#[doc = "Reader of field `B5_MAX_SECTOR`"]
pub type B5_MAX_SECTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B5_MUX_FACTOR`"]
pub type B5_MUX_FACTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B5_START_ADDR`"]
pub type B5_START_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 28:31 - B5_MAX_SECTOR"]
    #[inline(always)]
    pub fn b5_max_sector(&self) -> B5_MAX_SECTOR_R {
        B5_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - B5_MUX_FACTOR"]
    #[inline(always)]
    pub fn b5_mux_factor(&self) -> B5_MUX_FACTOR_R {
        B5_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - B5_START_ADDR"]
    #[inline(always)]
    pub fn b5_start_addr(&self) -> B5_START_ADDR_R {
        B5_START_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
