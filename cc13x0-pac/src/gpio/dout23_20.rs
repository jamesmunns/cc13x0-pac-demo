#[doc = "Reader of register DOUT23_20"]
pub type R = crate::R<u32, super::DOUT23_20>;
#[doc = "Writer for register DOUT23_20"]
pub type W = crate::W<u32, super::DOUT23_20>;
#[doc = "Register DOUT23_20 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUT23_20 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIO23`"]
pub type DIO23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO23`"]
pub struct DIO23_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO23_W<'a> {
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
#[doc = "Reader of field `DIO22`"]
pub type DIO22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO22`"]
pub struct DIO22_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO22_W<'a> {
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
#[doc = "Reader of field `DIO21`"]
pub type DIO21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO21`"]
pub struct DIO21_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO21_W<'a> {
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
#[doc = "Reader of field `DIO20`"]
pub type DIO20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO20`"]
pub struct DIO20_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO20_W<'a> {
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
    #[doc = "Bit 24 - DIO23"]
    #[inline(always)]
    pub fn dio23(&self) -> DIO23_R {
        DIO23_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DIO22"]
    #[inline(always)]
    pub fn dio22(&self) -> DIO22_R {
        DIO22_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIO21"]
    #[inline(always)]
    pub fn dio21(&self) -> DIO21_R {
        DIO21_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIO20"]
    #[inline(always)]
    pub fn dio20(&self) -> DIO20_R {
        DIO20_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - DIO23"]
    #[inline(always)]
    pub fn dio23(&mut self) -> DIO23_W {
        DIO23_W { w: self }
    }
    #[doc = "Bit 16 - DIO22"]
    #[inline(always)]
    pub fn dio22(&mut self) -> DIO22_W {
        DIO22_W { w: self }
    }
    #[doc = "Bit 8 - DIO21"]
    #[inline(always)]
    pub fn dio21(&mut self) -> DIO21_W {
        DIO21_W { w: self }
    }
    #[doc = "Bit 0 - DIO20"]
    #[inline(always)]
    pub fn dio20(&mut self) -> DIO20_W {
        DIO20_W { w: self }
    }
}
