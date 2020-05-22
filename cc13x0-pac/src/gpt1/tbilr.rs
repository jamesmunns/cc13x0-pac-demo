#[doc = "Reader of register TBILR"]
pub type R = crate::R<u32, super::TBILR>;
#[doc = "Writer for register TBILR"]
pub type W = crate::W<u32, super::TBILR>;
#[doc = "Register TBILR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::TBILR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `TBILR`"]
pub type TBILR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TBILR`"]
pub struct TBILR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBILR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TBILR"]
    #[inline(always)]
    pub fn tbilr(&self) -> TBILR_R {
        TBILR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TBILR"]
    #[inline(always)]
    pub fn tbilr(&mut self) -> TBILR_W {
        TBILR_W { w: self }
    }
}
