#[doc = "Reader of register FWPWRITE0"]
pub type R = crate::R<u32, super::FWPWRITE0>;
#[doc = "Writer for register FWPWRITE0"]
pub type W = crate::W<u32, super::FWPWRITE0>;
#[doc = "Register FWPWRITE0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FWPWRITE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `FWPWRITE0`"]
pub type FWPWRITE0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FWPWRITE0`"]
pub struct FWPWRITE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FWPWRITE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FWPWRITE0"]
    #[inline(always)]
    pub fn fwpwrite0(&self) -> FWPWRITE0_R {
        FWPWRITE0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FWPWRITE0"]
    #[inline(always)]
    pub fn fwpwrite0(&mut self) -> FWPWRITE0_W {
        FWPWRITE0_W { w: self }
    }
}
