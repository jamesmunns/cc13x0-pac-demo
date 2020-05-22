#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STANDBY_MODE_SEL`"]
pub type STANDBY_MODE_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STANDBY_MODE_SEL`"]
pub struct STANDBY_MODE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_MODE_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `STANDBY_PW_SEL`"]
pub type STANDBY_PW_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STANDBY_PW_SEL`"]
pub struct STANDBY_PW_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_PW_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DIS_EFUSECLK`"]
pub type DIS_EFUSECLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_EFUSECLK`"]
pub struct DIS_EFUSECLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_EFUSECLK_W<'a> {
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
#[doc = "Reader of field `DIS_READACCESS`"]
pub type DIS_READACCESS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_READACCESS`"]
pub struct DIS_READACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_READACCESS_W<'a> {
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
#[doc = "Reader of field `ENABLE_SWINTF`"]
pub type ENABLE_SWINTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_SWINTF`"]
pub struct ENABLE_SWINTF_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_SWINTF_W<'a> {
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
#[doc = "Reader of field `DIS_STANDBY`"]
pub type DIS_STANDBY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_STANDBY`"]
pub struct DIS_STANDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_STANDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DIS_IDLE`"]
pub type DIS_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_IDLE`"]
pub struct DIS_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_IDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - STANDBY_MODE_SEL"]
    #[inline(always)]
    pub fn standby_mode_sel(&self) -> STANDBY_MODE_SEL_R {
        STANDBY_MODE_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - STANDBY_PW_SEL"]
    #[inline(always)]
    pub fn standby_pw_sel(&self) -> STANDBY_PW_SEL_R {
        STANDBY_PW_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - DIS_EFUSECLK"]
    #[inline(always)]
    pub fn dis_efuseclk(&self) -> DIS_EFUSECLK_R {
        DIS_EFUSECLK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DIS_READACCESS"]
    #[inline(always)]
    pub fn dis_readaccess(&self) -> DIS_READACCESS_R {
        DIS_READACCESS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ENABLE_SWINTF"]
    #[inline(always)]
    pub fn enable_swintf(&self) -> ENABLE_SWINTF_R {
        ENABLE_SWINTF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIS_STANDBY"]
    #[inline(always)]
    pub fn dis_standby(&self) -> DIS_STANDBY_R {
        DIS_STANDBY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIS_IDLE"]
    #[inline(always)]
    pub fn dis_idle(&self) -> DIS_IDLE_R {
        DIS_IDLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - STANDBY_MODE_SEL"]
    #[inline(always)]
    pub fn standby_mode_sel(&mut self) -> STANDBY_MODE_SEL_W {
        STANDBY_MODE_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - STANDBY_PW_SEL"]
    #[inline(always)]
    pub fn standby_pw_sel(&mut self) -> STANDBY_PW_SEL_W {
        STANDBY_PW_SEL_W { w: self }
    }
    #[doc = "Bit 5 - DIS_EFUSECLK"]
    #[inline(always)]
    pub fn dis_efuseclk(&mut self) -> DIS_EFUSECLK_W {
        DIS_EFUSECLK_W { w: self }
    }
    #[doc = "Bit 4 - DIS_READACCESS"]
    #[inline(always)]
    pub fn dis_readaccess(&mut self) -> DIS_READACCESS_W {
        DIS_READACCESS_W { w: self }
    }
    #[doc = "Bit 3 - ENABLE_SWINTF"]
    #[inline(always)]
    pub fn enable_swintf(&mut self) -> ENABLE_SWINTF_W {
        ENABLE_SWINTF_W { w: self }
    }
    #[doc = "Bit 1 - DIS_STANDBY"]
    #[inline(always)]
    pub fn dis_standby(&mut self) -> DIS_STANDBY_W {
        DIS_STANDBY_W { w: self }
    }
    #[doc = "Bit 0 - DIS_IDLE"]
    #[inline(always)]
    pub fn dis_idle(&mut self) -> DIS_IDLE_W {
        DIS_IDLE_W { w: self }
    }
}
