#[doc = "Reader of register ISRCCTL"]
pub type R = crate::R<u32, super::ISRCCTL>;
#[doc = "Writer for register ISRCCTL"]
pub type W = crate::W<u32, super::ISRCCTL>;
#[doc = "Register ISRCCTL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::ISRCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RESET_N`"]
pub type RESET_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET_N`"]
pub struct RESET_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_N_W<'a> {
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
    #[doc = "Bit 0 - RESET_N"]
    #[inline(always)]
    pub fn reset_n(&self) -> RESET_N_R {
        RESET_N_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RESET_N"]
    #[inline(always)]
    pub fn reset_n(&mut self) -> RESET_N_W {
        RESET_N_W { w: self }
    }
}
