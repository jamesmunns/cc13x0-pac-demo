#[doc = "Reader of register KEYREADAREA"]
pub type R = crate::R<u32, super::KEYREADAREA>;
#[doc = "Writer for register KEYREADAREA"]
pub type W = crate::W<u32, super::KEYREADAREA>;
#[doc = "Register KEYREADAREA `reset()`'s with value 0x08"]
impl crate::ResetValue for super::KEYREADAREA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RAM_AREA`"]
pub type RAM_AREA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RAM_AREA`"]
pub struct RAM_AREA_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_AREA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - RAM_AREA"]
    #[inline(always)]
    pub fn ram_area(&self) -> RAM_AREA_R {
        RAM_AREA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bits 0:3 - RAM_AREA"]
    #[inline(always)]
    pub fn ram_area(&mut self) -> RAM_AREA_W {
        RAM_AREA_W { w: self }
    }
}
