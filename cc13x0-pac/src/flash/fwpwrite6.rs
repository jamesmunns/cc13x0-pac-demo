#[doc = "Reader of register FWPWRITE6"]
pub type R = crate::R<u32, super::FWPWRITE6>;
#[doc = "Writer for register FWPWRITE6"]
pub type W = crate::W<u32, super::FWPWRITE6>;
#[doc = "Register FWPWRITE6 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FWPWRITE6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `FWPWRITE6`"]
pub type FWPWRITE6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FWPWRITE6`"]
pub struct FWPWRITE6_W<'a> {
    w: &'a mut W,
}
impl<'a> FWPWRITE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FWPWRITE6"]
    #[inline(always)]
    pub fn fwpwrite6(&self) -> FWPWRITE6_R {
        FWPWRITE6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FWPWRITE6"]
    #[inline(always)]
    pub fn fwpwrite6(&mut self) -> FWPWRITE6_W {
        FWPWRITE6_W { w: self }
    }
}
