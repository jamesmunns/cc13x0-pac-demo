#[doc = "Reader of register FLASH_SIZE"]
pub type R = crate::R<u32, super::FLASH_SIZE>;
#[doc = "Writer for register FLASH_SIZE"]
pub type W = crate::W<u32, super::FLASH_SIZE>;
#[doc = "Register FLASH_SIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_SIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECTORS`"]
pub type SECTORS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECTORS`"]
pub struct SECTORS_W<'a> {
    w: &'a mut W,
}
impl<'a> SECTORS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SECTORS"]
    #[inline(always)]
    pub fn sectors(&self) -> SECTORS_R {
        SECTORS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SECTORS"]
    #[inline(always)]
    pub fn sectors(&mut self) -> SECTORS_W {
        SECTORS_W { w: self }
    }
}
