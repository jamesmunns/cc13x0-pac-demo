#[doc = "Reader of register FWPWRITE2"]
pub type R = crate::R<u32, super::FWPWRITE2>;
#[doc = "Writer for register FWPWRITE2"]
pub type W = crate::W<u32, super::FWPWRITE2>;
#[doc = "Register FWPWRITE2 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FWPWRITE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `FWPWRITE2`"]
pub type FWPWRITE2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FWPWRITE2`"]
pub struct FWPWRITE2_W<'a> {
    w: &'a mut W,
}
impl<'a> FWPWRITE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FWPWRITE2"]
    #[inline(always)]
    pub fn fwpwrite2(&self) -> FWPWRITE2_R {
        FWPWRITE2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FWPWRITE2"]
    #[inline(always)]
    pub fn fwpwrite2(&mut self) -> FWPWRITE2_W {
        FWPWRITE2_W { w: self }
    }
}
