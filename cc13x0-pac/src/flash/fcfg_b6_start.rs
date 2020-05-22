#[doc = "Reader of register FCFG_B6_START"]
pub type R = crate::R<u32, super::FCFG_B6_START>;
#[doc = "Reader of field `B6_MAX_SECTOR`"]
pub type B6_MAX_SECTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B6_MUX_FACTOR`"]
pub type B6_MUX_FACTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B6_START_ADDR`"]
pub type B6_START_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 28:31 - B6_MAX_SECTOR"]
    #[inline(always)]
    pub fn b6_max_sector(&self) -> B6_MAX_SECTOR_R {
        B6_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - B6_MUX_FACTOR"]
    #[inline(always)]
    pub fn b6_mux_factor(&self) -> B6_MUX_FACTOR_R {
        B6_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - B6_START_ADDR"]
    #[inline(always)]
    pub fn b6_start_addr(&self) -> B6_START_ADDR_R {
        B6_START_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
