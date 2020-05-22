#[doc = "Reader of register FSM_VSTAT"]
pub type R = crate::R<u32, super::FSM_VSTAT>;
#[doc = "Writer for register FSM_VSTAT"]
pub type W = crate::W<u32, super::FSM_VSTAT>;
#[doc = "Register FSM_VSTAT `reset()`'s with value 0x3000"]
impl crate::ResetValue for super::FSM_VSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3000
    }
}
#[doc = "Reader of field `VSTAT_CNT`"]
pub type VSTAT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSTAT_CNT`"]
pub struct VSTAT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> VSTAT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - VSTAT_CNT"]
    #[inline(always)]
    pub fn vstat_cnt(&self) -> VSTAT_CNT_R {
        VSTAT_CNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - VSTAT_CNT"]
    #[inline(always)]
    pub fn vstat_cnt(&mut self) -> VSTAT_CNT_W {
        VSTAT_CNT_W { w: self }
    }
}
