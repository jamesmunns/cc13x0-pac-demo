#[doc = "Reader of register SATCFG"]
pub type R = crate::R<u32, super::SATCFG>;
#[doc = "Writer for register SATCFG"]
pub type W = crate::W<u32, super::SATCFG>;
#[doc = "Register SATCFG `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::SATCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `LIMIT`"]
pub type LIMIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LIMIT`"]
pub struct LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - LIMIT"]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LIMIT"]
    #[inline(always)]
    pub fn limit(&mut self) -> LIMIT_W {
        LIMIT_W { w: self }
    }
}
