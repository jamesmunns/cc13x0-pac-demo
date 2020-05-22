#[doc = "Reader of register IOSTRMIN"]
pub type R = crate::R<u32, super::IOSTRMIN>;
#[doc = "Writer for register IOSTRMIN"]
pub type W = crate::W<u32, super::IOSTRMIN>;
#[doc = "Register IOSTRMIN `reset()`'s with value 0x03"]
impl crate::ResetValue for super::IOSTRMIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `GRAY_CODE`"]
pub type GRAY_CODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GRAY_CODE`"]
pub struct GRAY_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GRAY_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - GRAY_CODE"]
    #[inline(always)]
    pub fn gray_code(&self) -> GRAY_CODE_R {
        GRAY_CODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - GRAY_CODE"]
    #[inline(always)]
    pub fn gray_code(&mut self) -> GRAY_CODE_W {
        GRAY_CODE_W { w: self }
    }
}
