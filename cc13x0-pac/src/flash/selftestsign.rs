#[doc = "Writer for register SELFTESTSIGN"]
pub type W = crate::W<u32, super::SELFTESTSIGN>;
#[doc = "Register SELFTESTSIGN `reset()`'s with value 0"]
impl crate::ResetValue for super::SELFTESTSIGN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SIGNATURE`"]
pub struct SIGNATURE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGNATURE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - SIGNATURE"]
    #[inline(always)]
    pub fn signature(&mut self) -> SIGNATURE_W {
        SIGNATURE_W { w: self }
    }
}
