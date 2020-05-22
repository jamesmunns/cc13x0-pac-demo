#[doc = "Reader of register I2SBCLKSEL"]
pub type R = crate::R<u32, super::I2SBCLKSEL>;
#[doc = "Writer for register I2SBCLKSEL"]
pub type W = crate::W<u32, super::I2SBCLKSEL>;
#[doc = "Register I2SBCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::I2SBCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
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
    #[doc = "Bit 0 - SRC"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRC"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
