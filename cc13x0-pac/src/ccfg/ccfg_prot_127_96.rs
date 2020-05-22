#[doc = "Reader of register CCFG_PROT_127_96"]
pub type R = crate::R<u32, super::CCFG_PROT_127_96>;
#[doc = "Reader of field `WRT_PROT_SEC_127`"]
pub type WRT_PROT_SEC_127_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_126`"]
pub type WRT_PROT_SEC_126_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_125`"]
pub type WRT_PROT_SEC_125_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_124`"]
pub type WRT_PROT_SEC_124_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_123`"]
pub type WRT_PROT_SEC_123_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_122`"]
pub type WRT_PROT_SEC_122_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_121`"]
pub type WRT_PROT_SEC_121_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_120`"]
pub type WRT_PROT_SEC_120_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_119`"]
pub type WRT_PROT_SEC_119_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_118`"]
pub type WRT_PROT_SEC_118_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_117`"]
pub type WRT_PROT_SEC_117_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_116`"]
pub type WRT_PROT_SEC_116_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_115`"]
pub type WRT_PROT_SEC_115_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_114`"]
pub type WRT_PROT_SEC_114_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_113`"]
pub type WRT_PROT_SEC_113_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_112`"]
pub type WRT_PROT_SEC_112_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_111`"]
pub type WRT_PROT_SEC_111_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_110`"]
pub type WRT_PROT_SEC_110_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_109`"]
pub type WRT_PROT_SEC_109_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_108`"]
pub type WRT_PROT_SEC_108_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_107`"]
pub type WRT_PROT_SEC_107_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_106`"]
pub type WRT_PROT_SEC_106_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_105`"]
pub type WRT_PROT_SEC_105_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_104`"]
pub type WRT_PROT_SEC_104_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_103`"]
pub type WRT_PROT_SEC_103_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_102`"]
pub type WRT_PROT_SEC_102_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_101`"]
pub type WRT_PROT_SEC_101_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_100`"]
pub type WRT_PROT_SEC_100_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_99`"]
pub type WRT_PROT_SEC_99_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_98`"]
pub type WRT_PROT_SEC_98_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_97`"]
pub type WRT_PROT_SEC_97_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_96`"]
pub type WRT_PROT_SEC_96_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - WRT_PROT_SEC_127"]
    #[inline(always)]
    pub fn wrt_prot_sec_127(&self) -> WRT_PROT_SEC_127_R {
        WRT_PROT_SEC_127_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - WRT_PROT_SEC_126"]
    #[inline(always)]
    pub fn wrt_prot_sec_126(&self) -> WRT_PROT_SEC_126_R {
        WRT_PROT_SEC_126_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - WRT_PROT_SEC_125"]
    #[inline(always)]
    pub fn wrt_prot_sec_125(&self) -> WRT_PROT_SEC_125_R {
        WRT_PROT_SEC_125_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - WRT_PROT_SEC_124"]
    #[inline(always)]
    pub fn wrt_prot_sec_124(&self) -> WRT_PROT_SEC_124_R {
        WRT_PROT_SEC_124_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - WRT_PROT_SEC_123"]
    #[inline(always)]
    pub fn wrt_prot_sec_123(&self) -> WRT_PROT_SEC_123_R {
        WRT_PROT_SEC_123_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - WRT_PROT_SEC_122"]
    #[inline(always)]
    pub fn wrt_prot_sec_122(&self) -> WRT_PROT_SEC_122_R {
        WRT_PROT_SEC_122_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - WRT_PROT_SEC_121"]
    #[inline(always)]
    pub fn wrt_prot_sec_121(&self) -> WRT_PROT_SEC_121_R {
        WRT_PROT_SEC_121_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - WRT_PROT_SEC_120"]
    #[inline(always)]
    pub fn wrt_prot_sec_120(&self) -> WRT_PROT_SEC_120_R {
        WRT_PROT_SEC_120_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - WRT_PROT_SEC_119"]
    #[inline(always)]
    pub fn wrt_prot_sec_119(&self) -> WRT_PROT_SEC_119_R {
        WRT_PROT_SEC_119_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - WRT_PROT_SEC_118"]
    #[inline(always)]
    pub fn wrt_prot_sec_118(&self) -> WRT_PROT_SEC_118_R {
        WRT_PROT_SEC_118_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - WRT_PROT_SEC_117"]
    #[inline(always)]
    pub fn wrt_prot_sec_117(&self) -> WRT_PROT_SEC_117_R {
        WRT_PROT_SEC_117_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WRT_PROT_SEC_116"]
    #[inline(always)]
    pub fn wrt_prot_sec_116(&self) -> WRT_PROT_SEC_116_R {
        WRT_PROT_SEC_116_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - WRT_PROT_SEC_115"]
    #[inline(always)]
    pub fn wrt_prot_sec_115(&self) -> WRT_PROT_SEC_115_R {
        WRT_PROT_SEC_115_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - WRT_PROT_SEC_114"]
    #[inline(always)]
    pub fn wrt_prot_sec_114(&self) -> WRT_PROT_SEC_114_R {
        WRT_PROT_SEC_114_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - WRT_PROT_SEC_113"]
    #[inline(always)]
    pub fn wrt_prot_sec_113(&self) -> WRT_PROT_SEC_113_R {
        WRT_PROT_SEC_113_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WRT_PROT_SEC_112"]
    #[inline(always)]
    pub fn wrt_prot_sec_112(&self) -> WRT_PROT_SEC_112_R {
        WRT_PROT_SEC_112_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - WRT_PROT_SEC_111"]
    #[inline(always)]
    pub fn wrt_prot_sec_111(&self) -> WRT_PROT_SEC_111_R {
        WRT_PROT_SEC_111_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - WRT_PROT_SEC_110"]
    #[inline(always)]
    pub fn wrt_prot_sec_110(&self) -> WRT_PROT_SEC_110_R {
        WRT_PROT_SEC_110_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WRT_PROT_SEC_109"]
    #[inline(always)]
    pub fn wrt_prot_sec_109(&self) -> WRT_PROT_SEC_109_R {
        WRT_PROT_SEC_109_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - WRT_PROT_SEC_108"]
    #[inline(always)]
    pub fn wrt_prot_sec_108(&self) -> WRT_PROT_SEC_108_R {
        WRT_PROT_SEC_108_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WRT_PROT_SEC_107"]
    #[inline(always)]
    pub fn wrt_prot_sec_107(&self) -> WRT_PROT_SEC_107_R {
        WRT_PROT_SEC_107_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - WRT_PROT_SEC_106"]
    #[inline(always)]
    pub fn wrt_prot_sec_106(&self) -> WRT_PROT_SEC_106_R {
        WRT_PROT_SEC_106_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WRT_PROT_SEC_105"]
    #[inline(always)]
    pub fn wrt_prot_sec_105(&self) -> WRT_PROT_SEC_105_R {
        WRT_PROT_SEC_105_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WRT_PROT_SEC_104"]
    #[inline(always)]
    pub fn wrt_prot_sec_104(&self) -> WRT_PROT_SEC_104_R {
        WRT_PROT_SEC_104_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WRT_PROT_SEC_103"]
    #[inline(always)]
    pub fn wrt_prot_sec_103(&self) -> WRT_PROT_SEC_103_R {
        WRT_PROT_SEC_103_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WRT_PROT_SEC_102"]
    #[inline(always)]
    pub fn wrt_prot_sec_102(&self) -> WRT_PROT_SEC_102_R {
        WRT_PROT_SEC_102_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WRT_PROT_SEC_101"]
    #[inline(always)]
    pub fn wrt_prot_sec_101(&self) -> WRT_PROT_SEC_101_R {
        WRT_PROT_SEC_101_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WRT_PROT_SEC_100"]
    #[inline(always)]
    pub fn wrt_prot_sec_100(&self) -> WRT_PROT_SEC_100_R {
        WRT_PROT_SEC_100_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WRT_PROT_SEC_99"]
    #[inline(always)]
    pub fn wrt_prot_sec_99(&self) -> WRT_PROT_SEC_99_R {
        WRT_PROT_SEC_99_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WRT_PROT_SEC_98"]
    #[inline(always)]
    pub fn wrt_prot_sec_98(&self) -> WRT_PROT_SEC_98_R {
        WRT_PROT_SEC_98_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - WRT_PROT_SEC_97"]
    #[inline(always)]
    pub fn wrt_prot_sec_97(&self) -> WRT_PROT_SEC_97_R {
        WRT_PROT_SEC_97_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - WRT_PROT_SEC_96"]
    #[inline(always)]
    pub fn wrt_prot_sec_96(&self) -> WRT_PROT_SEC_96_R {
        WRT_PROT_SEC_96_R::new((self.bits & 0x01) != 0)
    }
}
