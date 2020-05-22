#[doc = "Reader of register CCFG_PROT_95_64"]
pub type R = crate::R<u32, super::CCFG_PROT_95_64>;
#[doc = "Reader of field `WRT_PROT_SEC_95`"]
pub type WRT_PROT_SEC_95_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_94`"]
pub type WRT_PROT_SEC_94_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_93`"]
pub type WRT_PROT_SEC_93_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_92`"]
pub type WRT_PROT_SEC_92_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_91`"]
pub type WRT_PROT_SEC_91_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_90`"]
pub type WRT_PROT_SEC_90_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_89`"]
pub type WRT_PROT_SEC_89_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_88`"]
pub type WRT_PROT_SEC_88_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_87`"]
pub type WRT_PROT_SEC_87_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_86`"]
pub type WRT_PROT_SEC_86_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_85`"]
pub type WRT_PROT_SEC_85_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_84`"]
pub type WRT_PROT_SEC_84_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_83`"]
pub type WRT_PROT_SEC_83_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_82`"]
pub type WRT_PROT_SEC_82_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_81`"]
pub type WRT_PROT_SEC_81_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_80`"]
pub type WRT_PROT_SEC_80_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_79`"]
pub type WRT_PROT_SEC_79_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_78`"]
pub type WRT_PROT_SEC_78_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_77`"]
pub type WRT_PROT_SEC_77_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_76`"]
pub type WRT_PROT_SEC_76_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_75`"]
pub type WRT_PROT_SEC_75_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_74`"]
pub type WRT_PROT_SEC_74_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_73`"]
pub type WRT_PROT_SEC_73_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_72`"]
pub type WRT_PROT_SEC_72_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_71`"]
pub type WRT_PROT_SEC_71_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_70`"]
pub type WRT_PROT_SEC_70_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_69`"]
pub type WRT_PROT_SEC_69_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_68`"]
pub type WRT_PROT_SEC_68_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_67`"]
pub type WRT_PROT_SEC_67_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_66`"]
pub type WRT_PROT_SEC_66_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_65`"]
pub type WRT_PROT_SEC_65_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_64`"]
pub type WRT_PROT_SEC_64_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - WRT_PROT_SEC_95"]
    #[inline(always)]
    pub fn wrt_prot_sec_95(&self) -> WRT_PROT_SEC_95_R {
        WRT_PROT_SEC_95_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - WRT_PROT_SEC_94"]
    #[inline(always)]
    pub fn wrt_prot_sec_94(&self) -> WRT_PROT_SEC_94_R {
        WRT_PROT_SEC_94_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - WRT_PROT_SEC_93"]
    #[inline(always)]
    pub fn wrt_prot_sec_93(&self) -> WRT_PROT_SEC_93_R {
        WRT_PROT_SEC_93_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - WRT_PROT_SEC_92"]
    #[inline(always)]
    pub fn wrt_prot_sec_92(&self) -> WRT_PROT_SEC_92_R {
        WRT_PROT_SEC_92_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - WRT_PROT_SEC_91"]
    #[inline(always)]
    pub fn wrt_prot_sec_91(&self) -> WRT_PROT_SEC_91_R {
        WRT_PROT_SEC_91_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - WRT_PROT_SEC_90"]
    #[inline(always)]
    pub fn wrt_prot_sec_90(&self) -> WRT_PROT_SEC_90_R {
        WRT_PROT_SEC_90_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - WRT_PROT_SEC_89"]
    #[inline(always)]
    pub fn wrt_prot_sec_89(&self) -> WRT_PROT_SEC_89_R {
        WRT_PROT_SEC_89_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - WRT_PROT_SEC_88"]
    #[inline(always)]
    pub fn wrt_prot_sec_88(&self) -> WRT_PROT_SEC_88_R {
        WRT_PROT_SEC_88_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - WRT_PROT_SEC_87"]
    #[inline(always)]
    pub fn wrt_prot_sec_87(&self) -> WRT_PROT_SEC_87_R {
        WRT_PROT_SEC_87_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - WRT_PROT_SEC_86"]
    #[inline(always)]
    pub fn wrt_prot_sec_86(&self) -> WRT_PROT_SEC_86_R {
        WRT_PROT_SEC_86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - WRT_PROT_SEC_85"]
    #[inline(always)]
    pub fn wrt_prot_sec_85(&self) -> WRT_PROT_SEC_85_R {
        WRT_PROT_SEC_85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WRT_PROT_SEC_84"]
    #[inline(always)]
    pub fn wrt_prot_sec_84(&self) -> WRT_PROT_SEC_84_R {
        WRT_PROT_SEC_84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - WRT_PROT_SEC_83"]
    #[inline(always)]
    pub fn wrt_prot_sec_83(&self) -> WRT_PROT_SEC_83_R {
        WRT_PROT_SEC_83_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - WRT_PROT_SEC_82"]
    #[inline(always)]
    pub fn wrt_prot_sec_82(&self) -> WRT_PROT_SEC_82_R {
        WRT_PROT_SEC_82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - WRT_PROT_SEC_81"]
    #[inline(always)]
    pub fn wrt_prot_sec_81(&self) -> WRT_PROT_SEC_81_R {
        WRT_PROT_SEC_81_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WRT_PROT_SEC_80"]
    #[inline(always)]
    pub fn wrt_prot_sec_80(&self) -> WRT_PROT_SEC_80_R {
        WRT_PROT_SEC_80_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - WRT_PROT_SEC_79"]
    #[inline(always)]
    pub fn wrt_prot_sec_79(&self) -> WRT_PROT_SEC_79_R {
        WRT_PROT_SEC_79_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - WRT_PROT_SEC_78"]
    #[inline(always)]
    pub fn wrt_prot_sec_78(&self) -> WRT_PROT_SEC_78_R {
        WRT_PROT_SEC_78_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WRT_PROT_SEC_77"]
    #[inline(always)]
    pub fn wrt_prot_sec_77(&self) -> WRT_PROT_SEC_77_R {
        WRT_PROT_SEC_77_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - WRT_PROT_SEC_76"]
    #[inline(always)]
    pub fn wrt_prot_sec_76(&self) -> WRT_PROT_SEC_76_R {
        WRT_PROT_SEC_76_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WRT_PROT_SEC_75"]
    #[inline(always)]
    pub fn wrt_prot_sec_75(&self) -> WRT_PROT_SEC_75_R {
        WRT_PROT_SEC_75_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - WRT_PROT_SEC_74"]
    #[inline(always)]
    pub fn wrt_prot_sec_74(&self) -> WRT_PROT_SEC_74_R {
        WRT_PROT_SEC_74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WRT_PROT_SEC_73"]
    #[inline(always)]
    pub fn wrt_prot_sec_73(&self) -> WRT_PROT_SEC_73_R {
        WRT_PROT_SEC_73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WRT_PROT_SEC_72"]
    #[inline(always)]
    pub fn wrt_prot_sec_72(&self) -> WRT_PROT_SEC_72_R {
        WRT_PROT_SEC_72_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WRT_PROT_SEC_71"]
    #[inline(always)]
    pub fn wrt_prot_sec_71(&self) -> WRT_PROT_SEC_71_R {
        WRT_PROT_SEC_71_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WRT_PROT_SEC_70"]
    #[inline(always)]
    pub fn wrt_prot_sec_70(&self) -> WRT_PROT_SEC_70_R {
        WRT_PROT_SEC_70_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WRT_PROT_SEC_69"]
    #[inline(always)]
    pub fn wrt_prot_sec_69(&self) -> WRT_PROT_SEC_69_R {
        WRT_PROT_SEC_69_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WRT_PROT_SEC_68"]
    #[inline(always)]
    pub fn wrt_prot_sec_68(&self) -> WRT_PROT_SEC_68_R {
        WRT_PROT_SEC_68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WRT_PROT_SEC_67"]
    #[inline(always)]
    pub fn wrt_prot_sec_67(&self) -> WRT_PROT_SEC_67_R {
        WRT_PROT_SEC_67_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WRT_PROT_SEC_66"]
    #[inline(always)]
    pub fn wrt_prot_sec_66(&self) -> WRT_PROT_SEC_66_R {
        WRT_PROT_SEC_66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - WRT_PROT_SEC_65"]
    #[inline(always)]
    pub fn wrt_prot_sec_65(&self) -> WRT_PROT_SEC_65_R {
        WRT_PROT_SEC_65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - WRT_PROT_SEC_64"]
    #[inline(always)]
    pub fn wrt_prot_sec_64(&self) -> WRT_PROT_SEC_64_R {
        WRT_PROT_SEC_64_R::new((self.bits & 0x01) != 0)
    }
}
