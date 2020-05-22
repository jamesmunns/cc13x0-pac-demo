#[doc = "Reader of register FCFG_B1_START"]
pub type R = crate::R<u32, super::FCFG_B1_START>;
#[doc = "Reader of field `B1_MAX_SECTOR`"]
pub type B1_MAX_SECTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B1_MUX_FACTOR`"]
pub type B1_MUX_FACTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B1_START_ADDR`"]
pub type B1_START_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 28:31 - B1_MAX_SECTOR"]
    #[inline(always)]
    pub fn b1_max_sector(&self) -> B1_MAX_SECTOR_R {
        B1_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - B1_MUX_FACTOR"]
    #[inline(always)]
    pub fn b1_mux_factor(&self) -> B1_MUX_FACTOR_R {
        B1_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - B1_START_ADDR"]
    #[inline(always)]
    pub fn b1_start_addr(&self) -> B1_START_ADDR_R {
        B1_START_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
