#[doc = "Reader of register DOUT15_12"]
pub type R = crate::R<u32, super::DOUT15_12>;
#[doc = "Writer for register DOUT15_12"]
pub type W = crate::W<u32, super::DOUT15_12>;
#[doc = "Register DOUT15_12 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUT15_12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIO15`"]
pub type DIO15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO15`"]
pub struct DIO15_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `DIO14`"]
pub type DIO14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO14`"]
pub struct DIO14_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIO13`"]
pub type DIO13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO13`"]
pub struct DIO13_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO13_W<'a> {
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
#[doc = "Reader of field `DIO12`"]
pub type DIO12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO12`"]
pub struct DIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO12_W<'a> {
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
    #[doc = "Bit 24 - DIO15"]
    #[inline(always)]
    pub fn dio15(&self) -> DIO15_R {
        DIO15_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DIO14"]
    #[inline(always)]
    pub fn dio14(&self) -> DIO14_R {
        DIO14_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIO13"]
    #[inline(always)]
    pub fn dio13(&self) -> DIO13_R {
        DIO13_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIO12"]
    #[inline(always)]
    pub fn dio12(&self) -> DIO12_R {
        DIO12_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - DIO15"]
    #[inline(always)]
    pub fn dio15(&mut self) -> DIO15_W {
        DIO15_W { w: self }
    }
    #[doc = "Bit 16 - DIO14"]
    #[inline(always)]
    pub fn dio14(&mut self) -> DIO14_W {
        DIO14_W { w: self }
    }
    #[doc = "Bit 8 - DIO13"]
    #[inline(always)]
    pub fn dio13(&mut self) -> DIO13_W {
        DIO13_W { w: self }
    }
    #[doc = "Bit 0 - DIO12"]
    #[inline(always)]
    pub fn dio12(&mut self) -> DIO12_W {
        DIO12_W { w: self }
    }
}
