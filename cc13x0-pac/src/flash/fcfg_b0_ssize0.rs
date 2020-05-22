#[doc = "Reader of register FCFG_B0_SSIZE0"]
pub type R = crate::R<u32, super::FCFG_B0_SSIZE0>;
#[doc = "Reader of field `B0_NUM_SECTORS`"]
pub type B0_NUM_SECTORS_R = crate::R<u16, u16>;
#[doc = "Reader of field `B0_SECT_SIZE`"]
pub type B0_SECT_SIZE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:27 - B0_NUM_SECTORS"]
    #[inline(always)]
    pub fn b0_num_sectors(&self) -> B0_NUM_SECTORS_R {
        B0_NUM_SECTORS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - B0_SECT_SIZE"]
    #[inline(always)]
    pub fn b0_sect_size(&self) -> B0_SECT_SIZE_R {
        B0_SECT_SIZE_R::new((self.bits & 0x0f) as u8)
    }
}
