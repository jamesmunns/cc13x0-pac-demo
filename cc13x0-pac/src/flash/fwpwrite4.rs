#[doc = "Reader of register FWPWRITE4"]
pub type R = crate::R<u32, super::FWPWRITE4>;
#[doc = "Writer for register FWPWRITE4"]
pub type W = crate::W<u32, super::FWPWRITE4>;
#[doc = "Register FWPWRITE4 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FWPWRITE4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `FWPWRITE4`"]
pub type FWPWRITE4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FWPWRITE4`"]
pub struct FWPWRITE4_W<'a> {
    w: &'a mut W,
}
impl<'a> FWPWRITE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FWPWRITE4"]
    #[inline(always)]
    pub fn fwpwrite4(&self) -> FWPWRITE4_R {
        FWPWRITE4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FWPWRITE4"]
    #[inline(always)]
    pub fn fwpwrite4(&mut self) -> FWPWRITE4_W {
        FWPWRITE4_W { w: self }
    }
}
