#[doc = "Reader of register FWLOCK"]
pub type R = crate::R<u32, super::FWLOCK>;
#[doc = "Writer for register FWLOCK"]
pub type W = crate::W<u32, super::FWLOCK>;
#[doc = "Register FWLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::FWLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FWLOCK`"]
pub type FWLOCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FWLOCK`"]
pub struct FWLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FWLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - FWLOCK"]
    #[inline(always)]
    pub fn fwlock(&self) -> FWLOCK_R {
        FWLOCK_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FWLOCK"]
    #[inline(always)]
    pub fn fwlock(&mut self) -> FWLOCK_W {
        FWLOCK_W { w: self }
    }
}
