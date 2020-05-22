#[doc = "Reader of register FEFUSESTAT"]
pub type R = crate::R<u32, super::FEFUSESTAT>;
#[doc = "Writer for register FEFUSESTAT"]
pub type W = crate::W<u32, super::FEFUSESTAT>;
#[doc = "Register FEFUSESTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::FEFUSESTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHIFT_DONE`"]
pub type SHIFT_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHIFT_DONE`"]
pub struct SHIFT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SHIFT_DONE"]
    #[inline(always)]
    pub fn shift_done(&self) -> SHIFT_DONE_R {
        SHIFT_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHIFT_DONE"]
    #[inline(always)]
    pub fn shift_done(&mut self) -> SHIFT_DONE_W {
        SHIFT_DONE_W { w: self }
    }
}
