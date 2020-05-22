#[doc = "Reader of register DIN31_0"]
pub type R = crate::R<u32, super::DIN31_0>;
#[doc = "Reader of field `DIO31`"]
pub type DIO31_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO30`"]
pub type DIO30_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO29`"]
pub type DIO29_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO28`"]
pub type DIO28_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO27`"]
pub type DIO27_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO26`"]
pub type DIO26_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO25`"]
pub type DIO25_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO24`"]
pub type DIO24_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO23`"]
pub type DIO23_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO22`"]
pub type DIO22_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO21`"]
pub type DIO21_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO20`"]
pub type DIO20_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO19`"]
pub type DIO19_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO18`"]
pub type DIO18_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO17`"]
pub type DIO17_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO16`"]
pub type DIO16_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO15`"]
pub type DIO15_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO14`"]
pub type DIO14_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO13`"]
pub type DIO13_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO12`"]
pub type DIO12_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO11`"]
pub type DIO11_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO10`"]
pub type DIO10_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO9`"]
pub type DIO9_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO8`"]
pub type DIO8_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO7`"]
pub type DIO7_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO6`"]
pub type DIO6_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO5`"]
pub type DIO5_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO4`"]
pub type DIO4_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO3`"]
pub type DIO3_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO2`"]
pub type DIO2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO1`"]
pub type DIO1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIO0`"]
pub type DIO0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - DIO31"]
    #[inline(always)]
    pub fn dio31(&self) -> DIO31_R {
        DIO31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DIO30"]
    #[inline(always)]
    pub fn dio30(&self) -> DIO30_R {
        DIO30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DIO29"]
    #[inline(always)]
    pub fn dio29(&self) -> DIO29_R {
        DIO29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DIO28"]
    #[inline(always)]
    pub fn dio28(&self) -> DIO28_R {
        DIO28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DIO27"]
    #[inline(always)]
    pub fn dio27(&self) -> DIO27_R {
        DIO27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DIO26"]
    #[inline(always)]
    pub fn dio26(&self) -> DIO26_R {
        DIO26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DIO25"]
    #[inline(always)]
    pub fn dio25(&self) -> DIO25_R {
        DIO25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DIO24"]
    #[inline(always)]
    pub fn dio24(&self) -> DIO24_R {
        DIO24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DIO23"]
    #[inline(always)]
    pub fn dio23(&self) -> DIO23_R {
        DIO23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DIO22"]
    #[inline(always)]
    pub fn dio22(&self) -> DIO22_R {
        DIO22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DIO21"]
    #[inline(always)]
    pub fn dio21(&self) -> DIO21_R {
        DIO21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DIO20"]
    #[inline(always)]
    pub fn dio20(&self) -> DIO20_R {
        DIO20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DIO19"]
    #[inline(always)]
    pub fn dio19(&self) -> DIO19_R {
        DIO19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DIO18"]
    #[inline(always)]
    pub fn dio18(&self) -> DIO18_R {
        DIO18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DIO17"]
    #[inline(always)]
    pub fn dio17(&self) -> DIO17_R {
        DIO17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DIO16"]
    #[inline(always)]
    pub fn dio16(&self) -> DIO16_R {
        DIO16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DIO15"]
    #[inline(always)]
    pub fn dio15(&self) -> DIO15_R {
        DIO15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DIO14"]
    #[inline(always)]
    pub fn dio14(&self) -> DIO14_R {
        DIO14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DIO13"]
    #[inline(always)]
    pub fn dio13(&self) -> DIO13_R {
        DIO13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DIO12"]
    #[inline(always)]
    pub fn dio12(&self) -> DIO12_R {
        DIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DIO11"]
    #[inline(always)]
    pub fn dio11(&self) -> DIO11_R {
        DIO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DIO10"]
    #[inline(always)]
    pub fn dio10(&self) -> DIO10_R {
        DIO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DIO9"]
    #[inline(always)]
    pub fn dio9(&self) -> DIO9_R {
        DIO9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIO8"]
    #[inline(always)]
    pub fn dio8(&self) -> DIO8_R {
        DIO8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DIO7"]
    #[inline(always)]
    pub fn dio7(&self) -> DIO7_R {
        DIO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DIO6"]
    #[inline(always)]
    pub fn dio6(&self) -> DIO6_R {
        DIO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DIO5"]
    #[inline(always)]
    pub fn dio5(&self) -> DIO5_R {
        DIO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIO4"]
    #[inline(always)]
    pub fn dio4(&self) -> DIO4_R {
        DIO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DIO3"]
    #[inline(always)]
    pub fn dio3(&self) -> DIO3_R {
        DIO3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DIO2"]
    #[inline(always)]
    pub fn dio2(&self) -> DIO2_R {
        DIO2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIO1"]
    #[inline(always)]
    pub fn dio1(&self) -> DIO1_R {
        DIO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIO0"]
    #[inline(always)]
    pub fn dio0(&self) -> DIO0_R {
        DIO0_R::new((self.bits & 0x01) != 0)
    }
}
