#[doc = "Reader of register ADCREF0"]
pub type R = crate::R<u8, super::ADCREF0>;
#[doc = "Writer for register ADCREF0"]
pub type W = crate::W<u8, super::ADCREF0>;
#[doc = "Register ADCREF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCREF0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REF_ON_IDLE`"]
pub type REF_ON_IDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REF_ON_IDLE`"]
pub struct REF_ON_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REF_ON_IDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `IOMUX`"]
pub type IOMUX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOMUX`"]
pub struct IOMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMUX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXT`"]
pub struct EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - REF_ON_IDLE"]
    #[inline(always)]
    pub fn ref_on_idle(&self) -> REF_ON_IDLE_R {
        REF_ON_IDLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - IOMUX"]
    #[inline(always)]
    pub fn iomux(&self) -> IOMUX_R {
        IOMUX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EXT"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SRC"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - REF_ON_IDLE"]
    #[inline(always)]
    pub fn ref_on_idle(&mut self) -> REF_ON_IDLE_W {
        REF_ON_IDLE_W { w: self }
    }
    #[doc = "Bit 5 - IOMUX"]
    #[inline(always)]
    pub fn iomux(&mut self) -> IOMUX_W {
        IOMUX_W { w: self }
    }
    #[doc = "Bit 4 - EXT"]
    #[inline(always)]
    pub fn ext(&mut self) -> EXT_W {
        EXT_W { w: self }
    }
    #[doc = "Bit 3 - SRC"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
