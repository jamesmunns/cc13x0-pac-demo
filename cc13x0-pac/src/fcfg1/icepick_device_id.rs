#[doc = "Reader of register ICEPICK_DEVICE_ID"]
pub type R = crate::R<u32, super::ICEPICK_DEVICE_ID>;
#[doc = "Reader of field `PG_REV`"]
pub type PG_REV_R = crate::R<u8, u8>;
#[doc = "Reader of field `WAFER_ID`"]
pub type WAFER_ID_R = crate::R<u16, u16>;
#[doc = "Reader of field `MANUFACTURER_ID`"]
pub type MANUFACTURER_ID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 28:31 - PG_REV"]
    #[inline(always)]
    pub fn pg_rev(&self) -> PG_REV_R {
        PG_REV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 12:27 - WAFER_ID"]
    #[inline(always)]
    pub fn wafer_id(&self) -> WAFER_ID_R {
        WAFER_ID_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 0:11 - MANUFACTURER_ID"]
    #[inline(always)]
    pub fn manufacturer_id(&self) -> MANUFACTURER_ID_R {
        MANUFACTURER_ID_R::new((self.bits & 0x0fff) as u16)
    }
}
