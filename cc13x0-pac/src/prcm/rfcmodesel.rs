#[doc = "Reader of register RFCMODESEL"]
pub type R = crate::R<u32, super::RFCMODESEL>;
#[doc = "Writer for register RFCMODESEL"]
pub type W = crate::W<u32, super::RFCMODESEL>;
#[doc = "Register RFCMODESEL `reset()`'s with value 0"]
impl crate::ResetValue for super::RFCMODESEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CURR`"]
pub type CURR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CURR`"]
pub struct CURR_W<'a> {
    w: &'a mut W,
}
impl<'a> CURR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CURR"]
    #[inline(always)]
    pub fn curr(&self) -> CURR_R {
        CURR_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CURR"]
    #[inline(always)]
    pub fn curr(&mut self) -> CURR_W {
        CURR_W { w: self }
    }
}
