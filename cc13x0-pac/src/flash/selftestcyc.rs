#[doc = "Writer for register SELFTESTCYC"]
pub type W = crate::W<u32, super::SELFTESTCYC>;
#[doc = "Register SELFTESTCYC `reset()`'s with value 0"]
impl crate::ResetValue for super::SELFTESTCYC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CYCLES`"]
pub struct CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - CYCLES"]
    #[inline(always)]
    pub fn cycles(&mut self) -> CYCLES_W {
        CYCLES_W { w: self }
    }
}
