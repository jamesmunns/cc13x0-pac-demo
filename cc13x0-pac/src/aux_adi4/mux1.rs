#[doc = "Reader of register MUX1"]
pub type R = crate::R<u8, super::MUX1>;
#[doc = "Writer for register MUX1"]
pub type W = crate::W<u8, super::MUX1>;
#[doc = "Register MUX1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MUX1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPA_IN`"]
pub type COMPA_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPA_IN`"]
pub struct COMPA_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPA_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COMPA_IN"]
    #[inline(always)]
    pub fn compa_in(&self) -> COMPA_IN_R {
        COMPA_IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COMPA_IN"]
    #[inline(always)]
    pub fn compa_in(&mut self) -> COMPA_IN_W {
        COMPA_IN_W { w: self }
    }
}
