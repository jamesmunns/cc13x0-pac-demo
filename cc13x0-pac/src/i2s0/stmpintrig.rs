#[doc = "Reader of register STMPINTRIG"]
pub type R = crate::R<u32, super::STMPINTRIG>;
#[doc = "Writer for register STMPINTRIG"]
pub type W = crate::W<u32, super::STMPINTRIG>;
#[doc = "Register STMPINTRIG `reset()`'s with value 0"]
impl crate::ResetValue for super::STMPINTRIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IN_START_WCNT`"]
pub type IN_START_WCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IN_START_WCNT`"]
pub struct IN_START_WCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_START_WCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN_START_WCNT"]
    #[inline(always)]
    pub fn in_start_wcnt(&self) -> IN_START_WCNT_R {
        IN_START_WCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN_START_WCNT"]
    #[inline(always)]
    pub fn in_start_wcnt(&mut self) -> IN_START_WCNT_W {
        IN_START_WCNT_W { w: self }
    }
}
