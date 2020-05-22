#[doc = "Reader of register TBMATCHR"]
pub type R = crate::R<u32, super::TBMATCHR>;
#[doc = "Writer for register TBMATCHR"]
pub type W = crate::W<u32, super::TBMATCHR>;
#[doc = "Register TBMATCHR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::TBMATCHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `TBMATCHR`"]
pub type TBMATCHR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TBMATCHR`"]
pub struct TBMATCHR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMATCHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TBMATCHR"]
    #[inline(always)]
    pub fn tbmatchr(&self) -> TBMATCHR_R {
        TBMATCHR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TBMATCHR"]
    #[inline(always)]
    pub fn tbmatchr(&mut self) -> TBMATCHR_W {
        TBMATCHR_W { w: self }
    }
}
