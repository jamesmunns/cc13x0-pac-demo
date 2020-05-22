#[doc = "Reader of register CCFG_PROT_63_32"]
pub type R = crate::R<u32, super::CCFG_PROT_63_32>;
#[doc = "Reader of field `WRT_PROT_SEC_63`"]
pub type WRT_PROT_SEC_63_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_62`"]
pub type WRT_PROT_SEC_62_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_61`"]
pub type WRT_PROT_SEC_61_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_60`"]
pub type WRT_PROT_SEC_60_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_59`"]
pub type WRT_PROT_SEC_59_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_58`"]
pub type WRT_PROT_SEC_58_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_57`"]
pub type WRT_PROT_SEC_57_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_56`"]
pub type WRT_PROT_SEC_56_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_55`"]
pub type WRT_PROT_SEC_55_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_54`"]
pub type WRT_PROT_SEC_54_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_53`"]
pub type WRT_PROT_SEC_53_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_52`"]
pub type WRT_PROT_SEC_52_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_51`"]
pub type WRT_PROT_SEC_51_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_50`"]
pub type WRT_PROT_SEC_50_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_49`"]
pub type WRT_PROT_SEC_49_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_48`"]
pub type WRT_PROT_SEC_48_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_47`"]
pub type WRT_PROT_SEC_47_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_46`"]
pub type WRT_PROT_SEC_46_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_45`"]
pub type WRT_PROT_SEC_45_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_44`"]
pub type WRT_PROT_SEC_44_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_43`"]
pub type WRT_PROT_SEC_43_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_42`"]
pub type WRT_PROT_SEC_42_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_41`"]
pub type WRT_PROT_SEC_41_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_40`"]
pub type WRT_PROT_SEC_40_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_39`"]
pub type WRT_PROT_SEC_39_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_38`"]
pub type WRT_PROT_SEC_38_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_37`"]
pub type WRT_PROT_SEC_37_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_36`"]
pub type WRT_PROT_SEC_36_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_35`"]
pub type WRT_PROT_SEC_35_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_34`"]
pub type WRT_PROT_SEC_34_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_33`"]
pub type WRT_PROT_SEC_33_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_32`"]
pub type WRT_PROT_SEC_32_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - WRT_PROT_SEC_63"]
    #[inline(always)]
    pub fn wrt_prot_sec_63(&self) -> WRT_PROT_SEC_63_R {
        WRT_PROT_SEC_63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - WRT_PROT_SEC_62"]
    #[inline(always)]
    pub fn wrt_prot_sec_62(&self) -> WRT_PROT_SEC_62_R {
        WRT_PROT_SEC_62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - WRT_PROT_SEC_61"]
    #[inline(always)]
    pub fn wrt_prot_sec_61(&self) -> WRT_PROT_SEC_61_R {
        WRT_PROT_SEC_61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - WRT_PROT_SEC_60"]
    #[inline(always)]
    pub fn wrt_prot_sec_60(&self) -> WRT_PROT_SEC_60_R {
        WRT_PROT_SEC_60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - WRT_PROT_SEC_59"]
    #[inline(always)]
    pub fn wrt_prot_sec_59(&self) -> WRT_PROT_SEC_59_R {
        WRT_PROT_SEC_59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - WRT_PROT_SEC_58"]
    #[inline(always)]
    pub fn wrt_prot_sec_58(&self) -> WRT_PROT_SEC_58_R {
        WRT_PROT_SEC_58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - WRT_PROT_SEC_57"]
    #[inline(always)]
    pub fn wrt_prot_sec_57(&self) -> WRT_PROT_SEC_57_R {
        WRT_PROT_SEC_57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - WRT_PROT_SEC_56"]
    #[inline(always)]
    pub fn wrt_prot_sec_56(&self) -> WRT_PROT_SEC_56_R {
        WRT_PROT_SEC_56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - WRT_PROT_SEC_55"]
    #[inline(always)]
    pub fn wrt_prot_sec_55(&self) -> WRT_PROT_SEC_55_R {
        WRT_PROT_SEC_55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - WRT_PROT_SEC_54"]
    #[inline(always)]
    pub fn wrt_prot_sec_54(&self) -> WRT_PROT_SEC_54_R {
        WRT_PROT_SEC_54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - WRT_PROT_SEC_53"]
    #[inline(always)]
    pub fn wrt_prot_sec_53(&self) -> WRT_PROT_SEC_53_R {
        WRT_PROT_SEC_53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WRT_PROT_SEC_52"]
    #[inline(always)]
    pub fn wrt_prot_sec_52(&self) -> WRT_PROT_SEC_52_R {
        WRT_PROT_SEC_52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - WRT_PROT_SEC_51"]
    #[inline(always)]
    pub fn wrt_prot_sec_51(&self) -> WRT_PROT_SEC_51_R {
        WRT_PROT_SEC_51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - WRT_PROT_SEC_50"]
    #[inline(always)]
    pub fn wrt_prot_sec_50(&self) -> WRT_PROT_SEC_50_R {
        WRT_PROT_SEC_50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - WRT_PROT_SEC_49"]
    #[inline(always)]
    pub fn wrt_prot_sec_49(&self) -> WRT_PROT_SEC_49_R {
        WRT_PROT_SEC_49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WRT_PROT_SEC_48"]
    #[inline(always)]
    pub fn wrt_prot_sec_48(&self) -> WRT_PROT_SEC_48_R {
        WRT_PROT_SEC_48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - WRT_PROT_SEC_47"]
    #[inline(always)]
    pub fn wrt_prot_sec_47(&self) -> WRT_PROT_SEC_47_R {
        WRT_PROT_SEC_47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - WRT_PROT_SEC_46"]
    #[inline(always)]
    pub fn wrt_prot_sec_46(&self) -> WRT_PROT_SEC_46_R {
        WRT_PROT_SEC_46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WRT_PROT_SEC_45"]
    #[inline(always)]
    pub fn wrt_prot_sec_45(&self) -> WRT_PROT_SEC_45_R {
        WRT_PROT_SEC_45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - WRT_PROT_SEC_44"]
    #[inline(always)]
    pub fn wrt_prot_sec_44(&self) -> WRT_PROT_SEC_44_R {
        WRT_PROT_SEC_44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WRT_PROT_SEC_43"]
    #[inline(always)]
    pub fn wrt_prot_sec_43(&self) -> WRT_PROT_SEC_43_R {
        WRT_PROT_SEC_43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - WRT_PROT_SEC_42"]
    #[inline(always)]
    pub fn wrt_prot_sec_42(&self) -> WRT_PROT_SEC_42_R {
        WRT_PROT_SEC_42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WRT_PROT_SEC_41"]
    #[inline(always)]
    pub fn wrt_prot_sec_41(&self) -> WRT_PROT_SEC_41_R {
        WRT_PROT_SEC_41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WRT_PROT_SEC_40"]
    #[inline(always)]
    pub fn wrt_prot_sec_40(&self) -> WRT_PROT_SEC_40_R {
        WRT_PROT_SEC_40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WRT_PROT_SEC_39"]
    #[inline(always)]
    pub fn wrt_prot_sec_39(&self) -> WRT_PROT_SEC_39_R {
        WRT_PROT_SEC_39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WRT_PROT_SEC_38"]
    #[inline(always)]
    pub fn wrt_prot_sec_38(&self) -> WRT_PROT_SEC_38_R {
        WRT_PROT_SEC_38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WRT_PROT_SEC_37"]
    #[inline(always)]
    pub fn wrt_prot_sec_37(&self) -> WRT_PROT_SEC_37_R {
        WRT_PROT_SEC_37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WRT_PROT_SEC_36"]
    #[inline(always)]
    pub fn wrt_prot_sec_36(&self) -> WRT_PROT_SEC_36_R {
        WRT_PROT_SEC_36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WRT_PROT_SEC_35"]
    #[inline(always)]
    pub fn wrt_prot_sec_35(&self) -> WRT_PROT_SEC_35_R {
        WRT_PROT_SEC_35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WRT_PROT_SEC_34"]
    #[inline(always)]
    pub fn wrt_prot_sec_34(&self) -> WRT_PROT_SEC_34_R {
        WRT_PROT_SEC_34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - WRT_PROT_SEC_33"]
    #[inline(always)]
    pub fn wrt_prot_sec_33(&self) -> WRT_PROT_SEC_33_R {
        WRT_PROT_SEC_33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - WRT_PROT_SEC_32"]
    #[inline(always)]
    pub fn wrt_prot_sec_32(&self) -> WRT_PROT_SEC_32_R {
        WRT_PROT_SEC_32_R::new((self.bits & 0x01) != 0)
    }
}
