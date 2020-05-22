#[doc = "Reader of register VOLT_LOAD_0"]
pub type R = crate::R<u32, super::VOLT_LOAD_0>;
#[doc = "Reader of field `VDDR_EXT_TP45`"]
pub type VDDR_EXT_TP45_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_EXT_TP25`"]
pub type VDDR_EXT_TP25_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_EXT_TP5`"]
pub type VDDR_EXT_TP5_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_EXT_TM15`"]
pub type VDDR_EXT_TM15_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - VDDR_EXT_TP45"]
    #[inline(always)]
    pub fn vddr_ext_tp45(&self) -> VDDR_EXT_TP45_R {
        VDDR_EXT_TP45_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - VDDR_EXT_TP25"]
    #[inline(always)]
    pub fn vddr_ext_tp25(&self) -> VDDR_EXT_TP25_R {
        VDDR_EXT_TP25_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VDDR_EXT_TP5"]
    #[inline(always)]
    pub fn vddr_ext_tp5(&self) -> VDDR_EXT_TP5_R {
        VDDR_EXT_TP5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - VDDR_EXT_TM15"]
    #[inline(always)]
    pub fn vddr_ext_tm15(&self) -> VDDR_EXT_TM15_R {
        VDDR_EXT_TM15_R::new((self.bits & 0xff) as u8)
    }
}
