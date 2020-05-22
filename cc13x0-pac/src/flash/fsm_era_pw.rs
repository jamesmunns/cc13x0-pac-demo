#[doc = "Reader of register FSM_ERA_PW"]
pub type R = crate::R<u32, super::FSM_ERA_PW>;
#[doc = "Writer for register FSM_ERA_PW"]
pub type W = crate::W<u32, super::FSM_ERA_PW>;
#[doc = "Register FSM_ERA_PW `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_ERA_PW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSM_ERA_PW`"]
pub type FSM_ERA_PW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FSM_ERA_PW`"]
pub struct FSM_ERA_PW_W<'a> {
    w: &'a mut W,
}
impl<'a> FSM_ERA_PW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FSM_ERA_PW"]
    #[inline(always)]
    pub fn fsm_era_pw(&self) -> FSM_ERA_PW_R {
        FSM_ERA_PW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FSM_ERA_PW"]
    #[inline(always)]
    pub fn fsm_era_pw(&mut self) -> FSM_ERA_PW_W {
        FSM_ERA_PW_W { w: self }
    }
}
