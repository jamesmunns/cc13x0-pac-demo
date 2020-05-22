#[doc = "Reader of register GPT1BCAPTSEL"]
pub type R = crate::R<u32, super::GPT1BCAPTSEL>;
#[doc = "Writer for register GPT1BCAPTSEL"]
pub type W = crate::W<u32, super::GPT1BCAPTSEL>;
#[doc = "Register GPT1BCAPTSEL `reset()`'s with value 0x58"]
impl crate::ResetValue for super::GPT1BCAPTSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x58
    }
}
#[doc = "Reader of field `EV`"]
pub type EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EV`"]
pub struct EV_W<'a> {
    w: &'a mut W,
}
impl<'a> EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - EV"]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - EV"]
    #[inline(always)]
    pub fn ev(&mut self) -> EV_W {
        EV_W { w: self }
    }
}
