#[doc = "Reader of register FWPWRITE3"]
pub type R = crate::R<u32, super::FWPWRITE3>;
#[doc = "Writer for register FWPWRITE3"]
pub type W = crate::W<u32, super::FWPWRITE3>;
#[doc = "Register FWPWRITE3 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FWPWRITE3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `FWPWRITE3`"]
pub type FWPWRITE3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FWPWRITE3`"]
pub struct FWPWRITE3_W<'a> {
    w: &'a mut W,
}
impl<'a> FWPWRITE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FWPWRITE3"]
    #[inline(always)]
    pub fn fwpwrite3(&self) -> FWPWRITE3_R {
        FWPWRITE3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FWPWRITE3"]
    #[inline(always)]
    pub fn fwpwrite3(&mut self) -> FWPWRITE3_W {
        FWPWRITE3_W { w: self }
    }
}
