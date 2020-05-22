#[doc = "Reader of register CCFG_PROT_31_0"]
pub type R = crate::R<u32, super::CCFG_PROT_31_0>;
#[doc = "Reader of field `WRT_PROT_SEC_31`"]
pub type WRT_PROT_SEC_31_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_30`"]
pub type WRT_PROT_SEC_30_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_29`"]
pub type WRT_PROT_SEC_29_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_28`"]
pub type WRT_PROT_SEC_28_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_27`"]
pub type WRT_PROT_SEC_27_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_26`"]
pub type WRT_PROT_SEC_26_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_25`"]
pub type WRT_PROT_SEC_25_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_24`"]
pub type WRT_PROT_SEC_24_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_23`"]
pub type WRT_PROT_SEC_23_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_22`"]
pub type WRT_PROT_SEC_22_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_21`"]
pub type WRT_PROT_SEC_21_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_20`"]
pub type WRT_PROT_SEC_20_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_19`"]
pub type WRT_PROT_SEC_19_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_18`"]
pub type WRT_PROT_SEC_18_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_17`"]
pub type WRT_PROT_SEC_17_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_16`"]
pub type WRT_PROT_SEC_16_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_15`"]
pub type WRT_PROT_SEC_15_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_14`"]
pub type WRT_PROT_SEC_14_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_13`"]
pub type WRT_PROT_SEC_13_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_12`"]
pub type WRT_PROT_SEC_12_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_11`"]
pub type WRT_PROT_SEC_11_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_10`"]
pub type WRT_PROT_SEC_10_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_9`"]
pub type WRT_PROT_SEC_9_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_8`"]
pub type WRT_PROT_SEC_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_7`"]
pub type WRT_PROT_SEC_7_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_6`"]
pub type WRT_PROT_SEC_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_5`"]
pub type WRT_PROT_SEC_5_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_4`"]
pub type WRT_PROT_SEC_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_3`"]
pub type WRT_PROT_SEC_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_2`"]
pub type WRT_PROT_SEC_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_1`"]
pub type WRT_PROT_SEC_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRT_PROT_SEC_0`"]
pub type WRT_PROT_SEC_0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - WRT_PROT_SEC_31"]
    #[inline(always)]
    pub fn wrt_prot_sec_31(&self) -> WRT_PROT_SEC_31_R {
        WRT_PROT_SEC_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - WRT_PROT_SEC_30"]
    #[inline(always)]
    pub fn wrt_prot_sec_30(&self) -> WRT_PROT_SEC_30_R {
        WRT_PROT_SEC_30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - WRT_PROT_SEC_29"]
    #[inline(always)]
    pub fn wrt_prot_sec_29(&self) -> WRT_PROT_SEC_29_R {
        WRT_PROT_SEC_29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - WRT_PROT_SEC_28"]
    #[inline(always)]
    pub fn wrt_prot_sec_28(&self) -> WRT_PROT_SEC_28_R {
        WRT_PROT_SEC_28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - WRT_PROT_SEC_27"]
    #[inline(always)]
    pub fn wrt_prot_sec_27(&self) -> WRT_PROT_SEC_27_R {
        WRT_PROT_SEC_27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - WRT_PROT_SEC_26"]
    #[inline(always)]
    pub fn wrt_prot_sec_26(&self) -> WRT_PROT_SEC_26_R {
        WRT_PROT_SEC_26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - WRT_PROT_SEC_25"]
    #[inline(always)]
    pub fn wrt_prot_sec_25(&self) -> WRT_PROT_SEC_25_R {
        WRT_PROT_SEC_25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - WRT_PROT_SEC_24"]
    #[inline(always)]
    pub fn wrt_prot_sec_24(&self) -> WRT_PROT_SEC_24_R {
        WRT_PROT_SEC_24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - WRT_PROT_SEC_23"]
    #[inline(always)]
    pub fn wrt_prot_sec_23(&self) -> WRT_PROT_SEC_23_R {
        WRT_PROT_SEC_23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - WRT_PROT_SEC_22"]
    #[inline(always)]
    pub fn wrt_prot_sec_22(&self) -> WRT_PROT_SEC_22_R {
        WRT_PROT_SEC_22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - WRT_PROT_SEC_21"]
    #[inline(always)]
    pub fn wrt_prot_sec_21(&self) -> WRT_PROT_SEC_21_R {
        WRT_PROT_SEC_21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WRT_PROT_SEC_20"]
    #[inline(always)]
    pub fn wrt_prot_sec_20(&self) -> WRT_PROT_SEC_20_R {
        WRT_PROT_SEC_20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - WRT_PROT_SEC_19"]
    #[inline(always)]
    pub fn wrt_prot_sec_19(&self) -> WRT_PROT_SEC_19_R {
        WRT_PROT_SEC_19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - WRT_PROT_SEC_18"]
    #[inline(always)]
    pub fn wrt_prot_sec_18(&self) -> WRT_PROT_SEC_18_R {
        WRT_PROT_SEC_18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - WRT_PROT_SEC_17"]
    #[inline(always)]
    pub fn wrt_prot_sec_17(&self) -> WRT_PROT_SEC_17_R {
        WRT_PROT_SEC_17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WRT_PROT_SEC_16"]
    #[inline(always)]
    pub fn wrt_prot_sec_16(&self) -> WRT_PROT_SEC_16_R {
        WRT_PROT_SEC_16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - WRT_PROT_SEC_15"]
    #[inline(always)]
    pub fn wrt_prot_sec_15(&self) -> WRT_PROT_SEC_15_R {
        WRT_PROT_SEC_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - WRT_PROT_SEC_14"]
    #[inline(always)]
    pub fn wrt_prot_sec_14(&self) -> WRT_PROT_SEC_14_R {
        WRT_PROT_SEC_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - WRT_PROT_SEC_13"]
    #[inline(always)]
    pub fn wrt_prot_sec_13(&self) -> WRT_PROT_SEC_13_R {
        WRT_PROT_SEC_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - WRT_PROT_SEC_12"]
    #[inline(always)]
    pub fn wrt_prot_sec_12(&self) -> WRT_PROT_SEC_12_R {
        WRT_PROT_SEC_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WRT_PROT_SEC_11"]
    #[inline(always)]
    pub fn wrt_prot_sec_11(&self) -> WRT_PROT_SEC_11_R {
        WRT_PROT_SEC_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - WRT_PROT_SEC_10"]
    #[inline(always)]
    pub fn wrt_prot_sec_10(&self) -> WRT_PROT_SEC_10_R {
        WRT_PROT_SEC_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - WRT_PROT_SEC_9"]
    #[inline(always)]
    pub fn wrt_prot_sec_9(&self) -> WRT_PROT_SEC_9_R {
        WRT_PROT_SEC_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WRT_PROT_SEC_8"]
    #[inline(always)]
    pub fn wrt_prot_sec_8(&self) -> WRT_PROT_SEC_8_R {
        WRT_PROT_SEC_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - WRT_PROT_SEC_7"]
    #[inline(always)]
    pub fn wrt_prot_sec_7(&self) -> WRT_PROT_SEC_7_R {
        WRT_PROT_SEC_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - WRT_PROT_SEC_6"]
    #[inline(always)]
    pub fn wrt_prot_sec_6(&self) -> WRT_PROT_SEC_6_R {
        WRT_PROT_SEC_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WRT_PROT_SEC_5"]
    #[inline(always)]
    pub fn wrt_prot_sec_5(&self) -> WRT_PROT_SEC_5_R {
        WRT_PROT_SEC_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WRT_PROT_SEC_4"]
    #[inline(always)]
    pub fn wrt_prot_sec_4(&self) -> WRT_PROT_SEC_4_R {
        WRT_PROT_SEC_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WRT_PROT_SEC_3"]
    #[inline(always)]
    pub fn wrt_prot_sec_3(&self) -> WRT_PROT_SEC_3_R {
        WRT_PROT_SEC_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WRT_PROT_SEC_2"]
    #[inline(always)]
    pub fn wrt_prot_sec_2(&self) -> WRT_PROT_SEC_2_R {
        WRT_PROT_SEC_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - WRT_PROT_SEC_1"]
    #[inline(always)]
    pub fn wrt_prot_sec_1(&self) -> WRT_PROT_SEC_1_R {
        WRT_PROT_SEC_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - WRT_PROT_SEC_0"]
    #[inline(always)]
    pub fn wrt_prot_sec_0(&self) -> WRT_PROT_SEC_0_R {
        WRT_PROT_SEC_0_R::new((self.bits & 0x01) != 0)
    }
}
