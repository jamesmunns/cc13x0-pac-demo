#[doc = "Reader of register FCFG_B3_START"]
pub type R = crate::R<u32, super::FCFG_B3_START>;
#[doc = "Reader of field `B3_MAX_SECTOR`"]
pub type B3_MAX_SECTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B3_MUX_FACTOR`"]
pub type B3_MUX_FACTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B3_START_ADDR`"]
pub type B3_START_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 28:31 - B3_MAX_SECTOR"]
    #[inline(always)]
    pub fn b3_max_sector(&self) -> B3_MAX_SECTOR_R {
        B3_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - B3_MUX_FACTOR"]
    #[inline(always)]
    pub fn b3_mux_factor(&self) -> B3_MUX_FACTOR_R {
        B3_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - B3_START_ADDR"]
    #[inline(always)]
    pub fn b3_start_addr(&self) -> B3_START_ADDR_R {
        B3_START_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
