#[doc = "Reader of register VOLT_TRIM"]
pub type R = crate::R<u32, super::VOLT_TRIM>;
#[doc = "Reader of field `VDDR_TRIM_HH`"]
pub type VDDR_TRIM_HH_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_TRIM_H`"]
pub type VDDR_TRIM_H_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_TRIM_SLEEP_H`"]
pub type VDDR_TRIM_SLEEP_H_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIMBOD_H`"]
pub type TRIMBOD_H_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:28 - VDDR_TRIM_HH"]
    #[inline(always)]
    pub fn vddr_trim_hh(&self) -> VDDR_TRIM_HH_R {
        VDDR_TRIM_HH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - VDDR_TRIM_H"]
    #[inline(always)]
    pub fn vddr_trim_h(&self) -> VDDR_TRIM_H_R {
        VDDR_TRIM_H_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - VDDR_TRIM_SLEEP_H"]
    #[inline(always)]
    pub fn vddr_trim_sleep_h(&self) -> VDDR_TRIM_SLEEP_H_R {
        VDDR_TRIM_SLEEP_H_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - TRIMBOD_H"]
    #[inline(always)]
    pub fn trimbod_h(&self) -> TRIMBOD_H_R {
        TRIMBOD_H_R::new((self.bits & 0x1f) as u8)
    }
}
