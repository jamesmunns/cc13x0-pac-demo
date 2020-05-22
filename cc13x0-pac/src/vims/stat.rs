#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `IDCODE_LB_DIS`"]
pub type IDCODE_LB_DIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `SYSBUS_LB_DIS`"]
pub type SYSBUS_LB_DIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MODE_CHANGING`"]
pub type MODE_CHANGING_R = crate::R<bool, bool>;
#[doc = "Reader of field `INV`"]
pub type INV_R = crate::R<bool, bool>;
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 5 - IDCODE_LB_DIS"]
    #[inline(always)]
    pub fn idcode_lb_dis(&self) -> IDCODE_LB_DIS_R {
        IDCODE_LB_DIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SYSBUS_LB_DIS"]
    #[inline(always)]
    pub fn sysbus_lb_dis(&self) -> SYSBUS_LB_DIS_R {
        SYSBUS_LB_DIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MODE_CHANGING"]
    #[inline(always)]
    pub fn mode_changing(&self) -> MODE_CHANGING_R {
        MODE_CHANGING_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - INV"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
