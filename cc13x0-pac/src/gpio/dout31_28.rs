#[doc = "Reader of register DOUT31_28"]
pub type R = crate::R<u32, super::DOUT31_28>;
#[doc = "Writer for register DOUT31_28"]
pub type W = crate::W<u32, super::DOUT31_28>;
#[doc = "Register DOUT31_28 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUT31_28 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIO31`"]
pub type DIO31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO31`"]
pub struct DIO31_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO31_W<'a> {
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
#[doc = "Reader of field `DIO30`"]
pub type DIO30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO30`"]
pub struct DIO30_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO30_W<'a> {
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
#[doc = "Reader of field `DIO29`"]
pub type DIO29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO29`"]
pub struct DIO29_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO29_W<'a> {
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
#[doc = "Reader of field `DIO28`"]
pub type DIO28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO28`"]
pub struct DIO28_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO28_W<'a> {
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
    #[doc = "Bit 24 - DIO31"]
    #[inline(always)]
    pub fn dio31(&self) -> DIO31_R {
        DIO31_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DIO30"]
    #[inline(always)]
    pub fn dio30(&self) -> DIO30_R {
        DIO30_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIO29"]
    #[inline(always)]
    pub fn dio29(&self) -> DIO29_R {
        DIO29_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIO28"]
    #[inline(always)]
    pub fn dio28(&self) -> DIO28_R {
        DIO28_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - DIO31"]
    #[inline(always)]
    pub fn dio31(&mut self) -> DIO31_W {
        DIO31_W { w: self }
    }
    #[doc = "Bit 16 - DIO30"]
    #[inline(always)]
    pub fn dio30(&mut self) -> DIO30_W {
        DIO30_W { w: self }
    }
    #[doc = "Bit 8 - DIO29"]
    #[inline(always)]
    pub fn dio29(&mut self) -> DIO29_W {
        DIO29_W { w: self }
    }
    #[doc = "Bit 0 - DIO28"]
    #[inline(always)]
    pub fn dio28(&mut self) -> DIO28_W {
        DIO28_W { w: self }
    }
}
