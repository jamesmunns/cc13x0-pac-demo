#[doc = "Reader of register LDO_TRIM"]
pub type R = crate::R<u32, super::LDO_TRIM>;
#[doc = "Reader of field `VDDR_TRIM_SLEEP`"]
pub type VDDR_TRIM_SLEEP_R = crate::R<u8, u8>;
#[doc = "Reader of field `GLDO_CURSRC`"]
pub type GLDO_CURSRC_R = crate::R<u8, u8>;
#[doc = "Reader of field `ITRIM_DIGLDO_LOAD`"]
pub type ITRIM_DIGLDO_LOAD_R = crate::R<u8, u8>;
#[doc = "Reader of field `ITRIM_UDIGLDO`"]
pub type ITRIM_UDIGLDO_R = crate::R<u8, u8>;
#[doc = "Reader of field `VTRIM_DELTA`"]
pub type VTRIM_DELTA_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:28 - VDDR_TRIM_SLEEP"]
    #[inline(always)]
    pub fn vddr_trim_sleep(&self) -> VDDR_TRIM_SLEEP_R {
        VDDR_TRIM_SLEEP_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - GLDO_CURSRC"]
    #[inline(always)]
    pub fn gldo_cursrc(&self) -> GLDO_CURSRC_R {
        GLDO_CURSRC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 11:12 - ITRIM_DIGLDO_LOAD"]
    #[inline(always)]
    pub fn itrim_digldo_load(&self) -> ITRIM_DIGLDO_LOAD_R {
        ITRIM_DIGLDO_LOAD_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - ITRIM_UDIGLDO"]
    #[inline(always)]
    pub fn itrim_udigldo(&self) -> ITRIM_UDIGLDO_R {
        ITRIM_UDIGLDO_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - VTRIM_DELTA"]
    #[inline(always)]
    pub fn vtrim_delta(&self) -> VTRIM_DELTA_R {
        VTRIM_DELTA_R::new((self.bits & 0x07) as u8)
    }
}
