#[doc = "Reader of register FMAC"]
pub type R = crate::R<u32, super::FMAC>;
#[doc = "Writer for register FMAC"]
pub type W = crate::W<u32, super::FMAC>;
#[doc = "Register FMAC `reset()`'s with value 0"]
impl crate::ResetValue for super::FMAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BANK`"]
pub type BANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BANK`"]
pub struct BANK_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - BANK"]
    #[inline(always)]
    pub fn bank(&self) -> BANK_R {
        BANK_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - BANK"]
    #[inline(always)]
    pub fn bank(&mut self) -> BANK_W {
        BANK_W { w: self }
    }
}
