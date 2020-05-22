#[doc = "Reader of register DOUT19_16"]
pub type R = crate::R<u32, super::DOUT19_16>;
#[doc = "Writer for register DOUT19_16"]
pub type W = crate::W<u32, super::DOUT19_16>;
#[doc = "Register DOUT19_16 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUT19_16 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIO19`"]
pub type DIO19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO19`"]
pub struct DIO19_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO19_W<'a> {
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
#[doc = "Reader of field `DIO18`"]
pub type DIO18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO18`"]
pub struct DIO18_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO18_W<'a> {
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
#[doc = "Reader of field `DIO17`"]
pub type DIO17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO17`"]
pub struct DIO17_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO17_W<'a> {
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
#[doc = "Reader of field `DIO16`"]
pub type DIO16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO16`"]
pub struct DIO16_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO16_W<'a> {
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
    #[doc = "Bit 24 - DIO19"]
    #[inline(always)]
    pub fn dio19(&self) -> DIO19_R {
        DIO19_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DIO18"]
    #[inline(always)]
    pub fn dio18(&self) -> DIO18_R {
        DIO18_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIO17"]
    #[inline(always)]
    pub fn dio17(&self) -> DIO17_R {
        DIO17_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIO16"]
    #[inline(always)]
    pub fn dio16(&self) -> DIO16_R {
        DIO16_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - DIO19"]
    #[inline(always)]
    pub fn dio19(&mut self) -> DIO19_W {
        DIO19_W { w: self }
    }
    #[doc = "Bit 16 - DIO18"]
    #[inline(always)]
    pub fn dio18(&mut self) -> DIO18_W {
        DIO18_W { w: self }
    }
    #[doc = "Bit 8 - DIO17"]
    #[inline(always)]
    pub fn dio17(&mut self) -> DIO17_W {
        DIO17_W { w: self }
    }
    #[doc = "Bit 0 - DIO16"]
    #[inline(always)]
    pub fn dio16(&mut self) -> DIO16_W {
        DIO16_W { w: self }
    }
}
