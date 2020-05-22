#[doc = "Reader of register PDCTL0"]
pub type R = crate::R<u32, super::PDCTL0>;
#[doc = "Writer for register PDCTL0"]
pub type W = crate::W<u32, super::PDCTL0>;
#[doc = "Register PDCTL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PDCTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PERIPH_ON`"]
pub type PERIPH_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERIPH_ON`"]
pub struct PERIPH_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIPH_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SERIAL_ON`"]
pub type SERIAL_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERIAL_ON`"]
pub struct SERIAL_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SERIAL_ON_W<'a> {
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
#[doc = "Reader of field `RFC_ON`"]
pub type RFC_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFC_ON`"]
pub struct RFC_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_ON_W<'a> {
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
    #[doc = "Bit 2 - PERIPH_ON"]
    #[inline(always)]
    pub fn periph_on(&self) -> PERIPH_ON_R {
        PERIPH_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - SERIAL_ON"]
    #[inline(always)]
    pub fn serial_on(&self) -> SERIAL_ON_R {
        SERIAL_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RFC_ON"]
    #[inline(always)]
    pub fn rfc_on(&self) -> RFC_ON_R {
        RFC_ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PERIPH_ON"]
    #[inline(always)]
    pub fn periph_on(&mut self) -> PERIPH_ON_W {
        PERIPH_ON_W { w: self }
    }
    #[doc = "Bit 1 - SERIAL_ON"]
    #[inline(always)]
    pub fn serial_on(&mut self) -> SERIAL_ON_W {
        SERIAL_ON_W { w: self }
    }
    #[doc = "Bit 0 - RFC_ON"]
    #[inline(always)]
    pub fn rfc_on(&mut self) -> RFC_ON_W {
        RFC_ON_W { w: self }
    }
}
