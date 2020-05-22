#[doc = "Reader of register AIFPWMVALUE"]
pub type R = crate::R<u32, super::AIFPWMVALUE>;
#[doc = "Writer for register AIFPWMVALUE"]
pub type W = crate::W<u32, super::AIFPWMVALUE>;
#[doc = "Register AIFPWMVALUE `reset()`'s with value 0"]
impl crate::ResetValue for super::AIFPWMVALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PULSE_WIDTH`"]
pub type PULSE_WIDTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PULSE_WIDTH`"]
pub struct PULSE_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSE_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PULSE_WIDTH"]
    #[inline(always)]
    pub fn pulse_width(&self) -> PULSE_WIDTH_R {
        PULSE_WIDTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PULSE_WIDTH"]
    #[inline(always)]
    pub fn pulse_width(&mut self) -> PULSE_WIDTH_W {
        PULSE_WIDTH_W { w: self }
    }
}
