#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATS_CLR`"]
pub type STATS_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATS_CLR`"]
pub struct STATS_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> STATS_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `STATS_EN`"]
pub type STATS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATS_EN`"]
pub struct STATS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> STATS_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DYN_CG_EN`"]
pub type DYN_CG_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DYN_CG_EN`"]
pub struct DYN_CG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DYN_CG_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `IDCODE_LB_DIS`"]
pub type IDCODE_LB_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDCODE_LB_DIS`"]
pub struct IDCODE_LB_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IDCODE_LB_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SYSBUS_LB_DIS`"]
pub type SYSBUS_LB_DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSBUS_LB_DIS`"]
pub struct SYSBUS_LB_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSBUS_LB_DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ARB_CFG`"]
pub type ARB_CFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARB_CFG`"]
pub struct ARB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_CFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PREF_EN`"]
pub type PREF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREF_EN`"]
pub struct PREF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREF_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - STATS_CLR"]
    #[inline(always)]
    pub fn stats_clr(&self) -> STATS_CLR_R {
        STATS_CLR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - STATS_EN"]
    #[inline(always)]
    pub fn stats_en(&self) -> STATS_EN_R {
        STATS_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DYN_CG_EN"]
    #[inline(always)]
    pub fn dyn_cg_en(&self) -> DYN_CG_EN_R {
        DYN_CG_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
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
    #[doc = "Bit 3 - ARB_CFG"]
    #[inline(always)]
    pub fn arb_cfg(&self) -> ARB_CFG_R {
        ARB_CFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PREF_EN"]
    #[inline(always)]
    pub fn pref_en(&self) -> PREF_EN_R {
        PREF_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - STATS_CLR"]
    #[inline(always)]
    pub fn stats_clr(&mut self) -> STATS_CLR_W {
        STATS_CLR_W { w: self }
    }
    #[doc = "Bit 30 - STATS_EN"]
    #[inline(always)]
    pub fn stats_en(&mut self) -> STATS_EN_W {
        STATS_EN_W { w: self }
    }
    #[doc = "Bit 29 - DYN_CG_EN"]
    #[inline(always)]
    pub fn dyn_cg_en(&mut self) -> DYN_CG_EN_W {
        DYN_CG_EN_W { w: self }
    }
    #[doc = "Bit 5 - IDCODE_LB_DIS"]
    #[inline(always)]
    pub fn idcode_lb_dis(&mut self) -> IDCODE_LB_DIS_W {
        IDCODE_LB_DIS_W { w: self }
    }
    #[doc = "Bit 4 - SYSBUS_LB_DIS"]
    #[inline(always)]
    pub fn sysbus_lb_dis(&mut self) -> SYSBUS_LB_DIS_W {
        SYSBUS_LB_DIS_W { w: self }
    }
    #[doc = "Bit 3 - ARB_CFG"]
    #[inline(always)]
    pub fn arb_cfg(&mut self) -> ARB_CFG_W {
        ARB_CFG_W { w: self }
    }
    #[doc = "Bit 2 - PREF_EN"]
    #[inline(always)]
    pub fn pref_en(&mut self) -> PREF_EN_W {
        PREF_EN_W { w: self }
    }
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
