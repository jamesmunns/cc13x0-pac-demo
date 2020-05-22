#[doc = "Reader of register INFRCLKDIVR"]
pub type R = crate::R<u32, super::INFRCLKDIVR>;
#[doc = "Writer for register INFRCLKDIVR"]
pub type W = crate::W<u32, super::INFRCLKDIVR>;
#[doc = "Register INFRCLKDIVR `reset()`'s with value 0"]
impl crate::ResetValue for super::INFRCLKDIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RATIO`"]
pub type RATIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RATIO`"]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RATIO"]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RATIO"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
}
