#[doc = "Reader of register FLOCK"]
pub type R = crate::R<u32, super::FLOCK>;
#[doc = "Writer for register FLOCK"]
pub type W = crate::W<u32, super::FLOCK>;
#[doc = "Register FLOCK `reset()`'s with value 0x55aa"]
impl crate::ResetValue for super::FLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x55aa
    }
}
#[doc = "Reader of field `ENCOM`"]
pub type ENCOM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ENCOM`"]
pub struct ENCOM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCOM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ENCOM"]
    #[inline(always)]
    pub fn encom(&self) -> ENCOM_R {
        ENCOM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ENCOM"]
    #[inline(always)]
    pub fn encom(&mut self) -> ENCOM_W {
        ENCOM_W { w: self }
    }
}
