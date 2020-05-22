#[doc = "Reader of register PDCTL0PERIPH"]
pub type R = crate::R<u32, super::PDCTL0PERIPH>;
#[doc = "Writer for register PDCTL0PERIPH"]
pub type W = crate::W<u32, super::PDCTL0PERIPH>;
#[doc = "Register PDCTL0PERIPH `reset()`'s with value 0"]
impl crate::ResetValue for super::PDCTL0PERIPH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ON`"]
pub type ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ON`"]
pub struct ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_W<'a> {
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
    #[doc = "Bit 0 - ON"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ON"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
}
