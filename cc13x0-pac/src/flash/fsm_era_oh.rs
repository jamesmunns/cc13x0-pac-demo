#[doc = "Reader of register FSM_ERA_OH"]
pub type R = crate::R<u32, super::FSM_ERA_OH>;
#[doc = "Writer for register FSM_ERA_OH"]
pub type W = crate::W<u32, super::FSM_ERA_OH>;
#[doc = "Register FSM_ERA_OH `reset()`'s with value 0x01"]
impl crate::ResetValue for super::FSM_ERA_OH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `ERA_OH`"]
pub type ERA_OH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ERA_OH`"]
pub struct ERA_OH_W<'a> {
    w: &'a mut W,
}
impl<'a> ERA_OH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ERA_OH"]
    #[inline(always)]
    pub fn era_oh(&self) -> ERA_OH_R {
        ERA_OH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ERA_OH"]
    #[inline(always)]
    pub fn era_oh(&mut self) -> ERA_OH_W {
        ERA_OH_W { w: self }
    }
}
