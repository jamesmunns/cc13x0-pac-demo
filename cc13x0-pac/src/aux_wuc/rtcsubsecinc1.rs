#[doc = "Reader of register RTCSUBSECINC1"]
pub type R = crate::R<u32, super::RTCSUBSECINC1>;
#[doc = "Writer for register RTCSUBSECINC1"]
pub type W = crate::W<u32, super::RTCSUBSECINC1>;
#[doc = "Register RTCSUBSECINC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTCSUBSECINC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INC23_16`"]
pub type INC23_16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INC23_16`"]
pub struct INC23_16_W<'a> {
    w: &'a mut W,
}
impl<'a> INC23_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - INC23_16"]
    #[inline(always)]
    pub fn inc23_16(&self) -> INC23_16_R {
        INC23_16_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - INC23_16"]
    #[inline(always)]
    pub fn inc23_16(&mut self) -> INC23_16_W {
        INC23_16_W { w: self }
    }
}
