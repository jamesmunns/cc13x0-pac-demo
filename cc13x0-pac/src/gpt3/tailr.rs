#[doc = "Reader of register TAILR"]
pub type R = crate::R<u32, super::TAILR>;
#[doc = "Writer for register TAILR"]
pub type W = crate::W<u32, super::TAILR>;
#[doc = "Register TAILR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::TAILR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `TAILR`"]
pub type TAILR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TAILR`"]
pub struct TAILR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAILR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TAILR"]
    #[inline(always)]
    pub fn tailr(&self) -> TAILR_R {
        TAILR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TAILR"]
    #[inline(always)]
    pub fn tailr(&mut self) -> TAILR_W {
        TAILR_W { w: self }
    }
}
