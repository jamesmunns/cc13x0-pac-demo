#[doc = "Reader of register FLASHPUMPP0"]
pub type R = crate::R<u32, super::FLASHPUMPP0>;
#[doc = "Writer for register FLASHPUMPP0"]
pub type W = crate::W<u32, super::FLASHPUMPP0>;
#[doc = "Register FLASHPUMPP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHPUMPP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FALLB`"]
pub type FALLB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FALLB`"]
pub struct FALLB_W<'a> {
    w: &'a mut W,
}
impl<'a> FALLB_W<'a> {
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
#[doc = "Reader of field `HIGHLIM`"]
pub type HIGHLIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HIGHLIM`"]
pub struct HIGHLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHLIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `LOWLIM`"]
pub type LOWLIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOWLIM`"]
pub struct LOWLIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWLIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVR`"]
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CFG`"]
pub type CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG`"]
pub struct CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - FALLB"]
    #[inline(always)]
    pub fn fallb(&self) -> FALLB_R {
        FALLB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - HIGHLIM"]
    #[inline(always)]
    pub fn highlim(&self) -> HIGHLIM_R {
        HIGHLIM_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - LOWLIM"]
    #[inline(always)]
    pub fn lowlim(&self) -> LOWLIM_R {
        LOWLIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - CFG"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - FALLB"]
    #[inline(always)]
    pub fn fallb(&mut self) -> FALLB_W {
        FALLB_W { w: self }
    }
    #[doc = "Bits 6:7 - HIGHLIM"]
    #[inline(always)]
    pub fn highlim(&mut self) -> HIGHLIM_W {
        HIGHLIM_W { w: self }
    }
    #[doc = "Bit 5 - LOWLIM"]
    #[inline(always)]
    pub fn lowlim(&mut self) -> LOWLIM_W {
        LOWLIM_W { w: self }
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    #[doc = "Bits 0:3 - CFG"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W {
        CFG_W { w: self }
    }
}
