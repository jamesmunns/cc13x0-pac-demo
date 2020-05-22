#[doc = "Reader of register FEFUSEDATA"]
pub type R = crate::R<u32, super::FEFUSEDATA>;
#[doc = "Writer for register FEFUSEDATA"]
pub type W = crate::W<u32, super::FEFUSEDATA>;
#[doc = "Register FEFUSEDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::FEFUSEDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FEFUSEDATA`"]
pub type FEFUSEDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FEFUSEDATA`"]
pub struct FEFUSEDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> FEFUSEDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FEFUSEDATA"]
    #[inline(always)]
    pub fn fefusedata(&self) -> FEFUSEDATA_R {
        FEFUSEDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FEFUSEDATA"]
    #[inline(always)]
    pub fn fefusedata(&mut self) -> FEFUSEDATA_W {
        FEFUSEDATA_W { w: self }
    }
}
