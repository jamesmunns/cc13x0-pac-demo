#[doc = "Reader of register FWFLAG"]
pub type R = crate::R<u32, super::FWFLAG>;
#[doc = "Writer for register FWFLAG"]
pub type W = crate::W<u32, super::FWFLAG>;
#[doc = "Register FWFLAG `reset()`'s with value 0"]
impl crate::ResetValue for super::FWFLAG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FWFLAG`"]
pub type FWFLAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FWFLAG`"]
pub struct FWFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> FWFLAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - FWFLAG"]
    #[inline(always)]
    pub fn fwflag(&self) -> FWFLAG_R {
        FWFLAG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FWFLAG"]
    #[inline(always)]
    pub fn fwflag(&mut self) -> FWFLAG_W {
        FWFLAG_W { w: self }
    }
}
