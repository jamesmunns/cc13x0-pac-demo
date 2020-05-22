#[doc = "Reader of register FCFG_B7_START"]
pub type R = crate::R<u32, super::FCFG_B7_START>;
#[doc = "Reader of field `B7_MAX_SECTOR`"]
pub type B7_MAX_SECTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B7_MUX_FACTOR`"]
pub type B7_MUX_FACTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B7_START_ADDR`"]
pub type B7_START_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 28:31 - B7_MAX_SECTOR"]
    #[inline(always)]
    pub fn b7_max_sector(&self) -> B7_MAX_SECTOR_R {
        B7_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - B7_MUX_FACTOR"]
    #[inline(always)]
    pub fn b7_mux_factor(&self) -> B7_MUX_FACTOR_R {
        B7_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - B7_START_ADDR"]
    #[inline(always)]
    pub fn b7_start_addr(&self) -> B7_START_ADDR_R {
        B7_START_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
