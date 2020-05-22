#[doc = "Reader of register SCEWEVSEL"]
pub type R = crate::R<u32, super::SCEWEVSEL>;
#[doc = "Writer for register SCEWEVSEL"]
pub type W = crate::W<u32, super::SCEWEVSEL>;
#[doc = "Register SCEWEVSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SCEWEVSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WEV7_EV`"]
pub type WEV7_EV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WEV7_EV`"]
pub struct WEV7_EV_W<'a> {
    w: &'a mut W,
}
impl<'a> WEV7_EV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - WEV7_EV"]
    #[inline(always)]
    pub fn wev7_ev(&self) -> WEV7_EV_R {
        WEV7_EV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - WEV7_EV"]
    #[inline(always)]
    pub fn wev7_ev(&mut self) -> WEV7_EV_W {
        WEV7_EV_W { w: self }
    }
}
