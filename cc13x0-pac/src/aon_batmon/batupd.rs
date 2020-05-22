#[doc = "Reader of register BATUPD"]
pub type R = crate::R<u32, super::BATUPD>;
#[doc = "Writer for register BATUPD"]
pub type W = crate::W<u32, super::BATUPD>;
#[doc = "Register BATUPD `reset()`'s with value 0"]
impl crate::ResetValue for super::BATUPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STAT`"]
pub type STAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STAT`"]
pub struct STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> STAT_W<'a> {
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
    #[doc = "Bit 0 - STAT"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STAT"]
    #[inline(always)]
    pub fn stat(&mut self) -> STAT_W {
        STAT_W { w: self }
    }
}
