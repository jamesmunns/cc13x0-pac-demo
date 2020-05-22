#[doc = "Reader of register ADCREF1"]
pub type R = crate::R<u8, super::ADCREF1>;
#[doc = "Writer for register ADCREF1"]
pub type W = crate::W<u8, super::ADCREF1>;
#[doc = "Register ADCREF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADCREF1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VTRIM`"]
pub type VTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VTRIM`"]
pub struct VTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u8) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - VTRIM"]
    #[inline(always)]
    pub fn vtrim(&self) -> VTRIM_R {
        VTRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - VTRIM"]
    #[inline(always)]
    pub fn vtrim(&mut self) -> VTRIM_W {
        VTRIM_W { w: self }
    }
}
