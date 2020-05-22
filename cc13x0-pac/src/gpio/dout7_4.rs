#[doc = "Reader of register DOUT7_4"]
pub type R = crate::R<u32, super::DOUT7_4>;
#[doc = "Writer for register DOUT7_4"]
pub type W = crate::W<u32, super::DOUT7_4>;
#[doc = "Register DOUT7_4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUT7_4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIO7`"]
pub type DIO7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO7`"]
pub struct DIO7_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO7_W<'a> {
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
#[doc = "Reader of field `DIO6`"]
pub type DIO6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO6`"]
pub struct DIO6_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO6_W<'a> {
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
#[doc = "Reader of field `DIO5`"]
pub type DIO5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO5`"]
pub struct DIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO5_W<'a> {
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
#[doc = "Reader of field `DIO4`"]
pub type DIO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO4`"]
pub struct DIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO4_W<'a> {
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
    #[doc = "Bit 24 - DIO7"]
    #[inline(always)]
    pub fn dio7(&self) -> DIO7_R {
        DIO7_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DIO6"]
    #[inline(always)]
    pub fn dio6(&self) -> DIO6_R {
        DIO6_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIO5"]
    #[inline(always)]
    pub fn dio5(&self) -> DIO5_R {
        DIO5_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIO4"]
    #[inline(always)]
    pub fn dio4(&self) -> DIO4_R {
        DIO4_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - DIO7"]
    #[inline(always)]
    pub fn dio7(&mut self) -> DIO7_W {
        DIO7_W { w: self }
    }
    #[doc = "Bit 16 - DIO6"]
    #[inline(always)]
    pub fn dio6(&mut self) -> DIO6_W {
        DIO6_W { w: self }
    }
    #[doc = "Bit 8 - DIO5"]
    #[inline(always)]
    pub fn dio5(&mut self) -> DIO5_W {
        DIO5_W { w: self }
    }
    #[doc = "Bit 0 - DIO4"]
    #[inline(always)]
    pub fn dio4(&mut self) -> DIO4_W {
        DIO4_W { w: self }
    }
}
