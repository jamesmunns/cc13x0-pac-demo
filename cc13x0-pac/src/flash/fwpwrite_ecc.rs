#[doc = "Reader of register FWPWRITE_ECC"]
pub type R = crate::R<u32, super::FWPWRITE_ECC>;
#[doc = "Writer for register FWPWRITE_ECC"]
pub type W = crate::W<u32, super::FWPWRITE_ECC>;
#[doc = "Register FWPWRITE_ECC `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FWPWRITE_ECC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `ECCBYTES07_00`"]
pub type ECCBYTES07_00_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ECCBYTES07_00`"]
pub struct ECCBYTES07_00_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCBYTES07_00_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `ECCBYTES15_08`"]
pub type ECCBYTES15_08_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ECCBYTES15_08`"]
pub struct ECCBYTES15_08_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCBYTES15_08_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ECCBYTES23_16`"]
pub type ECCBYTES23_16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ECCBYTES23_16`"]
pub struct ECCBYTES23_16_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCBYTES23_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ECCBYTES31_24`"]
pub type ECCBYTES31_24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ECCBYTES31_24`"]
pub struct ECCBYTES31_24_W<'a> {
    w: &'a mut W,
}
impl<'a> ECCBYTES31_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - ECCBYTES07_00"]
    #[inline(always)]
    pub fn eccbytes07_00(&self) -> ECCBYTES07_00_R {
        ECCBYTES07_00_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ECCBYTES15_08"]
    #[inline(always)]
    pub fn eccbytes15_08(&self) -> ECCBYTES15_08_R {
        ECCBYTES15_08_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ECCBYTES23_16"]
    #[inline(always)]
    pub fn eccbytes23_16(&self) -> ECCBYTES23_16_R {
        ECCBYTES23_16_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - ECCBYTES31_24"]
    #[inline(always)]
    pub fn eccbytes31_24(&self) -> ECCBYTES31_24_R {
        ECCBYTES31_24_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - ECCBYTES07_00"]
    #[inline(always)]
    pub fn eccbytes07_00(&mut self) -> ECCBYTES07_00_W {
        ECCBYTES07_00_W { w: self }
    }
    #[doc = "Bits 16:23 - ECCBYTES15_08"]
    #[inline(always)]
    pub fn eccbytes15_08(&mut self) -> ECCBYTES15_08_W {
        ECCBYTES15_08_W { w: self }
    }
    #[doc = "Bits 8:15 - ECCBYTES23_16"]
    #[inline(always)]
    pub fn eccbytes23_16(&mut self) -> ECCBYTES23_16_W {
        ECCBYTES23_16_W { w: self }
    }
    #[doc = "Bits 0:7 - ECCBYTES31_24"]
    #[inline(always)]
    pub fn eccbytes31_24(&mut self) -> ECCBYTES31_24_W {
        ECCBYTES31_24_W { w: self }
    }
}
