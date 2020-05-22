#[doc = "Reader of register FSM_WR_ENA"]
pub type R = crate::R<u32, super::FSM_WR_ENA>;
#[doc = "Writer for register FSM_WR_ENA"]
pub type W = crate::W<u32, super::FSM_WR_ENA>;
#[doc = "Register FSM_WR_ENA `reset()`'s with value 0x02"]
impl crate::ResetValue for super::FSM_WR_ENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `WR_ENA`"]
pub type WR_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WR_ENA`"]
pub struct WR_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - WR_ENA"]
    #[inline(always)]
    pub fn wr_ena(&self) -> WR_ENA_R {
        WR_ENA_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WR_ENA"]
    #[inline(always)]
    pub fn wr_ena(&mut self) -> WR_ENA_W {
        WR_ENA_W { w: self }
    }
}
