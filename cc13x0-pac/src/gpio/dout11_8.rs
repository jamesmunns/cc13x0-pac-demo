#[doc = "Reader of register DOUT11_8"]
pub type R = crate::R<u32, super::DOUT11_8>;
#[doc = "Writer for register DOUT11_8"]
pub type W = crate::W<u32, super::DOUT11_8>;
#[doc = "Register DOUT11_8 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUT11_8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIO11`"]
pub type DIO11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO11`"]
pub struct DIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO11_W<'a> {
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
#[doc = "Reader of field `DIO10`"]
pub type DIO10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO10`"]
pub struct DIO10_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO10_W<'a> {
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
#[doc = "Reader of field `DIO9`"]
pub type DIO9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO9`"]
pub struct DIO9_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO9_W<'a> {
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
#[doc = "Reader of field `DIO8`"]
pub type DIO8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO8`"]
pub struct DIO8_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO8_W<'a> {
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
    #[doc = "Bit 24 - DIO11"]
    #[inline(always)]
    pub fn dio11(&self) -> DIO11_R {
        DIO11_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DIO10"]
    #[inline(always)]
    pub fn dio10(&self) -> DIO10_R {
        DIO10_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIO9"]
    #[inline(always)]
    pub fn dio9(&self) -> DIO9_R {
        DIO9_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIO8"]
    #[inline(always)]
    pub fn dio8(&self) -> DIO8_R {
        DIO8_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - DIO11"]
    #[inline(always)]
    pub fn dio11(&mut self) -> DIO11_W {
        DIO11_W { w: self }
    }
    #[doc = "Bit 16 - DIO10"]
    #[inline(always)]
    pub fn dio10(&mut self) -> DIO10_W {
        DIO10_W { w: self }
    }
    #[doc = "Bit 8 - DIO9"]
    #[inline(always)]
    pub fn dio9(&mut self) -> DIO9_W {
        DIO9_W { w: self }
    }
    #[doc = "Bit 0 - DIO8"]
    #[inline(always)]
    pub fn dio8(&mut self) -> DIO8_W {
        DIO8_W { w: self }
    }
}
