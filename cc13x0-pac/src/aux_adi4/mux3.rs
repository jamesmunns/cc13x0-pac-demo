#[doc = "Reader of register MUX3"]
pub type R = crate::R<u8, super::MUX3>;
#[doc = "Writer for register MUX3"]
pub type W = crate::W<u8, super::MUX3>;
#[doc = "Register MUX3 `reset()`'s with value 0"]
impl crate::ResetValue for super::MUX3 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADCCOMPB_IN`"]
pub type ADCCOMPB_IN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCCOMPB_IN`"]
pub struct ADCCOMPB_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCCOMPB_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADCCOMPB_IN"]
    #[inline(always)]
    pub fn adccompb_in(&self) -> ADCCOMPB_IN_R {
        ADCCOMPB_IN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADCCOMPB_IN"]
    #[inline(always)]
    pub fn adccompb_in(&mut self) -> ADCCOMPB_IN_W {
        ADCCOMPB_IN_W { w: self }
    }
}
