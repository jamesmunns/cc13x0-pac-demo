#[doc = "Reader of register DMACH0LEN"]
pub type R = crate::R<u32, super::DMACH0LEN>;
#[doc = "Writer for register DMACH0LEN"]
pub type W = crate::W<u32, super::DMACH0LEN>;
#[doc = "Register DMACH0LEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACH0LEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEN`"]
pub type LEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEN`"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - LEN"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LEN"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
}
