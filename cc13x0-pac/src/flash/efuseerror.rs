#[doc = "Writer for register EFUSEERROR"]
pub type W = crate::W<u32, super::EFUSEERROR>;
#[doc = "Register EFUSEERROR `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSEERROR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `CODE`"]
pub struct CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl W {
    #[doc = "Bit 5 - DONE"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bits 0:4 - CODE"]
    #[inline(always)]
    pub fn code(&mut self) -> CODE_W {
        CODE_W { w: self }
    }
}
