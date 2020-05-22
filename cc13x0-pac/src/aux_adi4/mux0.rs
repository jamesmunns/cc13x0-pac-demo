#[doc = "Reader of register MUX0"]
pub type R = crate::R<u8, super::MUX0>;
#[doc = "Writer for register MUX0"]
pub type W = crate::W<u8, super::MUX0>;
#[doc = "Register MUX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MUX0 {
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
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
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
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:7 - COMPA_IN"]
    #[inline(always)]
    pub fn compa_in(&self) -> COMPA_IN_R {
        COMPA_IN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - COMPA_REF"]
    #[inline(always)]
    pub fn compa_ref(&self) -> COMPA_REF_R {
        COMPA_REF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - COMPA_IN"]
    #[inline(always)]
    pub fn compa_in(&mut self) -> COMPA_IN_W {
        COMPA_IN_W { w: self }
    }
    #[doc = "Bits 0:3 - COMPA_REF"]
    #[inline(always)]
    pub fn compa_ref(&mut self) -> COMPA_REF_W {
        COMPA_REF_W { w: self }
    }
}
