#[doc = "Reader of register FSM_SECTOR2"]
pub type R = crate::R<u32, super::FSM_SECTOR2>;
#[doc = "Writer for register FSM_SECTOR2"]
pub type W = crate::W<u32, super::FSM_SECTOR2>;
#[doc = "Register FSM_SECTOR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_SECTOR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSM_SECTOR2`"]
pub type FSM_SECTOR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FSM_SECTOR2`"]
pub struct FSM_SECTOR2_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_SECTOR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FSM_SECTOR2"]
    #[inline(always)]
    pub fn fsm_sector2(&self) -> FSM_SECTOR2_R {
        FSM_SECTOR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FSM_SECTOR2"]
    #[inline(always)]
    pub fn fsm_sector2(&mut self) -> FSM_SECTOR2_W {
        FSM_SECTOR2_W { w: self }
    }
}
