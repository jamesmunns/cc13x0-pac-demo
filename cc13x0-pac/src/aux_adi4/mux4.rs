#[doc = "Reader of register MUX4"]
pub type R = crate::R<u8, super::MUX4>;
#[doc = "Writer for register MUX4"]
pub type W = crate::W<u8, super::MUX4>;
#[doc = "Register MUX4 `reset()`'s with value 0"]
impl crate::ResetValue for super::MUX4 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPA_REF`"]
pub type COMPA_REF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPA_REF`"]
pub struct COMPA_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPA_REF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COMPA_REF"]
    #[inline(always)]
    pub fn compa_ref(&self) -> COMPA_REF_R {
        COMPA_REF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COMPA_REF"]
    #[inline(always)]
    pub fn compa_ref(&mut self) -> COMPA_REF_W {
        COMPA_REF_W { w: self }
    }
}
