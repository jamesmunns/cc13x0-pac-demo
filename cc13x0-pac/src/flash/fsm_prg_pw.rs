#[doc = "Reader of register FSM_PRG_PW"]
pub type R = crate::R<u32, super::FSM_PRG_PW>;
#[doc = "Writer for register FSM_PRG_PW"]
pub type W = crate::W<u32, super::FSM_PRG_PW>;
#[doc = "Register FSM_PRG_PW `reset()`'s with value 0"]
impl crate::ResetValue for super::FSM_PRG_PW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PROG_PUL_WIDTH`"]
pub type PROG_PUL_WIDTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PROG_PUL_WIDTH`"]
pub struct PROG_PUL_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PROG_PUL_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PROG_PUL_WIDTH"]
    #[inline(always)]
    pub fn prog_pul_width(&self) -> PROG_PUL_WIDTH_R {
        PROG_PUL_WIDTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PROG_PUL_WIDTH"]
    #[inline(always)]
    pub fn prog_pul_width(&mut self) -> PROG_PUL_WIDTH_W {
        PROG_PUL_WIDTH_W { w: self }
    }
}
