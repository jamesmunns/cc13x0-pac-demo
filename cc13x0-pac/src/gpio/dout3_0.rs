#[doc = "Reader of register DOUT3_0"]
pub type R = crate::R<u32, super::DOUT3_0>;
#[doc = "Writer for register DOUT3_0"]
pub type W = crate::W<u32, super::DOUT3_0>;
#[doc = "Register DOUT3_0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DOUT3_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIO3`"]
pub type DIO3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO3`"]
pub struct DIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO3_W<'a> {
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
#[doc = "Reader of field `DIO2`"]
pub type DIO2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO2`"]
pub struct DIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO2_W<'a> {
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
#[doc = "Reader of field `DIO1`"]
pub type DIO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO1`"]
pub struct DIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO1_W<'a> {
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
#[doc = "Reader of field `DIO0`"]
pub type DIO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIO0`"]
pub struct DIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIO0_W<'a> {
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
    #[doc = "Bit 24 - DIO3"]
    #[inline(always)]
    pub fn dio3(&self) -> DIO3_R {
        DIO3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DIO2"]
    #[inline(always)]
    pub fn dio2(&self) -> DIO2_R {
        DIO2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIO1"]
    #[inline(always)]
    pub fn dio1(&self) -> DIO1_R {
        DIO1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DIO0"]
    #[inline(always)]
    pub fn dio0(&self) -> DIO0_R {
        DIO0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - DIO3"]
    #[inline(always)]
    pub fn dio3(&mut self) -> DIO3_W {
        DIO3_W { w: self }
    }
    #[doc = "Bit 16 - DIO2"]
    #[inline(always)]
    pub fn dio2(&mut self) -> DIO2_W {
        DIO2_W { w: self }
    }
    #[doc = "Bit 8 - DIO1"]
    #[inline(always)]
    pub fn dio1(&mut self) -> DIO1_W {
        DIO1_W { w: self }
    }
    #[doc = "Bit 0 - DIO0"]
    #[inline(always)]
    pub fn dio0(&mut self) -> DIO0_W {
        DIO0_W { w: self }
    }
}
