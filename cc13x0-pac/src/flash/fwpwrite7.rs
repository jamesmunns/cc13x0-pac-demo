#[doc = "Reader of register FWPWRITE7"]
pub type R = crate::R<u32, super::FWPWRITE7>;
#[doc = "Writer for register FWPWRITE7"]
pub type W = crate::W<u32, super::FWPWRITE7>;
#[doc = "Register FWPWRITE7 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FWPWRITE7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `FWPWRITE7`"]
pub type FWPWRITE7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FWPWRITE7`"]
pub struct FWPWRITE7_W<'a> {
    w: &'a mut W,
}
impl<'a> FWPWRITE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FWPWRITE7"]
    #[inline(always)]
    pub fn fwpwrite7(&self) -> FWPWRITE7_R {
        FWPWRITE7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FWPWRITE7"]
    #[inline(always)]
    pub fn fwpwrite7(&mut self) -> FWPWRITE7_W {
        FWPWRITE7_W { w: self }
    }
}
