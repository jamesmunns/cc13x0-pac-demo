#[doc = "Reader of register RATCH7VAL"]
pub type R = crate::R<u32, super::RATCH7VAL>;
#[doc = "Writer for register RATCH7VAL"]
pub type W = crate::W<u32, super::RATCH7VAL>;
#[doc = "Register RATCH7VAL `reset()`'s with value 0"]
impl crate::ResetValue for super::RATCH7VAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL`"]
pub type VAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VAL`"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - VAL"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - VAL"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
}
