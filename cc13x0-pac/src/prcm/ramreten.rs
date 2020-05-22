#[doc = "Reader of register RAMRETEN"]
pub type R = crate::R<u32, super::RAMRETEN>;
#[doc = "Writer for register RAMRETEN"]
pub type W = crate::W<u32, super::RAMRETEN>;
#[doc = "Register RAMRETEN `reset()`'s with value 0x03"]
impl crate::ResetValue for super::RAMRETEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `RFC`"]
pub type RFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFC`"]
pub struct RFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC_W<'a> {
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
#[doc = "Reader of field `VIMS`"]
pub type VIMS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VIMS`"]
pub struct VIMS_W<'a> {
    w: &'a mut W,
}
impl<'a> VIMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - RFC"]
    #[inline(always)]
    pub fn rfc(&self) -> RFC_R {
        RFC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - VIMS"]
    #[inline(always)]
    pub fn vims(&self) -> VIMS_R {
        VIMS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - RFC"]
    #[inline(always)]
    pub fn rfc(&mut self) -> RFC_W {
        RFC_W { w: self }
    }
    #[doc = "Bits 0:1 - VIMS"]
    #[inline(always)]
    pub fn vims(&mut self) -> VIMS_W {
        VIMS_W { w: self }
    }
}
