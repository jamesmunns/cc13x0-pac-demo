#[doc = "Reader of register VOLT_LOAD_1"]
pub type R = crate::R<u32, super::VOLT_LOAD_1>;
#[doc = "Reader of field `VDDR_EXT_TP125`"]
pub type VDDR_EXT_TP125_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_EXT_TP105`"]
pub type VDDR_EXT_TP105_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_EXT_TP85`"]
pub type VDDR_EXT_TP85_R = crate::R<u8, u8>;
#[doc = "Reader of field `VDDR_EXT_TP65`"]
pub type VDDR_EXT_TP65_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 24:31 - VDDR_EXT_TP125"]
    #[inline(always)]
    pub fn vddr_ext_tp125(&self) -> VDDR_EXT_TP125_R {
        VDDR_EXT_TP125_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - VDDR_EXT_TP105"]
    #[inline(always)]
    pub fn vddr_ext_tp105(&self) -> VDDR_EXT_TP105_R {
        VDDR_EXT_TP105_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VDDR_EXT_TP85"]
    #[inline(always)]
    pub fn vddr_ext_tp85(&self) -> VDDR_EXT_TP85_R {
        VDDR_EXT_TP85_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - VDDR_EXT_TP65"]
    #[inline(always)]
    pub fn vddr_ext_tp65(&self) -> VDDR_EXT_TP65_R {
        VDDR_EXT_TP65_R::new((self.bits & 0xff) as u8)
    }
}
