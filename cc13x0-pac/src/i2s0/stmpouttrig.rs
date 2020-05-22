#[doc = "Reader of register STMPOUTTRIG"]
pub type R = crate::R<u32, super::STMPOUTTRIG>;
#[doc = "Writer for register STMPOUTTRIG"]
pub type W = crate::W<u32, super::STMPOUTTRIG>;
#[doc = "Register STMPOUTTRIG `reset()`'s with value 0"]
impl crate::ResetValue for super::STMPOUTTRIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUT_START_WCNT`"]
pub type OUT_START_WCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OUT_START_WCNT`"]
pub struct OUT_START_WCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_START_WCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - OUT_START_WCNT"]
    #[inline(always)]
    pub fn out_start_wcnt(&self) -> OUT_START_WCNT_R {
        OUT_START_WCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - OUT_START_WCNT"]
    #[inline(always)]
    pub fn out_start_wcnt(&mut self) -> OUT_START_WCNT_W {
        OUT_START_WCNT_W { w: self }
    }
}
