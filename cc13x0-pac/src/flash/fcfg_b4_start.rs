#[doc = "Reader of register FCFG_B4_START"]
pub type R = crate::R<u32, super::FCFG_B4_START>;
#[doc = "Reader of field `B4_MAX_SECTOR`"]
pub type B4_MAX_SECTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B4_MUX_FACTOR`"]
pub type B4_MUX_FACTOR_R = crate::R<u8, u8>;
#[doc = "Reader of field `B4_START_ADDR`"]
pub type B4_START_ADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 28:31 - B4_MAX_SECTOR"]
    #[inline(always)]
    pub fn b4_max_sector(&self) -> B4_MAX_SECTOR_R {
        B4_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - B4_MUX_FACTOR"]
    #[inline(always)]
    pub fn b4_mux_factor(&self) -> B4_MUX_FACTOR_R {
        B4_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:23 - B4_START_ADDR"]
    #[inline(always)]
    pub fn b4_start_addr(&self) -> B4_START_ADDR_R {
        B4_START_ADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
