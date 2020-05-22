#[doc = "Reader of register FSM_SECTOR1"]
pub type R = crate::R<u32, super::FSM_SECTOR1>;
#[doc = "Writer for register FSM_SECTOR1"]
pub type W = crate::W<u32, super::FSM_SECTOR1>;
#[doc = "Register FSM_SECTOR1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FSM_SECTOR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `FSM_SECTOR1`"]
pub type FSM_SECTOR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FSM_SECTOR1`"]
pub struct FSM_SECTOR1_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_SECTOR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FSM_SECTOR1"]
    #[inline(always)]
    pub fn fsm_sector1(&self) -> FSM_SECTOR1_R {
        FSM_SECTOR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FSM_SECTOR1"]
    #[inline(always)]
    pub fn fsm_sector1(&mut self) -> FSM_SECTOR1_W {
        FSM_SECTOR1_W { w: self }
    }
}
