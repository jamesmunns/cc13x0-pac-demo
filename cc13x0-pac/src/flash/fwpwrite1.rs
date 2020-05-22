#[doc = "Reader of register FWPWRITE1"]
pub type R = crate::R<u32, super::FWPWRITE1>;
#[doc = "Writer for register FWPWRITE1"]
pub type W = crate::W<u32, super::FWPWRITE1>;
#[doc = "Register FWPWRITE1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FWPWRITE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `FWPWRITE1`"]
pub type FWPWRITE1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FWPWRITE1`"]
pub struct FWPWRITE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FWPWRITE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FWPWRITE1"]
    #[inline(always)]
    pub fn fwpwrite1(&self) -> FWPWRITE1_R {
        FWPWRITE1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FWPWRITE1"]
    #[inline(always)]
    pub fn fwpwrite1(&mut self) -> FWPWRITE1_W {
        FWPWRITE1_W { w: self }
    }
}
