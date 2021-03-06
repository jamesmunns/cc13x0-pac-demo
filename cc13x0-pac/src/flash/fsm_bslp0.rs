#[doc = "Reader of register FSM_BSLP0"]
pub type R = crate::R<u32, super::FSM_BSLP0>;
#[doc = "Writer for register FSM_BSLP0"]
pub type W = crate::W<u32, super::FSM_BSLP0>;
#[doc = "Register FSM_BSLP0 `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_BSLP0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSM_BSLP0`"]
pub type FSM_BSLP0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FSM_BSLP0`"]
pub struct FSM_BSLP0_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_BSLP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FSM_BSLP0"]
    #[inline(always)]
    pub fn fsm_bslp0(&self) -> FSM_BSLP0_R {
        FSM_BSLP0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FSM_BSLP0"]
    #[inline(always)]
    pub fn fsm_bslp0(&mut self) -> FSM_BSLP0_W {
        FSM_BSLP0_W { w: self }
    }
}
