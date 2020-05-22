#[doc = "Reader of register MEASCFG"]
pub type R = crate::R<u32, super::MEASCFG>;
#[doc = "Writer for register MEASCFG"]
pub type W = crate::W<u32, super::MEASCFG>;
#[doc = "Register MEASCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MEASCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PER`"]
pub type PER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PER`"]
pub struct PER_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PER"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PER"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W { w: self }
    }
}
