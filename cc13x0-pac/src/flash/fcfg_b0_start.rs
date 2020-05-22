#[doc = "Reader of register FCFG_B0_START"]
pub type R = crate::R<u32, super::FCFG_B0_START>;
#[doc = "Reader of field `B0_MAX_SECTOR`"]
pub type B0_MAX_SECTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B0_MUX_FACTOR`"]
pub type B0_MUX_FACTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B0_START_ADDR`"]
pub type B0_START_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 28:31 - B0_MAX_SECTOR"]
    #[inline(always)]
    pub fn b0_max_sector(&self) -> B0_MAX_SECTOR_R {
        B0_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - B0_MUX_FACTOR"]
    #[inline(always)]
    pub fn b0_mux_factor(&self) -> B0_MUX_FACTOR_R {
        B0_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - B0_START_ADDR"]
    #[inline(always)]
    pub fn b0_start_addr(&self) -> B0_START_ADDR_R {
        B0_START_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
