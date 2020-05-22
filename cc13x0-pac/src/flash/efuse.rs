#[doc = "Writer for register EFUSE"]
pub type W = crate::W<u32, super::EFUSE>;
#[doc = "Register EFUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INSTRUCTION`"]
pub struct INSTRUCTION_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTRUCTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `DUMPWORD`"]
pub struct DUMPWORD_W<'a> {
    w: &'a mut W,
}
impl<'a> DUMPWORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 24:28 - INSTRUCTION"]
    #[inline(always)]
    pub fn instruction(&mut self) -> INSTRUCTION_W {
        INSTRUCTION_W { w: self }
    }
    #[doc = "Bits 0:15 - DUMPWORD"]
    #[inline(always)]
    pub fn dumpword(&mut self) -> DUMPWORD_W {
        DUMPWORD_W { w: self }
    }
}
