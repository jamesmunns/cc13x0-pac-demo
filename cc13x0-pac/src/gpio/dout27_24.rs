#[doc = "Reader of register DOUT27_24"]
pub type R = crate::R<u32, super::DOUT27_24>;
#[doc = "Writer for register DOUT27_24"]
pub type W = crate::W<u32, super::DOUT27_24>;
#[doc = "Register DOUT27_24 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUT27_24 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIO27`"]
pub type DIO27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO27`"]
pub struct DIO27_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO27_W<'a> {
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
#[doc = "Reader of field `DIO26`"]
pub type DIO26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO26`"]
pub struct DIO26_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO26_W<'a> {
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
#[doc = "Reader of field `DIO25`"]
pub type DIO25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO25`"]
pub struct DIO25_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO25_W<'a> {
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
#[doc = "Reader of field `DIO24`"]
pub type DIO24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO24`"]
pub struct DIO24_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO24_W<'a> {
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
    #[doc = "Bit 24 - DIO27"]
    #[inline(always)]
    pub fn dio27(&self) -> DIO27_R {
        DIO27_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DIO26"]
    #[inline(always)]
    pub fn dio26(&self) -> DIO26_R {
        DIO26_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIO25"]
    #[inline(always)]
    pub fn dio25(&self) -> DIO25_R {
        DIO25_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIO24"]
    #[inline(always)]
    pub fn dio24(&self) -> DIO24_R {
        DIO24_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - DIO27"]
    #[inline(always)]
    pub fn dio27(&mut self) -> DIO27_W {
        DIO27_W { w: self }
    }
    #[doc = "Bit 16 - DIO26"]
    #[inline(always)]
    pub fn dio26(&mut self) -> DIO26_W {
        DIO26_W { w: self }
    }
    #[doc = "Bit 8 - DIO25"]
    #[inline(always)]
    pub fn dio25(&mut self) -> DIO25_W {
        DIO25_W { w: self }
    }
    #[doc = "Bit 0 - DIO24"]
    #[inline(always)]
    pub fn dio24(&mut self) -> DIO24_W {
        DIO24_W { w: self }
    }
}
