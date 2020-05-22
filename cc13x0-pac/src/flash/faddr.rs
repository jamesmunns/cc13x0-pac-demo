#[doc = "Reader of register FADDR"]
pub type R = crate::R<u32, super::FADDR>;
#[doc = "Writer for register FADDR"]
pub type W = crate::W<u32, super::FADDR>;
#[doc = "Register FADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FADDR`"]
pub type FADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FADDR`"]
pub struct FADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> FADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FADDR"]
    #[inline(always)]
    pub fn faddr(&self) -> FADDR_R {
        FADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FADDR"]
    #[inline(always)]
    pub fn faddr(&mut self) -> FADDR_W {
        FADDR_W { w: self }
    }
}
