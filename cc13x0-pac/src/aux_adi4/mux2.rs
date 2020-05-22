#[doc = "Reader of register MUX2"]
pub type R = crate::R<u8, super::MUX2>;
#[doc = "Writer for register MUX2"]
pub type W = crate::W<u8, super::MUX2>;
#[doc = "Register MUX2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MUX2 {
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
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u8) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `COMPB_REF`"]
pub type COMPB_REF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPB_REF`"]
pub struct COMPB_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPB_REF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - ADCCOMPB_IN"]
    #[inline(always)]
    pub fn adccompb_in(&self) -> ADCCOMPB_IN_R {
        ADCCOMPB_IN_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 0:2 - COMPB_REF"]
    #[inline(always)]
    pub fn compb_ref(&self) -> COMPB_REF_R {
        COMPB_REF_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - ADCCOMPB_IN"]
    #[inline(always)]
    pub fn adccompb_in(&mut self) -> ADCCOMPB_IN_W {
        ADCCOMPB_IN_W { w: self }
    }
    #[doc = "Bits 0:2 - COMPB_REF"]
    #[inline(always)]
    pub fn compb_ref(&mut self) -> COMPB_REF_W {
        COMPB_REF_W { w: self }
    }
}
