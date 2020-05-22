#[doc = "Writer for register EFUSEKEY"]
pub type W = crate::W<u32, super::EFUSEKEY>;
#[doc = "Register EFUSEKEY `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSEKEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CODE`"]
pub struct CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - CODE"]
    #[inline(always)]
    pub fn code(&mut self) -> CODE_W {
        CODE_W { w: self }
    }
}
