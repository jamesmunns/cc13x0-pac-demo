#[doc = "Reader of register FSM_CMD"]
pub type R = crate::R<u32, super::FSM_CMD>;
#[doc = "Writer for register FSM_CMD"]
pub type W = crate::W<u32, super::FSM_CMD>;
#[doc = "Register FSM_CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSMCMD`"]
pub type FSMCMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSMCMD`"]
pub struct FSMCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - FSMCMD"]
    #[inline(always)]
    pub fn fsmcmd(&self) -> FSMCMD_R {
        FSMCMD_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - FSMCMD"]
    #[inline(always)]
    pub fn fsmcmd(&mut self) -> FSMCMD_W {
        FSMCMD_W { w: self }
    }
}
