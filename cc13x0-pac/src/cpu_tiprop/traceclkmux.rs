#[doc = "Reader of register TRACECLKMUX"]
pub type R = crate::R<u32, super::TRACECLKMUX>;
#[doc = "Writer for register TRACECLKMUX"]
pub type W = crate::W<u32, super::TRACECLKMUX>;
#[doc = "Register TRACECLKMUX `reset()`'s with value 0"]
impl crate::ResetValue for super::TRACECLKMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRACECLK_N_SWV`"]
pub type TRACECLK_N_SWV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRACECLK_N_SWV`"]
pub struct TRACECLK_N_SWV_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECLK_N_SWV_W<'a> {
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
    #[doc = "Bit 0 - TRACECLK_N_SWV"]
    #[inline(always)]
    pub fn traceclk_n_swv(&self) -> TRACECLK_N_SWV_R {
        TRACECLK_N_SWV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRACECLK_N_SWV"]
    #[inline(always)]
    pub fn traceclk_n_swv(&mut self) -> TRACECLK_N_SWV_W {
        TRACECLK_N_SWV_W { w: self }
    }
}
