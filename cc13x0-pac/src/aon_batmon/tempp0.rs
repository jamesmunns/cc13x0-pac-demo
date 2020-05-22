#[doc = "Reader of register TEMPP0"]
pub type R = crate::R<u32, super::TEMPP0>;
#[doc = "Writer for register TEMPP0"]
pub type W = crate::W<u32, super::TEMPP0>;
#[doc = "Register TEMPP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TEMPP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFG`"]
pub type CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFG`"]
pub struct CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CFG"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CFG"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W {
        CFG_W { w: self }
    }
}
