#[doc = "Reader of register ALARMSTOP"]
pub type R = crate::R<u32, super::ALARMSTOP>;
#[doc = "Writer for register ALARMSTOP"]
pub type W = crate::W<u32, super::ALARMSTOP>;
#[doc = "Register ALARMSTOP `reset()`'s with value 0"]
impl crate::ResetValue for super::ALARMSTOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRO_FLAGS`"]
pub type FRO_FLAGS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FRO_FLAGS`"]
pub struct FRO_FLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO_FLAGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - FRO_FLAGS"]
    #[inline(always)]
    pub fn fro_flags(&self) -> FRO_FLAGS_R {
        FRO_FLAGS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - FRO_FLAGS"]
    #[inline(always)]
    pub fn fro_flags(&mut self) -> FRO_FLAGS_W {
        FRO_FLAGS_W { w: self }
    }
}
