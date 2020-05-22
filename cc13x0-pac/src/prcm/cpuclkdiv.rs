#[doc = "Reader of register CPUCLKDIV"]
pub type R = crate::R<u32, super::CPUCLKDIV>;
#[doc = "Writer for register CPUCLKDIV"]
pub type W = crate::W<u32, super::CPUCLKDIV>;
#[doc = "Register CPUCLKDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CPUCLKDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RATIO`"]
pub type RATIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RATIO`"]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
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
    #[doc = "Bit 0 - RATIO"]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RATIO"]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
}
