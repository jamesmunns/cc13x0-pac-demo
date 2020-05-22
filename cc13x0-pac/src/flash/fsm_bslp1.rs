#[doc = "Reader of register FSM_BSLP1"]
pub type R = crate::R<u32, super::FSM_BSLP1>;
#[doc = "Writer for register FSM_BSLP1"]
pub type W = crate::W<u32, super::FSM_BSLP1>;
#[doc = "Register FSM_BSLP1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_BSLP1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSM_BSL1`"]
pub type FSM_BSL1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FSM_BSL1`"]
pub struct FSM_BSL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_BSL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FSM_BSL1"]
    #[inline(always)]
    pub fn fsm_bsl1(&self) -> FSM_BSL1_R {
        FSM_BSL1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FSM_BSL1"]
    #[inline(always)]
    pub fn fsm_bsl1(&mut self) -> FSM_BSL1_W {
        FSM_BSL1_W { w: self }
    }
}
